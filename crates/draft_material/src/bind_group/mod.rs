mod bind_group_layout;
mod bind_group_layout_entries;

pub use bind_group_layout_entries::*;
pub use bind_group_layout::*;

use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroup {
    pub name: String,
    pub layout: MaterialBindGroupLayout,
}
