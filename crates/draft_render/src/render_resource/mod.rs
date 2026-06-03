mod batched_uniform_buffer;
mod bind_group_layout;
mod gpu_array_buffer;
mod uniform_buffer;

pub use batched_uniform_buffer::*;
pub use bind_group_layout::*;
pub use gpu_array_buffer::*;
pub use uniform_buffer::*;

pub mod encase {
    pub use encase::*;
}

use encase::{ShaderSize, ShaderType, internal::WriteInto};

pub trait GpuArrayBufferable: ShaderType + ShaderSize + WriteInto + Clone {}

impl<T: ShaderType + ShaderSize + WriteInto + Clone> GpuArrayBufferable for T {}
