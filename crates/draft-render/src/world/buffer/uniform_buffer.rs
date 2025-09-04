use core::{marker::PhantomData, num::NonZero};

use crate::frame_graph::{BufferInfo, TransientBuffer};
use draft_gfx_base::{
    BufferDescriptor, BufferInitDescriptor, BufferUsages, RenderDevice, RenderQueue,
};
use encase::{
    DynamicUniformBuffer as DynamicUniformBufferWrapper, ShaderType,
    UniformBuffer as UniformBufferWrapper,
    internal::{AlignmentValue, BufferMut, WriteInto},
};

pub struct UniformBuffer<T: ShaderType> {
    value: T,
    scratch: UniformBufferWrapper<Vec<u8>>,
    buffer: Option<TransientBuffer>,
    label: Option<String>,
    changed: bool,
    buffer_usage: BufferUsages,
}

impl<T: ShaderType> From<T> for UniformBuffer<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            scratch: UniformBufferWrapper::new(Vec::new()),
            buffer: None,
            label: None,
            changed: false,
            buffer_usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        }
    }
}

impl<T: ShaderType + Default> Default for UniformBuffer<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            scratch: UniformBufferWrapper::new(Vec::new()),
            buffer: None,
            label: None,
            changed: false,
            buffer_usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        }
    }
}

impl<T: ShaderType + WriteInto> UniformBuffer<T> {
    /// Set the data the buffer stores.
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn set_label(&mut self, label: Option<&str>) {
        let label = label.map(str::to_string);

        if label != self.label {
            self.changed = true;
        }

        self.label = label;
    }

    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Add more [`BufferUsages`] to the buffer.
    ///
    /// This method only allows addition of flags to the default usage flags.
    ///
    /// The default values for buffer usage are `BufferUsages::COPY_DST` and `BufferUsages::UNIFORM`.
    pub fn add_usages(&mut self, usage: BufferUsages) {
        self.buffer_usage |= usage;
        self.changed = true;
    }

    /// Queues writing of data from system RAM to VRAM using the [`RenderDevice`]
    /// and the provided [`RenderQueue`], if a GPU-side backing buffer already exists.
    ///
    /// If a GPU-side buffer does not already exist for this data, such a buffer is initialized with currently
    /// available data.
    pub fn write_buffer(&mut self, device: &RenderDevice, queue: &RenderQueue) {
        self.scratch.write(&self.value).unwrap();

        if self.changed || self.buffer.is_none() {
            let desc = BufferInitDescriptor {
                label: self.label.as_ref().map(|label| label.clone().into()),
                usage: self.buffer_usage,
                contents: self.scratch.as_ref(),
            };

            let buffer = device.create_buffer_init(&desc);

            self.buffer = Some(TransientBuffer {
                resource: buffer,
                desc: BufferInfo::from_buffer_init_desc(&desc),
            });

            self.changed = false;
        } else if let Some(buffer) = &self.buffer {
            queue.write_buffer(&buffer.resource, 0, self.scratch.as_ref());
        }
    }
}

pub struct DynamicUniformBuffer<T: ShaderType> {
    scratch: DynamicUniformBufferWrapper<Vec<u8>>,
    buffer: Option<TransientBuffer>,
    label: Option<String>,
    changed: bool,
    buffer_usage: BufferUsages,
    _marker: PhantomData<fn() -> T>,
}

impl<T: ShaderType> Default for DynamicUniformBuffer<T> {
    fn default() -> Self {
        Self {
            scratch: DynamicUniformBufferWrapper::new(Vec::new()),
            buffer: None,
            label: None,
            changed: false,
            buffer_usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            _marker: PhantomData,
        }
    }
}

impl<T: ShaderType + WriteInto> DynamicUniformBuffer<T> {
    pub fn new_with_alignment(alignment: u64) -> Self {
        Self {
            scratch: DynamicUniformBufferWrapper::new_with_alignment(Vec::new(), alignment),
            buffer: None,
            label: None,
            changed: false,
            buffer_usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            _marker: PhantomData,
        }
    }

    pub fn into_inner(self) -> Option<TransientBuffer> {
        self.buffer
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.scratch.as_ref().is_empty()
    }

    /// Push data into the `DynamicUniformBuffer`'s internal vector (residing on system RAM).
    #[inline]
    pub fn push(&mut self, value: &T) -> u32 {
        self.scratch.write(value).unwrap() as u32
    }

    pub fn set_label(&mut self, label: Option<&str>) {
        let label = label.map(str::to_string);

        if label != self.label {
            self.changed = true;
        }

        self.label = label;
    }

    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Add more [`BufferUsages`] to the buffer.
    ///
    /// This method only allows addition of flags to the default usage flags.
    ///
    /// The default values for buffer usage are `BufferUsages::COPY_DST` and `BufferUsages::UNIFORM`.
    pub fn add_usages(&mut self, usage: BufferUsages) {
        self.buffer_usage |= usage;
        self.changed = true;
    }

