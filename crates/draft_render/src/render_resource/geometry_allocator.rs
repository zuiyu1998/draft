use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

use draft_geometry::{Geometry, GeometryResource, GeometryVertexBufferLayouts, Indices};
use draft_graphics::{
    gfx_base::{Buffer, BufferDescriptor},
    wgpu::{BufferUsages, COPY_BUFFER_ALIGNMENT},
};
use fxhash::FxHashMap;
use nonmax::NonMaxU32;
use offset_allocator::{Allocation, Allocator};
use tracing::error;

type GeometryId = u64;

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

    fn vertex(layouts: &mut GeometryVertexBufferLayouts, geometry: &Geometry) -> ElementLayout {
        let layout = geometry.get_geometry_vertex_buffer_layout(layouts);
        ElementLayout::new(ElementClass::Vertex, layout.0.layout().array_stride)
    }

    fn index(geometry: &Geometry) -> Option<ElementLayout> {
        let size = match geometry.indices()? {
            Indices::U16(_) => 2,
            Indices::U32(_) => 4,
        };
        Some(ElementLayout::new(ElementClass::Index, size))
    }
}

pub struct GeometryAllocatorSettings {
    pub min_slab_size: u64,
    pub max_slab_size: u64,
    pub large_threshold: u64,
    pub growth_factor: f64,
}

impl Default for GeometryAllocatorSettings {
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
    allocation: Allocation,
    slot_count: u32,
}

struct GeometryAllocation {
    slab_id: SlabId,
    slab_allocation: SlabAllocation,
}

pub struct GeneralSlab {
    allocator: Allocator,
    buffer: Option<Buffer>,
    resident_allocations: FxHashMap<GeometryId, SlabAllocation>,
    pending_allocations: FxHashMap<GeometryId, SlabAllocation>,
    element_layout: ElementLayout,
    current_slot_capacity: u32,
}

impl GeneralSlab {
    fn new(
        new_slab_id: SlabId,
        geometry_allocation: &mut Option<GeometryAllocation>,
        settings: &GeometryAllocatorSettings,
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
            *geometry_allocation = Some(GeometryAllocation {
                slab_id: new_slab_id,
                slab_allocation: SlabAllocation {
                    slot_count: data_slot_count,
                    allocation,
                },
            });
        }

        new_slab
    }
}

pub enum Slab {
    General(GeneralSlab),
}

#[derive(Default)]
pub struct SlabToReallocate {
    old_slot_capacity: u32,
}

#[derive(Default)]
pub struct SlabsToReallocate(FxHashMap<SlabId, SlabToReallocate>);

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

pub struct GeometryAllocator {
    slabs: FxHashMap<SlabId, Slab>,

    slab_layouts: FxHashMap<ElementLayout, Vec<SlabId>>,
    geometry_id_to_vertex_slab: FxHashMap<GeometryId, SlabId>,
    geometry_id_to_index_slab: FxHashMap<GeometryId, SlabId>,

    general_vertex_slabs_supported: bool,
    next_slab_id: SlabId,

    pub extra_buffer_usages: BufferUsages,
}

impl GeometryAllocator {
    pub fn new() -> Self {
        Self {
            slab_layouts: Default::default(),
            next_slab_id: Default::default(),
            geometry_id_to_vertex_slab: Default::default(),
            geometry_id_to_index_slab: Default::default(),
            slabs: Default::default(),
            general_vertex_slabs_supported: true,
            extra_buffer_usages: BufferUsages::empty(),
        }
    }

    fn record_allocation(
        &mut self,
        geometry_id: GeometryId,
        slab_id: SlabId,
        element_class: ElementClass,
    ) {
        match element_class {
            ElementClass::Vertex => {
                self.geometry_id_to_vertex_slab.insert(geometry_id, slab_id);
            }
            ElementClass::Index => {
                self.geometry_id_to_index_slab.insert(geometry_id, slab_id);
            }
        }
    }

    fn allocate_general(
        &mut self,
        geometry_id: GeometryId,
        data_slot_count: u32,
        layout: ElementLayout,
        settings: &GeometryAllocatorSettings,
        slabs_to_grow: &mut SlabsToReallocate,
    ) {
        let candidate_slabs = self.slab_layouts.entry(layout).or_default();

        let mut geometry_allocation = None;
        for &slab_id in &*candidate_slabs {
            //todo
        }

        if geometry_allocation.is_none() {
            let new_slab_id = self.next_slab_id;
            self.next_slab_id.0 = NonMaxU32::new(self.next_slab_id.0.get() + 1).unwrap_or_default();

            let new_slab = GeneralSlab::new(
                new_slab_id,
                &mut geometry_allocation,
                settings,
                layout,
                data_slot_count,
            );

            self.slabs.insert(new_slab_id, Slab::General(new_slab));
            candidate_slabs.push(new_slab_id);
            slabs_to_grow.insert(new_slab_id, SlabToReallocate::default());
        }

        let geometry_allocation = geometry_allocation.expect("Should have been able to allocate");

        if let Some(Slab::General(general_slab)) = self.slabs.get_mut(&geometry_allocation.slab_id)
        {
            general_slab
                .pending_allocations
                .insert(geometry_id, geometry_allocation.slab_allocation);
        };

        self.record_allocation(geometry_id, geometry_allocation.slab_id, layout.class);
    }

    fn allocate(
        &mut self,
        geometry_id: GeometryId,
        data_byte_len: u64,
        layout: ElementLayout,
        settings: &GeometryAllocatorSettings,
        slabs_to_grow: &mut SlabsToReallocate,
    ) {
        let data_element_count = data_byte_len.div_ceil(layout.size) as u32;
        let data_slot_count = data_element_count.div_ceil(layout.elements_per_slot);

        self.allocate_general(
            geometry_id,
            data_slot_count,
            layout,
            settings,
            slabs_to_grow,
        );
    }

    fn reallocate_slab(&mut self, slab_id: SlabId, slab_to_grow: SlabToReallocate) {
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
    }

    pub fn allocate_geometry(
        &mut self,
        layouts: &mut GeometryVertexBufferLayouts,
        geometry: &GeometryResource,
        settings: &GeometryAllocatorSettings,
        slabs_to_grow: &mut SlabsToReallocate,
    ) {
        let geometry_id = geometry.key();
        let geometry = geometry.data_ref();

        let vertex_buffer_size = geometry.get_vertex_buffer_size() as u64;
        if vertex_buffer_size == 0 {
            return;
        }

        self.allocate(
            geometry_id,
            vertex_buffer_size,
            ElementLayout::vertex(layouts, &geometry),
            settings,
            slabs_to_grow,
        );

        if let (Some(index_buffer_data), Some(index_element_layout)) = (
            geometry.get_index_buffer_bytes(),
            ElementLayout::index(&geometry),
        ) {
            self.allocate(
                geometry_id,
                index_buffer_data.len() as u64,
                index_element_layout,
                settings,
                slabs_to_grow,
            );
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
