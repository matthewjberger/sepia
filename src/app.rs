pub use gl::types::*;
pub use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use std::{cmp, sync::mpsc::Receiver};

const BACKGROUND_COLOR: &[GLfloat; 4] = &[0.25, 0.25, 0.25, 1.0];

pub struct StateData<'a> {
    pub window: &'a mut glfw::Window,
    pub delta_time: f32,
    pub current_time: f32,
    pub aspect_ratio: f32,
}

pub trait State {
    // Called once at the beginning of a run
    fn initialize(&mut self) {}

    // Called when events are handled
    fn handle_events(&mut self, _: &mut StateData, _: &glfw::WindowEvent) {}

    // Called each frame
    fn update(&mut self, _: &mut StateData) {}

    // Called each frame after updates
    fn render(&mut self, _: &mut StateData) {}
}

pub struct EmptyState;
impl State for EmptyState {}

pub struct App<'a> {
    context: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    state_machine: Vec<&'a mut dyn State>,
}

impl<'a> App<'a> {
    pub fn new(state_machine: Vec<&'a mut dyn State>) -> Self {
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
            state_machine,
        }
    }

    pub fn run(&mut self) {
        if self.state_machine.is_empty() {
            return;
        }

        let state = self
            .state_machine
            .first_mut()
            .expect("Couldn't get first state!");
        state.initialize();

        let mut current_time = self.context.get_time();
        let mut last_frame_time = current_time;

        while !self.window.should_close() {
            let (window_width, window_height) = self.window.get_size();
            let aspect_ratio = window_width as f32 / cmp::max(0, window_height) as f32;

            current_time = self.context.get_time();
            let delta_time = (current_time - last_frame_time) as f32;
            last_frame_time = current_time;

            let mut state_data = StateData {
                window: &mut self.window,
                delta_time,
                current_time: current_time as f32,
                aspect_ratio,
            };

            self.context.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                if let WindowEvent::FramebufferSize(width, height) = event {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                state.handle_events(&mut state_data, &event);
            }

            state.update(&mut state_data);

            unsafe {
                gl::ClearBufferfv(gl::COLOR, 0, BACKGROUND_COLOR as *const f32);
            }

            state.render(&mut state_data);

            self.window.swap_buffers();
        }
    }
}
