use draft_app::{
    app::{App, Executor, WinitExecutor},
    scene::{Mesh2d, Node, SceneNode, Surface},
};
use fyrox_core::variable::VariableFlags;

fn main() {
    let mut app = App::empty();

    let mut mesh2d = Mesh2d {
        node: Node::empty(),
        surfaces: Default::default(),
    };

    let surface = Surface {
        mesh: Default::default(),
        material: Default::default(),
        unique_material: Default::default(),
    };

    mesh2d
        .surfaces
        .set_value_with_flags(vec![surface], VariableFlags::MODIFIED);

    app.scene.graph.add_node(SceneNode::new(mesh2d));

    let mut executor = WinitExecutor::default();
    executor.run(app);
}
