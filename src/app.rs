pub use gl::types::*;
pub use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use std::{cmp, sync::mpsc::Receiver};

const BACKGROUND_COLOR: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

pub struct App {
    context: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl App {
    pub fn new() -> Self {
        let context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = context
            .create_window(800, 600, "Sepia", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        App {
            window,
            context,
            events,
        }
    }

    fn aspect_ratio(&self) -> f32 {
        let (window_width, window_height) = self.window.get_size();
        window_width as f32 / cmp::max(0, window_height) as f32
    }

    pub fn run(&mut self) {
        let mut current_time = self.context.get_time();
        let mut last_frame_time = current_time;

        while !self.window.should_close() {
            current_time = self.context.get_time();
            let _delta_time = (current_time - last_frame_time) as f32;
            last_frame_time = current_time;

            self.context.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    }
                    glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                        gl::Viewport(0, 0, width, height);
                    },
                    _ => {}
                }
            }

            unsafe {
                gl::ClearBufferfv(gl::COLOR, 0, BACKGROUND_COLOR as *const f32);
            }

            self.window.swap_buffers();
        }
    }

    fn center_cursor(&mut self) {
        let (window_width, window_height) = self.window.get_size();
        self.window.set_cursor_pos(
            f64::from(window_width) / 2.0,
            f64::from(window_height) / 2.0,
        );
        self.window.set_cursor_mode(CursorMode::Disabled);
    }
}