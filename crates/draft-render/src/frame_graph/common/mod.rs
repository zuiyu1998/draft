mod bind_group;
mod color_attachment;
mod depth_stencil_attachment;

mod render_pass;
mod texel_copy_texture_info;
mod texture_view;

pub use bind_group::*;
pub use color_attachment::*;
pub use depth_stencil_attachment::*;
pub use render_pass::*;
pub use texel_copy_texture_info::*;
pub use texture_view::*;

use crate::frame_graph::FrameGraphContext;

pub trait TransientResourceBinding {
    type Resource;

    fn make_resource(&self, frame_graph_context: &FrameGraphContext<'_>) -> Self::Resource;
}
