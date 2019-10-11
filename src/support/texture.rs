use image::{ImageBuffer, Pixel, Rgb};

type RendererImage = ImageBuffer<Rgb<u8>, Vec<u8>>;
type RendererColor = Rgb<u8>;

struct Texture {
    id: u32,
}
