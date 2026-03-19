mod point_cloud;

mod space_colony;

mod state;
pub use state::AppState;

use std::{fmt::Display, ops::AddAssign, path::PathBuf, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct FrameIndex(pub u64);
impl Display for FrameIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl From<u64> for FrameIndex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl AddAssign<i64> for FrameIndex {
    fn add_assign(&mut self, rhs: i64) {
        self.0 = self.0.wrapping_add_signed(rhs)
    }
}

pub struct App {
    asset_dir: PathBuf,
    state: Option<AppState>,
}
impl App {
    pub fn new(asset_dir: PathBuf) -> Self {
        Self {
            asset_dir,
            state: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let display = Box::new(event_loop.owned_display_handle());
        let state = pollster::block_on(AppState::new(&self, window.clone(), display));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();

        state.handle_event(&event);

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                state.current_frame += 1;

                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            _ => (),
        }
    }
}
