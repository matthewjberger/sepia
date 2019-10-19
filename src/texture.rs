use gl::types::GLvoid;
use image::{DynamicImage, DynamicImage::*, GenericImageView};

#[derive(Default)]
pub struct Texture {
    id: u32,
    img: Vec<DynamicImage>,
    target: u32,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let mut texture = Texture::default();
        texture.create();
        texture.target = gl::TEXTURE_2D;
        texture.bind(0);
        texture.set_wrapping_repeat();
        texture.set_filtering_defaults();
        texture
            .img
            .push(Texture::load_image(path, texture.target, true));
        texture
    }

    pub fn cubemap_from_files(paths: &[String; 6]) -> Self {
        let mut texture = Texture::default();
        texture.create();
        texture.target = gl::TEXTURE_CUBE_MAP;
        texture.bind(0);
        texture.set_wrapping_clamp();
        texture.set_filtering_defaults();
        for (offset, path) in paths.iter().enumerate() {
            texture.img.push(Texture::load_image(
                path,
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + offset as u32,
                false,
            ));
        }
        texture
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(self.target, self.id);
        }
    }

    pub fn free(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }

    fn create(&mut self) {
        unsafe {
            gl::GenTextures(1, &mut self.id);
        }
    }

    fn set_wrapping_clamp(&mut self) {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }
    }

    fn set_wrapping_repeat(&mut self) {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);
        }
    }

    fn set_filtering_defaults(&mut self) {
        unsafe {
            // Default filtering options
            gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }

    fn load_image(path: &str, target: u32, flipv: bool) -> DynamicImage {
        let mut img = image::open(path).expect("Texture failed to load!");
        let pixel_format = match img {
            ImageLuma8(_) => gl::RED,
            ImageLumaA8(_) => gl::RG,
            ImageRgb8(_) => gl::RGB,
            ImageRgba8(_) => gl::RGBA,
            ImageBgr8(_) => gl::BGR,
            ImageBgra8(_) => gl::BGRA,
        };
        if flipv {
            img = img.flipv();
        }
        unsafe {
            gl::TexImage2D(
                target,
                0,
                pixel_format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                pixel_format,
                gl::UNSIGNED_BYTE,
                img.raw_pixels().as_ptr() as *const GLvoid,
            );
            gl::GenerateMipmap(target)
        }
        img
    }
}
