pub mod bind_group_entry_handle;
pub mod bind_group_entry_info;
pub mod bind_group_handle_builder;
pub mod bind_group_info;

pub use bind_group_entry_handle::*;
pub use bind_group_entry_info::*;
pub use bind_group_handle_builder::*;
pub use bind_group_info::*;

use crate::frame_graph::{FrameGraph, PassNodeBuilder};

pub trait BindGroupResourceBindingHelper {
    fn make_bind_group_resource_binding(
        &self,
        pass_node_builder: &mut PassNodeBuilder,
    ) -> BindGroupResourceBinding;
}

pub trait BindGroupTextureViewHandleHelper {
    fn make_bind_group_texture_view_handle(
        &self,
        frame_graph: &mut FrameGraph,
    ) -> BindGroupTextureViewHandle;
}

pub trait BindGroupBufferHandleHelper {
    fn make_bind_group_buffer_handle(&self, frame_graph: &mut FrameGraph) -> BindGroupBufferHandle;
}
