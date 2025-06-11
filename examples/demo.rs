use std::{collections::HashMap, sync::Arc};

use draft_render::wgpu::{
    CompositeAlphaMode, Device, DeviceDescriptor, Instance, InstanceDescriptor, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat,
    TextureUsages, TextureView as RawTextureView, TextureViewDescriptor,
};
use fyrox_core::{futures, task::TaskPool};
use fyrox_resource::{io::FsResourceIo, manager::ResourceManager};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct Windows {
    primary: WindowId,
    windows: HashMap<WindowId, WindowData>,
}

impl Windows {
    pub fn get_window(&self, window_id: Option<WindowId>) -> Option<&WindowData> {
        if let Some(window_id) = window_id {
            self.windows.get(&window_id)
        } else {
            self.windows.get(&self.primary)
        }
    }

    pub fn get_primary_window(&self) -> &WindowData {
        self.windows.get(&self.primary).unwrap()
    }

    pub fn new(data: WindowData) -> Self {
        let primary = data.window.id();
        let mut windows = HashMap::default();
        windows.insert(primary, data);

        Windows { primary, windows }
    }

    pub fn add_window_data(&mut self, data: WindowData) {
        self.windows.insert(data.window.id(), data);
    }

    pub fn request_redraw(&self) {
        for window_data in self.windows.values() {
            window_data.window.request_redraw();
        }
    }

    pub fn set_swapchain_texture(&mut self) {
        for window_data in self.windows.values_mut() {
            window_data.set_swapchain_texture();
        }
    }

    pub fn present(&mut self) {
        for window_data in self.windows.values_mut() {
            window_data.present();
        }
    }
}

pub struct WindowData {
    window: Arc<Window>,
    surface: Surface<'static>,

    pub swap_chain_texture_view: Option<RawTextureView>,
    pub swap_chain_texture: Option<SurfaceTexture>,
    pub swap_chain_texture_format: Option<TextureFormat>,
}

impl WindowData {
    pub fn new(window: Arc<Window>, surface: Surface<'static>) -> Self {
        Self {
            window,
            surface,
            swap_chain_texture: None,
            swap_chain_texture_format: None,
            swap_chain_texture_view: None,
        }
    }

    pub fn set_swapchain_texture(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();

        let texture_view_descriptor = TextureViewDescriptor {
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        };
        let texture_view = frame.texture.create_view(&texture_view_descriptor);

        self.swap_chain_texture_view = Some(texture_view);

        self.swap_chain_texture = Some(frame);
    }

    pub fn present(&mut self) {
        self.swap_chain_texture_view = None;
        self.swap_chain_texture_format = None;

        if let Some(frame) = self.swap_chain_texture.take() {
            frame.present();
        }
    }
}

struct State {
    windows: Windows,
    _device: Device,
    _queue: Queue,
    size: winit::dpi::PhysicalSize<u32>,
    _resource_manager: ResourceManager,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![surface_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoVsync,
        };

        surface.configure(&device, &surface_config);

        let windows = Windows::new(WindowData::new(window, surface));

        let task_pool = Arc::new(TaskPool::new());
        let resource_manager = ResourceManager::new(Arc::new(FsResourceIo), task_pool);

        let (shader_event_sender, _shader_event_receiver) = std::sync::mpsc::channel();

        resource_manager
            .state()
            .event_broadcaster
            .add(shader_event_sender);

        State {
            windows,
            size,
            _resource_manager: resource_manager,
            _device: device,
            _queue: queue,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
    }

    fn render(&mut self) {
        self.windows.set_swapchain_texture();

        self.windows.present();
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = futures::executor::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                // Emits a new redraw requested event.
                state.windows.request_redraw();
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            _ => (),
        }
    }
}

fn main() {
    tracing_subscriber::fmt().init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
