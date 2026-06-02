use std::num::NonZeroU64;

use draft_graphics::{BindGroup, BindGroupLayout};

use crate::frame_graph::{ResourceRead, ResourceRef, TransientBuffer};

#[derive(Clone)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: BindingResource,
}

pub trait IntoBindingResource {
    fn into_binding_resource(self) -> BindingResource;
}

#[derive(Clone)]
pub enum BindingResource {
    Buffer(BindingBuffer),
}

impl IntoBindingResource for BindingBuffer {
    fn into_binding_resource(self) -> BindingResource {
        BindingResource::Buffer(self)
    }
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

impl TransientBindGroupDescriptor {
    pub fn new(layout: BindGroupLayout) -> Self {
        Self {
            layout,
            entries: vec![],
        }
    }

    pub fn add_entry(&mut self, binding: u32, value: impl IntoBindingResource) -> &mut Self {
        let entry = BindGroupEntry {
            binding,
            resource: value.into_binding_resource(),
        };
        self.entries.push(entry);
        self
    }
}
#[derive(Clone)]
pub enum TransientBindGroup {
    BindGroup(BindGroup),
    Desc(TransientBindGroupDescriptor),
}
