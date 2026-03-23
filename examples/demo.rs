use draft_app::app::App;
use draft_winit::WinitPlugin;

fn main() {
    let mut app = App::empty();

    app.add_plugin(WinitPlugin::default());

    app.run();
}