    #[inline]
    pub fn get_writer<'a>(
        &'a mut self,
        max_count: usize,
        device: &RenderDevice,
        queue: &'a RenderQueue,
    ) -> Option<DynamicUniformBufferWriter<'a, T>> {
        let alignment = if cfg!(target_abi = "sim") {
            // On iOS simulator on silicon macs, metal validation check that the host OS alignment
            // is respected, but the device reports the correct value for iOS, which is smaller.
            // Use the larger value.
            // See https://github.com/gfx-rs/wgpu/issues/7057 - remove if it's not needed anymore.
            AlignmentValue::new(256)
        } else {
            AlignmentValue::new(device.limits().min_uniform_buffer_offset_alignment as u64)
        };

        let mut capacity = self
            .buffer
            .as_ref()
            .map(|buffer| buffer.resource.size())
            .unwrap_or(0);
        let size = alignment
            .round_up(T::min_size().get())
            .checked_mul(max_count as u64)
            .unwrap();

        if capacity < size || (self.changed && size > 0) {
            let desc = BufferDescriptor {
                label: self.label.as_ref().map(|label| label.to_string().into()),
                usage: self.buffer_usage,
                size,
                mapped_at_creation: false,
            };

            let buffer = device.create_buffer(&desc);

            capacity = buffer.size();
            self.buffer = Some(TransientBuffer {
                resource: buffer,
                desc: BufferInfo::from_buffer_desc(&desc),
            });
            self.changed = false;
        }

        if let Some(ref buffer) = self.buffer {
            let buffer_view = queue
                .write_buffer_with(
                    &buffer.resource,
                    0,
                    NonZero::<u64>::new(buffer.resource.size())?,
                )
                .unwrap();
            Some(DynamicUniformBufferWriter {
                buffer: encase::DynamicUniformBuffer::new_with_alignment(
                    QueueWriteBufferViewWrapper {
                        capacity: capacity as usize,
                        buffer_view,
                    },
                    alignment.get(),
                ),
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    /// Queues writing of data from system RAM to VRAM using the [`RenderDevice`]
    /// and the provided [`RenderQueue`].
    ///
    /// If there is no GPU-side buffer allocated to hold the data currently stored, or if a GPU-side buffer previously
    /// allocated does not have enough capacity, a new GPU-side buffer is created.
    #[inline]
    pub fn write_buffer(&mut self, device: &RenderDevice, queue: &RenderQueue) {
        let capacity = self
            .buffer
            .as_ref()
            .map(|buffer| buffer.resource.size())
            .unwrap_or(0);
        let size = self.scratch.as_ref().len() as u64;

        if capacity < size || (self.changed && size > 0) {
            let desc = BufferInitDescriptor {
                label: self.label.as_ref().map(|label| label.to_string().into()),
                usage: self.buffer_usage,
                contents: self.scratch.as_ref(),
            };

            let buffer = device.create_buffer_init(&desc);

            self.buffer = Some(TransientBuffer {
                resource: buffer,
                desc: BufferInfo::from_buffer_init_desc(&desc),
            });
            self.changed = false;
        } else if let Some(buffer) = &self.buffer {
            queue.write_buffer(&buffer.resource, 0, self.scratch.as_ref());
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.scratch.as_mut().clear();
        self.scratch.set_offset(0);
    }
}

/// A writer that can be used to directly write elements into the target buffer.
///
/// For more information, see [`DynamicUniformBuffer::get_writer`].
pub struct DynamicUniformBufferWriter<'a, T> {
    buffer: encase::DynamicUniformBuffer<QueueWriteBufferViewWrapper<'a>>,
    _marker: PhantomData<fn() -> T>,
}

impl<'a, T: ShaderType + WriteInto> DynamicUniformBufferWriter<'a, T> {
    pub fn write(&mut self, value: &T) -> u32 {
        self.buffer.write(value).unwrap() as u32
    }
}

/// A wrapper to work around the orphan rule so that [`wgpu::QueueWriteBufferView`] can  implement
/// [`BufferMut`].
struct QueueWriteBufferViewWrapper<'a> {
    buffer_view: wgpu::QueueWriteBufferView<'a>,
    // Must be kept separately and cannot be retrieved from buffer_view, as the read-only access will
    // invoke a panic.
    capacity: usize,
}

impl<'a> BufferMut for QueueWriteBufferViewWrapper<'a> {
    #[inline]
    fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    fn write<const N: usize>(&mut self, offset: usize, val: &[u8; N]) {
        self.buffer_view.write(offset, val);
    }

    #[inline]
    fn write_slice(&mut self, offset: usize, val: &[u8]) {
        self.buffer_view.write_slice(offset, val);
    }
}
