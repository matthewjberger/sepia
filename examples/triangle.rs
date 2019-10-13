use gl::types::*;
use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use na::{Matrix4, Perspective3, Vector3};
use nalgebra as na;
use std::{cmp, mem, ptr};
use sepia::camera::*;
use sepia::shader::*;
use sepia::skybox::*;
use sepia::texture::*;

const BACKGROUND_COLOR: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];

#[rustfmt::skip]
const VERTEX_POSITIONS: &[GLfloat; 108] =
    &[
       -0.25,  0.25, -0.25,
       -0.25, -0.25, -0.25,
        0.25, -0.25, -0.25,

        0.25, -0.25, -0.25,
        0.25,  0.25, -0.25,
       -0.25,  0.25, -0.25,

        0.25, -0.25, -0.25,
        0.25, -0.25,  0.25,
        0.25,  0.25, -0.25,

        0.25, -0.25,  0.25,
        0.25,  0.25,  0.25,
        0.25,  0.25, -0.25,

        0.25, -0.25,  0.25,
       -0.25, -0.25,  0.25,
        0.25,  0.25,  0.25,

       -0.25, -0.25,  0.25,
       -0.25,  0.25,  0.25,
        0.25,  0.25,  0.25,

       -0.25, -0.25,  0.25,
       -0.25, -0.25, -0.25,
       -0.25,  0.25,  0.25,

       -0.25, -0.25, -0.25,
       -0.25,  0.25, -0.25,
       -0.25,  0.25,  0.25,

       -0.25, -0.25,  0.25,
        0.25, -0.25,  0.25,
        0.25, -0.25, -0.25,

        0.25, -0.25, -0.25,
       -0.25, -0.25, -0.25,
       -0.25, -0.25,  0.25,

       -0.25,  0.25, -0.25,
        0.25,  0.25, -0.25,
        0.25,  0.25,  0.25,

        0.25,  0.25,  0.25,
       -0.25,  0.25,  0.25,
       -0.25,  0.25, -0.25
    ];

#[rustfmt::skip]
const QUAD_VERTICES: &[GLfloat; 20] =
    &[
        0.5,  0.5, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
       -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
       -0.5,  0.5, 0.0, 0.0, 1.0  // top left
    ];

const QUAD_INDICES: &[GLfloat; 6] = &[0.0, 1.0, 3.0, 1.0, 2.0, 3.0];

