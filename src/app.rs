pub use gl::types::*;
pub use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use std::sync::mpsc::Receiver;

const BACKGROUND_COLOR: &[GLfloat; 4] = &[0.0, 0.25, 0.0, 1.0];

// TODO: Generalize refs here into StateData struct and pass that around
pub trait State {
    fn initialize(&mut self);

    // Called when events are handled
    fn handle_events(
        &mut self,
        event: &glfw::WindowEvent,
        window: &mut glfw::Window,
        delta_time: f32,
    );

    // Called each frame
    fn update(&mut self, window: &mut glfw::Window, delta_time: f32);

    // Called each frame after updates
    fn render(&mut self, current_time: f32, window: &mut glfw::Window);
}

pub struct EmptyState;
impl State for EmptyState {
    fn initialize(&mut self) {}
    fn handle_events(
        &mut self,
        event: &glfw::WindowEvent,
        window: &mut glfw::Window,
        delta_time: f32,
    ) {
    }
    fn update(&mut self, window: &mut glfw::Window, delta_time: f32) {}
    fn render(&mut self, current_time: f32, window: &mut glfw::Window) {}
}

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

        let state = self.state_machine.first_mut().unwrap();
        state.initialize();

        let mut current_time = self.context.get_time();
        let mut last_frame_time = current_time;

        while !self.window.should_close() {
            current_time = self.context.get_time();
            let delta_time = (current_time - last_frame_time) as f32;
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
                state.handle_events(&event, &mut self.window, delta_time);
            }

            state.update(&mut self.window, delta_time);

            unsafe {
                gl::ClearBufferfv(gl::COLOR, 0, BACKGROUND_COLOR as *const f32);
            }

            state.render(current_time as f32, &mut self.window);

            self.window.swap_buffers();
        }
    }
}
