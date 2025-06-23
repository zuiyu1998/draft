pub mod bind_group;
pub mod color_attachment;
pub mod depth_stencil_attachment;

pub mod render_pass_info;
pub mod texel_copy_texture_info;
pub mod texture_view;

pub use bind_group::*;
pub use color_attachment::*;
pub use depth_stencil_attachment::*;
pub use render_pass_info::*;
pub use texel_copy_texture_info::*;
pub use texture_view::*;

use crate::frame_graph::RenderContext;

pub trait ResourceBinding {
    type Resource;

    fn make_resource(&self, render_context: &RenderContext<'_>) -> Self::Resource;
}
