use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};

#[derive(Default)]
pub struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(size) => {
                println!("{:?}", size);
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        
        let mut new_app = Self::default();
        let _ = event_loop.run_app(&mut new_app);

        return new_app;
    }
}