fn main() {
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(800, 600, "Sepia", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // let texture = Texture::from_file("assets/textures/blue.jpg");
    // let mut quad_shader_program = ShaderProgram::new();
    // quad_shader_program
    //     .vertex_shader("assets/shaders/texture/texture.vs.glsl")
    //     .fragment_shader("assets/shaders/texture/texture.fs.glsl")
    //     .link();

    // let mut quad_vao = 0;
    // let mut quad_vbo = 0;
    // let mut quad_ebo = 0;

    // let data_length = (5 * mem::size_of::<GLfloat>()) as i32;

    // unsafe {
    //     gl::GenVertexArrays(1, &mut quad_vao);
    //     gl::BindVertexArray(quad_vao);

    //     gl::GenBuffers(1, &mut quad_vbo);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (QUAD_VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
    //         QUAD_VERTICES.as_ptr() as *const gl::types::GLvoid,
    //         gl::STATIC_DRAW,
    //     );

    //     gl::GenBuffers(1, &mut quad_ebo);
    //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_ebo);
    //     gl::BufferData(
    //         gl::ELEMENT_ARRAY_BUFFER,
    //         (QUAD_INDICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
    //         QUAD_INDICES.as_ptr() as *const gl::types::GLvoid,
    //         gl::STATIC_DRAW,
    //     );

    //     gl::EnableVertexAttribArray(0);
    //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, data_length, ptr::null());

    //     gl::EnableVertexAttribArray(1);
    //     gl::VertexAttribPointer(
    //         1,
    //         2,
    //         gl::FLOAT,
    //         gl::FALSE,
    //         data_length,
    //         (3 * mem::size_of::<GLfloat>()) as *const GLvoid,
    //     );
    // }

    // let skybox = Skybox::new(&[
    //     "assets/textures/skyboxes/mountains/right.tga".to_string(),
    //     "assets/textures/skyboxes/mountains/left.tga".to_string(),
    //     "assets/textures/skyboxes/mountains/top.tga".to_string(),
    //     "assets/textures/skyboxes/mountains/bottom.tga".to_string(),
    //     "assets/textures/skyboxes/mountains/back.tga".to_string(),
    //     "assets/textures/skyboxes/mountains/front.tga".to_string(),
    // ]);

    let skybox = Skybox::new(&[
        "assets/textures/blue.jpg".to_string(),
        "assets/textures/blue.jpg".to_string(),
        "assets/textures/blue.jpg".to_string(),
        "assets/textures/blue.jpg".to_string(),
        "assets/textures/blue.jpg".to_string(),
        "assets/textures/blue.jpg".to_string(),
    ]);

    // TODO: Make common asset loader to build path to locate shaders, models, etc
    // let mut shader_program = ShaderProgram::new();
    // shader_program
    //     .vertex_shader("assets/shaders/spinny-cube/spinny-cube.vs.glsl")
    //     .fragment_shader("assets/shaders/spinny-cube/spinny-cube.fs.glsl")
    //     .link();

    // let mut vao = 0;
    // let mut vbo = 0;

    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    //     gl::BindVertexArray(vao);

    //     gl::GenBuffers(1, &mut vbo);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (VERTEX_POSITIONS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
    //         VERTEX_POSITIONS.as_ptr() as *const gl::types::GLvoid,
    //         gl::STATIC_DRAW,
    //     );
    //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
    //     gl::EnableVertexAttribArray(0);

    //     gl::Enable(gl::CULL_FACE);
    //     gl::FrontFace(gl::CW);

    //     gl::Enable(gl::DEPTH_TEST);
    //     gl::DepthFunc(gl::LEQUAL);
    // }

    let mut camera = Camera::new();
    camera.process_mouse_movement(0.0, 0.0);

    let mut current_time = context.get_time();
    let mut last_frame_time = current_time;

    let (window_width, window_height) = window.get_size();
    let mut aspect_ratio = window_width as f32 / cmp::max(0, window_height) as f32;

    while !window.should_close() {
        current_time = context.get_time();
        let delta_time = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;

        let (window_width, window_height) = window.get_size();
        window.set_cursor_pos(
            f64::from(window_width) / 2.0,
            f64::from(window_height) / 2.0,
        );
        window.set_cursor_mode(CursorMode::Disabled);

        // Handle events
        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                    aspect_ratio = width as f32 / cmp::max(0, height) as f32;
                }
                WindowEvent::CursorPos(cursor_x, cursor_y) => {
                    camera.process_mouse_movement(
                        (window_width as f32 / 2.0) - cursor_x as f32,
                        (window_height as f32 / 2.0) - cursor_y as f32,
                    );
                }
                _ => (),
            }
        }

        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            camera.translate(CameraDirection::Forward, delta_time);
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            camera.translate(CameraDirection::Left, delta_time);
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            camera.translate(CameraDirection::Backward, delta_time);
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            camera.translate(CameraDirection::Right, delta_time);
        }

        // Render
        let projection = Perspective3::new(aspect_ratio, 50_f32.to_degrees(), 0.1_f32, 1000_f32);
        let view = camera.view_matrix();

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BACKGROUND_COLOR as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }

        skybox.render(&projection.to_homogeneous(), &view.to_homogeneous());

        // shader_program.activate();
        // let modelview_matrix_location = shader_program.uniform_location("modelview_matrix");
        // let projection_matrix_location = shader_program.uniform_location("projection_matrix");
        // unsafe {
        //     gl::UniformMatrix4fv(
        //         projection_matrix_location,
        //         1,
        //         gl::FALSE,
        //         projection.into_inner().as_ptr(),
        //     );
        // }
        // for cube_id in 0..24 {
        //     let factor: f32 = cube_id as f32 + (current_time as f32 * 0.3);
        //     let modelview = view.to_homogeneous()
        //         * (Matrix4::new_translation(&Vector3::new(0.0, 0.0, -4.0))
        //             * Matrix4::new_rotation(Vector3::new(
        //                 0.0,
        //                 (current_time as f32 * 45_f32).to_radians(),
        //                 (current_time as f32 * 21_f32).to_radians(),
        //             ))
        //             * Matrix4::new_translation(&Vector3::new(
        //                 (2.1 * factor).sin() * 2.0,
        //                 (1.7 * factor).cos() * 2.0,
        //                 (1.3 * factor).sin() * (1.5 * factor).cos() * 2.0,
        //             )));

        //     unsafe {
        //         gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
        //         gl::DrawArrays(gl::TRIANGLES, 0, 36);
        //     }
        // }

        // let modelview = view.to_homogeneous()
        //     * Matrix4::new_translation(&Vector3::new(0.0, 10.0, 0.0))
        //     * Matrix4::new_nonuniform_scaling(&Vector3::new(100.0, 0.2, 100.0));
        // unsafe {
        //     gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
        //     gl::DrawArrays(gl::TRIANGLES, 0, 36);
        // }

        // quad_shader_program.activate();
        // texture.bind(0);
        // let mvp_matrix_location = quad_shader_program.uniform_location("mvpMatrix");
        // let modelview = view.to_homogeneous()
        //     * Matrix4::new_translation(&Vector3::new(0.0, 0.0, 0.0))
        //     * Matrix4::new_nonuniform_scaling(&Vector3::new(100.0, 100.0, 100.0));
        // unsafe {
        //     gl::BindVertexArray(quad_vao);
        //     gl::UniformMatrix4fv(
        //         mvp_matrix_location,
        //         1,
        //         gl::FALSE,
        //         (modelview * projection.to_homogeneous()).as_ptr(),
        //     );
        //     gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        // }

        window.swap_buffers();
    }
}
