use draft::{app::App, prelude::*};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.run();
}
