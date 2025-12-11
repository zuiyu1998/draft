use draft_render::{RenderDataBundle, RenderServer};

pub struct WorldRenderer {
    render_server: RenderServer,
}

impl WorldRenderer {
    pub fn update(&mut self) {}

    fn prepare_frame<W: World>(&mut self, world: &W) -> Frame {
        let mut frame = Frame::empty();

        let mut context = RenderContext {
            render_data_bundle: &mut frame.render_data_bundle,
        };

        world.prepare(&mut context);

        frame
    }

    fn render_frame(&mut self, _frame: Frame) {}

    pub fn render<W: World>(&mut self, world: &W) {
        let frame = self.prepare_frame(world);
        self.render_frame(frame);
    }
}

pub struct Frame {
    render_data_bundle: RenderDataBundle,
}

impl Frame {
    pub fn empty() -> Self {
        Self {
            render_data_bundle: RenderDataBundle::empty(),
        }
    }
}

pub struct RenderContext<'a> {
    render_data_bundle: &'a mut RenderDataBundle,
}

pub trait World {
    fn prepare(&self, context: &mut RenderContext);
}
