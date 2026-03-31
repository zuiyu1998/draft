use draft_app::{
    app::{App, Executor, WinitExecutor},
    scene::{Mesh2d, Node, SceneNode},
};

fn main() {
    let mut app = App::empty();

    let mesh2d = Mesh2d {
        node: Node::empty(),
        surfaces: Default::default(),
    };

    app.scene.graph.add_node(SceneNode::new(mesh2d));

    let mut executor = WinitExecutor::default();
    executor.run(app);
}
