use draft::{
    DefaultPlugins,
    app::App,
    render::{IWorld, RenderContext},
};

pub struct SceneTree {}

impl SceneTree {
    pub fn new() -> SceneTree {
        SceneTree {}
    }
}

impl IWorld for SceneTree {
    fn render(&self, _context: &mut RenderContext) {}
}

fn main() {
    let mut app = App::new();

    app.set_world(SceneTree {});
    app.add_plugin(DefaultPlugins);

    app.run();
}
