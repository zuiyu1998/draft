use draft_app::{App, AppInitializeParams, GraphicsContextParams, Plugin};
use draft_graphics::RenderServer;
use draft_window::{
    HasDisplayHandle, ISystemWindow, PhysicalSize, RawDisplayHandle, RawWindowHandle, SystemWindow,
};
use fyrox_core::futures::executor::block_on;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop,
    raw_window_handle::HasWindowHandle, window::Window as WinitWindow,
};

pub struct WindowWrapper(WinitWindow);

impl ISystemWindow for WindowWrapper {
    fn get_physical_size(&self) -> PhysicalSize {
        let size = self.0.inner_size();
        PhysicalSize {
            width: size.width,
            height: size.height,
        }
    }

    fn get_raw_display_handle(&self) -> RawDisplayHandle {
        self.0.display_handle().unwrap().as_raw()
    }

    fn get_raw_window_handle(&self) -> RawWindowHandle {
        self.0.window_handle().unwrap().as_raw()
    }
}

#[derive(Default)]
pub struct WinitPlugin;

pub struct WinitAppRunnerState {
    app: App,
}

impl WinitAppRunnerState {
    pub fn new(app: App) -> Self {
        Self { app }
    }
}

fn create_render_server(
    _graphics_context_params: &GraphicsContextParams,
    window: SystemWindow,
) -> RenderServer {
    block_on(RenderServer::initialize(window))
}

impl ApplicationHandler for WinitAppRunnerState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let raw_window = event_loop
            .create_window(Default::default())
            .expect("create window faild.");

        let window = SystemWindow::new(WindowWrapper(raw_window));

        self.app.initialize(AppInitializeParams {
            window,
            render_server_constructor: Box::new(create_render_server),
        });
        self.app.finished();
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.destroy();
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.update();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            _ => {}
        }
    }
}

fn winit_runner(app: App, event_loop: EventLoop<()>) {
    let mut runner_state = WinitAppRunnerState::new(app);

    event_loop
        .run_app(&mut runner_state)
        .expect("run app faild.");
}

impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) {
        let event_loop = EventLoop::builder().build().expect("event loop faild.");

        app.set_runner(|app| winit_runner(app, event_loop));
    }
}
