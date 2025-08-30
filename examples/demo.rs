use std::{collections::HashMap, sync::Arc};

use draft_render::{
    ErasedMaterial, FragmentState, Geometry, GeometryResource, Material, MaterialEffectInfo,
    MaterialInfo, MaterialResource, MaterialTextureBinding, MeshRenderPhase, PipelineInfo,
    RenderPhasesContainer, RenderPipelineInfo, RenderServer, RenderWorld,
    ResourceBindingDefinition, ResourceBindingName, Shader, ShaderResource, Texture,
    TextureResource, Vertex, VertexAttributeDescriptor,
    frame_graph::{ColorAttachment, FrameGraph, TextureView},
    gfx_base::{
        BlendComponent, BlendState, ColorTargetState, ColorWrites, GpuTextureView,
        RawTextureFormat, SamplerBindingType, ShaderStages, TextureFormat, TextureSampleType,
        VertexFormat,
        binding_types::{sampler, texture_2d},
        initialize_resources,
    },
    wgpu::{
        self, Color, CompositeAlphaMode, Instance, InstanceDescriptor, LoadOp, Operations,
        PresentMode, StoreOp, Surface, SurfaceConfiguration, SurfaceTexture, TextureUsages,
        TextureViewDescriptor,
    },
};

use draft::renderer::{
    Batch, Observer, ObserversCollection, Pipeline, PipelineContext, PipelineNode, WorldRenderer,
};

use fyrox_core::{ImmutableString, futures, task::TaskPool, uuid};
use fyrox_resource::{
    embedded_data_source,
    io::FsResourceIo,
    manager::{BuiltInResource, ResourceManager},
    untyped::ResourceKind,
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BUILT_IN_SHADER: BuiltInResource<Shader> = BuiltInResource::new(
        "__BUILT_IN_SHADER__",
        embedded_data_source!("./shader.wgsl"),
        |data| {
            ShaderResource::new_ok(
                uuid!("77260e8e-f6fa-429c-8009-13dda2673925"),
                ResourceKind::External,
                Shader::from_memory(data.to_vec()).unwrap(),
            )
        }
    );
}

pub struct TestMaterial;

impl TestMaterial {
    pub fn get_test_effct_name() -> ImmutableString {
        "test".into()
    }
}

impl ErasedMaterial for TestMaterial {
    fn material_info() -> MaterialInfo {
        let mut desc = RenderPipelineInfo::default();

        desc.vertex.shader = BUILT_IN_SHADER.resource().clone();
        desc.vertex.entry_point = Some("vs_main".into());
        desc.fragment = Some(FragmentState {
            shader: BUILT_IN_SHADER.resource().clone(),
            entry_point: Some("fs_main".into()),
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::Bgra8UnormSrgb,
                blend: Some(BlendState {
                    color: BlendComponent::REPLACE,
                    alpha: BlendComponent::REPLACE,
                }),
                write_mask: ColorWrites::ALL,
            })],
        });

        let pipeline_info = PipelineInfo::RenderPipelineInfo(Box::new(desc));

        let test_effct_name = TestMaterial::get_test_effct_name();

        let mut test_effect_info = MaterialEffectInfo {
            effect_name: test_effct_name,
            ..Default::default()
        };

        test_effect_info
            .resource_binding_definitions
            .push(ResourceBindingDefinition {
                name: ResourceBindingName::Local("t_diffuse".into()),
                entry: texture_2d(TextureSampleType::Float { filterable: true })
                    .build(0, ShaderStages::default()),
            });

        test_effect_info
            .resource_binding_definitions
            .push(ResourceBindingDefinition {
                name: ResourceBindingName::Local("t_diffuse".into()),
                entry: sampler(SamplerBindingType::Filtering).build(1, ShaderStages::default()),
            });

        MaterialInfo {
            pipeline_info,
            effect_infos: vec![test_effect_info],
        }
    }
}

pub struct TestNode;

impl PipelineNode for TestNode {
    fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        context: &PipelineContext,
        phases_container: &RenderPhasesContainer,
    ) {
        let mut pass_builder = frame_graph.create_pass_builder("test_node");
        let mut render_pass_builder = pass_builder.create_render_pass_builder("test_pass");

        render_pass_builder.add_out_color_attachment(ColorAttachment {
            view: context.texture_view.clone(),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::WHITE),
                store: StoreOp::Store,
            },
        });

        if let Some(phases) = phases_container.get_phases::<MeshRenderPhase>() {
            phases.render(&mut render_pass_builder, world);
        }
    }
}

pub struct Windows {
    primary: WindowId,
    windows: HashMap<WindowId, WindowData>,
}

impl Windows {
    pub fn from_server(render_server: &RenderServer, window: Arc<Window>) -> Self {
        Windows::new(WindowData::from_server(render_server, window))
    }

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

    pub swap_chain_texture_view: Option<GpuTextureView>,
    pub swap_chain_texture: Option<SurfaceTexture>,
    pub swap_chain_texture_format: RawTextureFormat,
}

