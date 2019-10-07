use gl::types::*;
use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use na::{Matrix4, Perspective3, Vector3};
use nalgebra as na;
use std::{cmp, mem, ptr};
use support::camera::*;
use support::shader::*;

const BACKGROUND_COLOR: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];
const ONES: &[GLfloat; 1] = &[1.0];

#[rustfmt::skip]
static VERTEX_POSITIONS: &[GLfloat; 108] =
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

    // Initialize
    let mut shader_program = ShaderProgram::new();
    shader_program
        .vertex_shader("assets/shaders/spinny-cube/spinny-cube.vs.glsl")
        .fragment_shader("assets/shaders/spinny-cube/spinny-cube.fs.glsl")
        .link();

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_POSITIONS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            VERTEX_POSITIONS.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CW);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

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
        window.set_cursor_pos(window_width as f64 / 2.0, window_height as f64 / 2.0);
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
                },
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

        // Update

        // Render
        shader_program.activate();

        let modelview_matrix_location = shader_program.uniform_location("modelview_matrix");
        let projection_matrix_location = shader_program.uniform_location("projection_matrix");
        let projection = Perspective3::new(aspect_ratio, 50_f32.to_degrees(), 0.1_f32, 1000_f32);

        let view = camera.view_matrix();

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BACKGROUND_COLOR as *const f32);
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection.into_inner().as_ptr(),
            );
        }

        for cube_id in 0..24 {
            let factor: f32 = cube_id as f32 + (current_time as f32 * 0.3);
            let model = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -4.0))
                * Matrix4::new_rotation(Vector3::new(
                    0.0,
                    (current_time as f32 * 45_f32).to_radians(),
                    (current_time as f32 * 21_f32).to_radians(),
                ))
                * Matrix4::new_translation(&Vector3::new(
                    (2.1 * factor).sin() * 2.0,
                    (1.7 * factor).cos() * 2.0,
                    (1.3 * factor).sin() * (1.5 * factor).cos() * 2.0,
                ));

            let modelview = view.to_homogeneous() * model;

            unsafe {
                gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        window.swap_buffers();
    }
}
