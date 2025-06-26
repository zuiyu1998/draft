use crate::{GeometryResource, MaterialResource, TextureResource, gfx_base::RawTextureView};

pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
}

pub struct SceneRenderData<'a> {
    pub batch: &'a Batch,
    pub texture_view: RawTextureView,
    pub image: &'a TextureResource,
}
