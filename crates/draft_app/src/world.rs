use draft_render::{IWorld, RenderContext};

pub struct World(Box<dyn IWorld>);

impl World {
    pub fn empty() -> World {
        World(Box::new(EmptyWorld))
    }

    pub fn new<W: IWorld>(world: W) -> Self {
        Self(Box::new(world))
    }
}

impl IWorld for World {
    fn render(&self, context: &mut RenderContext) {
        self.0.render(context);
    }
}

struct EmptyWorld;

impl IWorld for EmptyWorld {
    fn render(&self, _context: &mut RenderContext) {}
}
