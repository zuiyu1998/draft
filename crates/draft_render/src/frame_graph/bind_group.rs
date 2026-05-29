use std::num::NonZeroU64;

use draft_graphics::{BindGroup, BindGroupLayout};

use crate::frame_graph::{ResourceRead, ResourceRef, TransientBuffer};

#[derive(Clone)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: BindingResource,
}

#[derive(Clone)]
pub enum BindingResource {
    Buffer(BindingBuffer),
}

#[derive(Clone)]
pub struct BindingBuffer {
    pub buffer_ref: ResourceRef<TransientBuffer, ResourceRead>,
    pub offset: u64,
    pub size: Option<NonZeroU64>,
}

#[derive(Clone)]
pub struct TransientBindGroupDescriptor {
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupEntry>,
}

#[derive(Clone)]
pub enum TransientBindGroup {
    BindGroup(BindGroup),
    Desc(TransientBindGroupDescriptor),
}
