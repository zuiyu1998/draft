use draft_app::plugin_group;

plugin_group! {
   /// This plugin group will add all the default plugins for a *Draft* application:
   pub struct DefaultPlugins {
       draft_scene:::ScenePlugin,
       draft_winit:::WinitPlugin,
       draft_render_2d:::Material2dPlugin,
   }
}