impl WindowData {
    pub fn new(
        window: Arc<Window>,
        surface: Surface<'static>,
        swap_chain_texture_format: RawTextureFormat,
    ) -> Self {
        Self {
            window,
            surface,
            swap_chain_texture: None,
            swap_chain_texture_format,
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

        self.swap_chain_texture_view = Some(GpuTextureView::new(texture_view));

        self.swap_chain_texture = Some(frame);
    }

    pub fn present(&mut self) {
        self.swap_chain_texture_view = None;

        if let Some(frame) = self.swap_chain_texture.take() {
            frame.present();
        }
    }

    pub fn from_server(render_server: &RenderServer, window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let surface = render_server
            .instance
            .0
            .create_surface(window.clone())
            .unwrap();

        let cap = surface.get_capabilities(&render_server.adapter.0);
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

        surface.configure(render_server.device.wgpu_device(), &surface_config);

        WindowData::new(window, surface, surface_format)
    }
}

fn new_server(window: Arc<Window>) -> RenderServer {
    futures::executor::block_on(async_server(window))
}

async fn async_server(window: Arc<Window>) -> RenderServer {
    let instance = Instance::new(&InstanceDescriptor::default());

    let size = window.inner_size();

    let surface = instance.create_surface(window.clone()).unwrap();
    let request_adapter_options = wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    };

    let (device, queue, adapter, _, instance) =
        initialize_resources(instance, &request_adapter_options).await;

    let cap = surface.get_capabilities(&adapter.0);
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

    surface.configure(device.wgpu_device(), &surface_config);

    RenderServer {
        device,
        queue,
        instance,
        adapter,
    }
}

struct State {
    windows: Windows,
    size: winit::dpi::PhysicalSize<u32>,
    _resource_manager: ResourceManager,
    renderer: WorldRenderer,
    batch: Batch,
}

impl State {
    fn new(window: Arc<Window>) -> State {
        let task_pool = Arc::new(TaskPool::new());
        let resource_manager = ResourceManager::new(Arc::new(FsResourceIo), task_pool);

        let size = window.inner_size();

        let render_server = new_server(window.clone());

        let windows = Windows::from_server(&render_server, window);

        let mut renderer = WorldRenderer::new(render_server, &resource_manager);

        renderer.world.register_material::<TestMaterial>();

        let mut pipeline = Pipeline::empty();
        pipeline.push_node(TestNode);

        renderer.pipeline_container.insert("test".into(), pipeline);

        resource_manager.update_or_load_registry();

        let image = resource_manager.request::<Texture>("data/happy-tree.png");
        let batch = new_batch(&image);

        State {
            windows,
            size,
            _resource_manager: resource_manager,
            renderer,
            batch,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
    }

    fn render(&mut self) {
        self.renderer.update(0.0);

        self.windows.set_swapchain_texture();

        let mut observers: ObserversCollection = ObserversCollection::default();
        let observer = Observer {
            pipeline_name: "test".into(),
            ..Default::default()
        };

        observers.cameras.push(observer);

        if let Some(texture_view) = self
            .windows
            .get_primary_window()
            .swap_chain_texture_view
            .clone()
        {
            let pipeline_context = PipelineContext {
                texture_view: TextureView::new(texture_view),
                batch: &self.batch,
            };

            self.renderer.render(&pipeline_context, &observers);
        }

        self.windows.present();
    }
}

fn new_batch(image: &TextureResource) -> Batch {
    let mut vertex = Vertex::default();
    let mut modifier = vertex.modify();
    modifier.insert_attribute(
        VertexAttributeDescriptor::ATTRIBUTE_POSITION,
        vec![
            [-0.0868241, 0.49240386, 0.0],
            [-0.49513406, 0.06958647, 0.0],
            [-0.21918549, -0.44939706, 0.0],
            [0.35966998, -0.3473291, 0.0],
            [0.44147372, 0.2347359, 0.0],
        ],
    );

    modifier.insert_attribute(
        VertexAttributeDescriptor::new(1, VertexFormat::Float32x2),
        vec![
            [0.4131759, 0.00759614],
            [0.0048659444, 0.43041354],
            [0.28081453, 0.949397],
            [0.85967, 0.84732914],
            [0.9414737, 0.2652641],
        ],
    );

    modifier.set_need_update(true);

    let indexes: Vec<u16> = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];
    let geometry = Geometry::new(vertex, indexes.into());

    let mut material = Material::from_material::<TestMaterial>();

    let test_effct_name = TestMaterial::get_test_effct_name();

    if let Some(effect) = material.effect_mut(&test_effct_name) {
        effect.resource_bindings.insert(
            ResourceBindingName::Local("t_diffuse".into()),
            MaterialTextureBinding {
                value: Some(image.clone()),
            }
            .into(),
        );
    }

    Batch::new(
        GeometryResource::new_embedded(geometry),
        MaterialResource::new_embedded(material),
    )
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

        let state = State::new(window.clone());
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
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
