use core::fmt;
use std::{
    collections::{HashSet, hash_map::Entry},
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut, Range},
};

use draft_graphics::{
    gfx_base::{BufferDescriptor, CommandEncoderDescriptor, RenderDevice, RenderQueue},
    wgpu::{BufferSize, BufferUsages, COPY_BUFFER_ALIGNMENT},
};
use draft_mesh::{Indices, Mesh, MeshResource, MeshVertexBufferLayoutRef, MeshVertexBufferLayouts};
use fxhash::FxHashMap;
use nonmax::NonMaxU32;
use offset_allocator::{Allocation, Allocator};
use tracing::error;

type MeshKey = u64;

use crate::{BufferAllocator, ImportBufferMeta};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ElementClass {
    /// Data for a vertex.
    Vertex,
    /// A vertex index.
    Index,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ElementLayout {
    class: ElementClass,
    size: u64,
    elements_per_slot: u32,
}

impl ElementLayout {
    fn new(class: ElementClass, size: u64) -> ElementLayout {
        const {
            assert!(4 == COPY_BUFFER_ALIGNMENT);
        }
        // this is equivalent to `4 / gcd(4,size)` but lets us not implement gcd.
        // ping @atlv if above assert ever fails (likely never)
        let elements_per_slot = [1, 4, 2, 4][size as usize & 3];
        ElementLayout {
            class,
            size,
            // Make sure that slot boundaries begin and end on
            // `COPY_BUFFER_ALIGNMENT`-byte (4-byte) boundaries.
            elements_per_slot,
        }
    }

    fn slot_size(&self) -> u64 {
        self.size * self.elements_per_slot as u64
    }

    fn vertex(layout: &MeshVertexBufferLayoutRef) -> ElementLayout {
        ElementLayout::new(ElementClass::Vertex, layout.0.layout().array_stride)
    }

    fn index(mesh: &Mesh) -> Option<ElementLayout> {
        let size = match mesh.indices()? {
            Indices::U16(_) => 2,
            Indices::U32(_) => 4,
        };
        Some(ElementLayout::new(ElementClass::Index, size))
    }
}

pub struct MeshAllocatorSettings {
    pub min_slab_size: u64,
    pub max_slab_size: u64,
    pub large_threshold: u64,
    pub growth_factor: f64,
}

impl Default for MeshAllocatorSettings {
    fn default() -> Self {
        Self {
            // 1 MiB
            min_slab_size: 1024 * 1024,
            // 512 MiB
            max_slab_size: 1024 * 1024 * 512,
            // 256 MiB
            large_threshold: 1024 * 1024 * 256,
            // 1.5× growth
            growth_factor: 1.5,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct SlabId(pub NonMaxU32);

impl Display for SlabId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct SlabAllocation {
    pub allocation: Allocation,
    pub slot_count: u32,
}

struct MeshAllocation {
    slab_id: SlabId,
    slab_allocation: SlabAllocation,
}

enum SlabGrowthResult {
    /// The mesh data already fits in the slab; the slab doesn't need to grow.
    NoGrowthNeeded,
    /// The slab needed to grow.
    ///
    /// The [`SlabToReallocate`] contains the old capacity of the slab.
    NeededGrowth(SlabToReallocate),
    /// The slab wanted to grow but couldn't because it hit its maximum size.
    CantGrow,
}

pub struct GeneralSlab {
    allocator: Allocator,
    buffer: Option<ImportBufferMeta>,
    resident_allocations: FxHashMap<MeshKey, SlabAllocation>,
    pending_allocations: FxHashMap<MeshKey, SlabAllocation>,
    element_layout: ElementLayout,
    current_slot_capacity: u32,
}

impl GeneralSlab {
    fn is_empty(&self) -> bool {
        self.resident_allocations.is_empty() && self.pending_allocations.is_empty()
    }

    fn new(
        new_slab_id: SlabId,
        mesh_allocation: &mut Option<MeshAllocation>,
        settings: &MeshAllocatorSettings,
        layout: ElementLayout,
        data_slot_count: u32,
    ) -> Self {
        let initial_slab_slot_capacity = (settings.min_slab_size.div_ceil(layout.slot_size())
            as u32)
            .max(offset_allocator::ext::min_allocator_size(data_slot_count));
        let max_slab_slot_capacity = (settings.max_slab_size.div_ceil(layout.slot_size()) as u32)
            .max(offset_allocator::ext::min_allocator_size(data_slot_count));

        let mut new_slab = GeneralSlab {
            allocator: Allocator::new(max_slab_slot_capacity),
            buffer: None,
            resident_allocations: FxHashMap::default(),
            pending_allocations: FxHashMap::default(),
            element_layout: layout,
            current_slot_capacity: initial_slab_slot_capacity,
        };

        // This should never fail.
        if let Some(allocation) = new_slab.allocator.allocate(data_slot_count) {
            *mesh_allocation = Some(MeshAllocation {
                slab_id: new_slab_id,
                slab_allocation: SlabAllocation {
                    slot_count: data_slot_count,
                    allocation,
                },
            });
        }

        new_slab
    }

    fn grow_if_necessary(
        &mut self,
        new_size_in_slots: u32,
        settings: &MeshAllocatorSettings,
    ) -> SlabGrowthResult {
        // Is the slab big enough already?
        let initial_slot_capacity = self.current_slot_capacity;
        if self.current_slot_capacity >= new_size_in_slots {
            return SlabGrowthResult::NoGrowthNeeded;
        }

        // Try to grow in increments of `MeshAllocatorSettings::growth_factor`
        // until we're big enough.
        while self.current_slot_capacity < new_size_in_slots {
            let new_slab_slot_capacity =
                ((self.current_slot_capacity as f64 * settings.growth_factor).ceil() as u32)
                    .min((settings.max_slab_size / self.element_layout.slot_size()) as u32);
            if new_slab_slot_capacity == self.current_slot_capacity {
                // The slab is full.
                return SlabGrowthResult::CantGrow;
            }

            self.current_slot_capacity = new_slab_slot_capacity;
        }

        // Tell our caller what we did.
        SlabGrowthResult::NeededGrowth(SlabToReallocate {
            old_slot_capacity: initial_slot_capacity,
        })
    }
}

pub enum Slab {
    General(GeneralSlab),
}

#[derive(Default)]
struct SlabToReallocate {
    old_slot_capacity: u32,
}

#[derive(Default)]
struct SlabsToReallocate(FxHashMap<SlabId, SlabToReallocate>);

impl Deref for SlabsToReallocate {
    type Target = FxHashMap<SlabId, SlabToReallocate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SlabsToReallocate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MeshBufferSlice<'a> {
    pub range: Range<u32>,
    pub buffer: &'a ImportBufferMeta,
}

pub struct MeshAllocator {
    slabs: FxHashMap<SlabId, Slab>,
    slab_layouts: FxHashMap<ElementLayout, Vec<SlabId>>,
    mesh_key_to_vertex_slab: FxHashMap<MeshKey, SlabId>,
    mesh_key_to_index_slab: FxHashMap<MeshKey, SlabId>,
    pub general_vertex_slabs_supported: bool,
    next_slab_id: SlabId,

    pub extra_buffer_usages: BufferUsages,
}

impl MeshAllocator {
    pub fn new() -> Self {
        Self {
            slab_layouts: Default::default(),
            next_slab_id: Default::default(),
            mesh_key_to_vertex_slab: Default::default(),
            mesh_key_to_index_slab: Default::default(),
            slabs: Default::default(),
            general_vertex_slabs_supported: true,
            extra_buffer_usages: BufferUsages::empty(),
        }
    }

    pub fn mesh_vertex_slice(&self, mesh_key: &MeshKey) -> Option<MeshBufferSlice<'_>> {
        self.mesh_slice_in_slab(mesh_key, *self.mesh_key_to_vertex_slab.get(mesh_key)?)
    }

    pub fn mesh_index_slice(&self, mesh_key: &MeshKey) -> Option<MeshBufferSlice<'_>> {
        self.mesh_slice_in_slab(mesh_key, *self.mesh_key_to_index_slab.get(mesh_key)?)
    }

    fn mesh_slice_in_slab(
        &self,
        mesh_key: &MeshKey,
        slab_id: SlabId,
    ) -> Option<MeshBufferSlice<'_>> {
        match self.slabs.get(&slab_id)? {
            Slab::General(general_slab) => {
                let slab_allocation = general_slab.resident_allocations.get(mesh_key)?;
                Some(MeshBufferSlice {
                    buffer: general_slab.buffer.as_ref()?,
                    range: (slab_allocation.allocation.offset
                        * general_slab.element_layout.elements_per_slot)
                        ..((slab_allocation.allocation.offset + slab_allocation.slot_count)
                            * general_slab.element_layout.elements_per_slot),
                })
            }
        }
    }

    fn record_allocation(
        &mut self,
        mesh_key: MeshKey,
        slab_id: SlabId,
        element_class: ElementClass,
    ) {
        match element_class {
            ElementClass::Vertex => {
                self.mesh_key_to_vertex_slab.insert(mesh_key, slab_id);
            }
            ElementClass::Index => {
                self.mesh_key_to_index_slab.insert(mesh_key, slab_id);
            }
        }
    }

    fn allocate_general(
        &mut self,
        mesh_key: MeshKey,
        data_slot_count: u32,
        layout: ElementLayout,
        settings: &MeshAllocatorSettings,
        slabs_to_grow: &mut SlabsToReallocate,
    ) {
        let candidate_slabs = self.slab_layouts.entry(layout).or_default();

        let mut mesh_allocation = None;
        for &slab_id in &*candidate_slabs {
            let Some(Slab::General(slab)) = self.slabs.get_mut(&slab_id) else {
                unreachable!("Slab not found")
            };

            let Some(allocation) = slab.allocator.allocate(data_slot_count) else {
                continue;
            };

            // Try to fit the object in the slab, growing if necessary.
            match slab.grow_if_necessary(allocation.offset + data_slot_count, settings) {
                SlabGrowthResult::NoGrowthNeeded => {}
                SlabGrowthResult::NeededGrowth(slab_to_reallocate) => {
                    // If we already grew the slab this frame, don't replace the
                    // `SlabToReallocate` entry. We want to keep the entry
                    // corresponding to the size that the slab had at the start
                    // of the frame, so that we can copy only the used portion
                    // of the initial buffer to the new one.
                    if let Entry::Vacant(vacant_entry) = slabs_to_grow.entry(slab_id) {
                        vacant_entry.insert(slab_to_reallocate);
                    }
                }
                SlabGrowthResult::CantGrow => continue,
            }

            mesh_allocation = Some(MeshAllocation {
                slab_id,
                slab_allocation: SlabAllocation {
                    allocation,
                    slot_count: data_slot_count,
                },
            });
            break;
        }

        if mesh_allocation.is_none() {
            let new_slab_id = self.next_slab_id;
            self.next_slab_id.0 = NonMaxU32::new(self.next_slab_id.0.get() + 1).unwrap_or_default();

            let new_slab = GeneralSlab::new(
                new_slab_id,
                &mut mesh_allocation,
                settings,
                layout,
                data_slot_count,
            );

            self.slabs.insert(new_slab_id, Slab::General(new_slab));
            candidate_slabs.push(new_slab_id);
            slabs_to_grow.insert(new_slab_id, SlabToReallocate::default());
        }

        let mesh_allocation = mesh_allocation.expect("Should have been able to allocate");

        if let Some(Slab::General(general_slab)) = self.slabs.get_mut(&mesh_allocation.slab_id) {
            general_slab
                .pending_allocations
                .insert(mesh_key, mesh_allocation.slab_allocation);
        };

        self.record_allocation(mesh_key, mesh_allocation.slab_id, layout.class);
    }

    fn allocate(
        &mut self,
        mesh_key: MeshKey,
        data_byte_len: u64,
        layout: ElementLayout,
        settings: &MeshAllocatorSettings,
        slabs_to_grow: &mut SlabsToReallocate,
    ) {
        let data_element_count = data_byte_len.div_ceil(layout.size) as u32;
        let data_slot_count = data_element_count.div_ceil(layout.elements_per_slot);

        self.allocate_general(mesh_key, data_slot_count, layout, settings, slabs_to_grow);
    }

    fn reallocate_slab(
        &mut self,
        slab_id: SlabId,
        slab_to_grow: SlabToReallocate,
        buffer_allocator: &mut BufferAllocator,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) {
        let Some(Slab::General(slab)) = self.slabs.get_mut(&slab_id) else {
            error!("Couldn't find slab {} to grow", slab_id);
            return;
        };

        let old_buffer = slab.buffer.take();

        let mut buffer_usages = BufferUsages::COPY_SRC | BufferUsages::COPY_DST;
        match slab.element_layout.class {
            ElementClass::Vertex => buffer_usages |= BufferUsages::VERTEX,
            ElementClass::Index => buffer_usages |= BufferUsages::INDEX,
        };

        let key = format!(
            "general mesh slab {} ({}buffer)",
            slab_id,
            buffer_usages_to_str(buffer_usages)
        );

        let desc = BufferDescriptor {
            label: Some(key.clone()),
            size: slab.current_slot_capacity as u64 * slab.element_layout.slot_size(),
            usage: buffer_usages | self.extra_buffer_usages,
            mapped_at_creation: false,
        };

        let new_buffer_handle = buffer_allocator.allocate(desc.clone());
        let new_buffer = buffer_allocator.get_buffer(new_buffer_handle);

        slab.buffer = Some(ImportBufferMeta {
            key,
            value: new_buffer.clone(),
            desc,
        });

        let Some(old_buffer) = old_buffer else { return };

        let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("slab resize encoder".to_string()),
        });

        // Copy the data from the old buffer into the new one.
        encoder.copy_buffer_to_buffer(
            &old_buffer.value.get_wgpu_buffer(),
            0,
            &new_buffer.get_wgpu_buffer(),
            0,
            slab_to_grow.old_slot_capacity as u64 * slab.element_layout.slot_size(),
        );

        let command_buffer = encoder.finish();
        render_queue.submit([command_buffer]);
    }

    fn free_allocation_in_slab(
        &mut self,
        mesh_key: &MeshKey,
        slab_id: SlabId,
        empty_slabs: &mut HashSet<SlabId>,
    ) {
        let Some(slab) = self.slabs.get_mut(&slab_id) else {
            return;
        };

        match *slab {
            Slab::General(ref mut general_slab) => {
                let Some(slab_allocation) = general_slab
                    .resident_allocations
                    .remove(mesh_key)
                    .or_else(|| general_slab.pending_allocations.remove(mesh_key))
                else {
                    return;
                };

                general_slab.allocator.free(slab_allocation.allocation);

                if general_slab.is_empty() {
                    empty_slabs.insert(slab_id);
                }
            }
        }
    }

    pub fn free_meshes<'a>(&mut self, meshes_to_free: impl Iterator<Item = &'a MeshKey>) {
        let mut empty_slabs = HashSet::default();

        for mesh_id in meshes_to_free {
            if let Some(slab_id) = self.mesh_key_to_vertex_slab.remove(mesh_id) {
                self.free_allocation_in_slab(mesh_id, slab_id, &mut empty_slabs);
            }
            if let Some(slab_id) = self.mesh_key_to_index_slab.remove(mesh_id) {
                self.free_allocation_in_slab(mesh_id, slab_id, &mut empty_slabs);
            }
        }

        for empty_slab in empty_slabs {
            self.slab_layouts.values_mut().for_each(|slab_ids| {
                let idx = slab_ids.iter().position(|&slab_id| slab_id == empty_slab);
                if let Some(idx) = idx {
                    slab_ids.remove(idx);
                }
            });
            self.slabs.remove(&empty_slab);
        }
    }

    pub fn allocate_meshes(
        &mut self,
        meshs: &[MeshResource],
        settings: &MeshAllocatorSettings,
        layouts: &mut MeshVertexBufferLayouts,
        buffer_allocator: &mut BufferAllocator,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) {
        let mut slabs_to_grow = SlabsToReallocate::default();

        for mesh in meshs {
            let mesh_key = mesh.key();
            let mesh = mesh.data_ref();

            let vertex_buffer_size = mesh.get_vertex_buffer_size() as u64;
            if vertex_buffer_size == 0 {
                return;
            }

            let layout = mesh.get_mesh_vertex_buffer_layout(layouts);

            self.allocate(
                mesh_key,
                vertex_buffer_size,
                ElementLayout::vertex(&layout),
                settings,
                &mut slabs_to_grow,
            );

            if let (Some(index_buffer_data), Some(index_element_layout)) =
                (mesh.get_index_buffer_bytes(), ElementLayout::index(&mesh))
            {
                self.allocate(
                    mesh_key,
                    index_buffer_data.len() as u64,
                    index_element_layout,
                    settings,
                    &mut slabs_to_grow,
                );
            }
        }

        for (slab_id, slab_to_grow) in slabs_to_grow.0 {
            self.reallocate_slab(
                slab_id,
                slab_to_grow,
                buffer_allocator,
                render_device,
                render_queue,
            );
        }

        for mesh in meshs {
            self.copy_mesh_vertex_data(mesh, render_device, render_queue);
            self.copy_mesh_index_data(mesh, render_device, render_queue);
        }
    }

    fn copy_mesh_index_data(
        &mut self,
        mesh: &MeshResource,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) {
        let mesh_key = mesh.key();

        let Some(&slab_id) = self.mesh_key_to_index_slab.get(&mesh_key) else {
            return;
        };
        let mesh = mesh.data_ref();

        let Some(index_data) = mesh.get_index_buffer_bytes() else {
            return;
        };

        // Call the generic function.
        self.copy_element_data(
            mesh_key,
            index_data.len(),
            |slice| slice.copy_from_slice(index_data),
            BufferUsages::INDEX,
            slab_id,
            render_device,
            render_queue,
        );
    }

    fn copy_mesh_vertex_data(
        &mut self,
        mesh: &MeshResource,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) {
        let mesh_key = mesh.key();

        let Some(&slab_id) = self.mesh_key_to_vertex_slab.get(&mesh_key) else {
            return;
        };

        let mesh = mesh.data_ref();

        self.copy_element_data(
            mesh_key,
            mesh.get_vertex_buffer_size(),
            |slice| mesh.write_packed_vertex_buffer_data(slice),
            BufferUsages::VERTEX,
            slab_id,
            render_device,
            render_queue,
        );
    }

    fn copy_element_data(
        &mut self,
        mesh_key: MeshKey,
        len: usize,
        fill_data: impl Fn(&mut [u8]),
        _buffer_usages: BufferUsages,
        slab_id: SlabId,
        _render_device: &RenderDevice,
        render_queue: &RenderQueue,
    ) {
        let Some(slab) = self.slabs.get_mut(&slab_id) else {
            return;
        };

        match *slab {
            Slab::General(ref mut general_slab) => {
                let (Some(buffer), Some(allocated_range)) = (
                    &general_slab.buffer,
                    general_slab.pending_allocations.remove(&mesh_key),
                ) else {
                    return;
                };

                let slot_size = general_slab.element_layout.slot_size();

                // round up size to a multiple of the slot size to satisfy wgpu alignment requirements
                if let Some(size) = BufferSize::new((len as u64).next_multiple_of(slot_size)) {
                    // Write the data in.
                    if let Some(mut buffer) = render_queue.write_buffer_with(
                        &buffer.value,
                        allocated_range.allocation.offset as u64 * slot_size,
                        size,
                    ) {
                        let slice = &mut buffer.get_writer()[..len];
                        fill_data(slice);
                    }
                }

                // Mark the allocation as resident.
                general_slab
                    .resident_allocations
                    .insert(mesh_key, allocated_range);
            }
        }
    }
}

fn buffer_usages_to_str(buffer_usages: BufferUsages) -> &'static str {
    if buffer_usages.contains(BufferUsages::VERTEX) {
        "vertex "
    } else if buffer_usages.contains(BufferUsages::INDEX) {
        "index "
    } else {
        ""
    }
}
