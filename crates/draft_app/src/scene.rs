use draft_render::{RenderContext, World};

pub struct SceneTree {}

impl SceneTree {
    pub fn empty() -> SceneTree {
        SceneTree {  }
    }
}

impl World for SceneTree {
    fn render(&self, _context: &mut RenderContext) {}
}
