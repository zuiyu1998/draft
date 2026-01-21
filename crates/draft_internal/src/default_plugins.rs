use draft_app::plugin_group;

plugin_group! {
   /// This plugin group will add all the default plugins for a *Draft* application:
   pub struct DefaultPlugins {
       draft_winit:::WinitPlugin,

   }
}
