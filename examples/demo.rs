use draft::{DefaultPlugins, app::App};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.run();
}
