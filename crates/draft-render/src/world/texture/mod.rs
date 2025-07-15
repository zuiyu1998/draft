mod cache;
mod loader;

pub use cache::*;
use image::{DynamicImage, ImageFormat};
pub use loader::*;

use fyrox_core::{
    TypeUuidProvider, Uuid, io::FileError, reflect::*, sparse::AtomicIndex, uuid, visitor::*,
};
use fyrox_resource::{Resource, ResourceData, io::ResourceIo, options::ImportOptions};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    path::Path,
    sync::Arc,
};
use thiserror::Error;

use crate::{
    frame_graph::TextureInfo,
    gfx_base::{
        Extent3d, RawSamplerDescriptor, SamplerInfo, TextureDimension, TextureFormat,
        TextureFormatPixelInfo, TextureUsages, Volume,
    },
};

pub type TextureResource = Resource<Texture>;

#[derive(Clone, Deserialize, Serialize, Debug, Reflect)]
pub struct TextureImportOptions {
    sampler_info: SamplerInfo,
    is_srgb: bool,
}

impl Default for TextureImportOptions {
    fn default() -> Self {
        TextureImportOptions {
            sampler_info: Default::default(),
            is_srgb: true,
        }
    }
}

impl ImportOptions for TextureImportOptions {}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct TextureSamplerInfo {
    info: SamplerInfo,
    #[visit(optional)]
    modifications_counter: u64,
}

impl TextureSamplerInfo {
    pub fn as_desc(&self) -> RawSamplerDescriptor {
        self.info.as_desc()
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct Image {
    pub bytes: TextureBytes,
    pub texture_info: TextureInfo,
    #[visit(optional)]
    modifications_counter: u64,
}

#[derive(Debug, Error)]
pub enum TextureError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("A file load error has occurred {0:?}")]
    FileLoadError(FileError),
    #[error(transparent)]
    Image(#[from] image::ImageError),
}

impl From<FileError> for TextureError {
    fn from(value: FileError) -> Self {
        TextureError::FileLoadError(value)
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "8ebc2e08-a5ae-4fd0-9ef7-6882d73ac871")]
pub struct Texture {
    sampler_info: TextureSamplerInfo,
    image: Image,
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Texture {
    pub fn new(
        size: Extent3d,
        dimension: TextureDimension,
        data: Vec<u8>,
        format: TextureFormat,
        sampler_info: SamplerInfo,
    ) -> Self {
        debug_assert_eq!(
            size.volume() * format.pixel_size(),
            data.len(),
            "Pixel data, size and format have to match",
        );
        Texture::new_uninit(size, dimension, format, sampler_info, data)
    }

    pub fn new_uninit(
        size: Extent3d,
        dimension: TextureDimension,
        format: TextureFormat,
        sampler_info: SamplerInfo,
        data: Vec<u8>,
    ) -> Self {
        Texture {
            sampler_info: TextureSamplerInfo {
                info: sampler_info,
                modifications_counter: 0,
            },
            image: Image {
                bytes: TextureBytes(data),
                texture_info: TextureInfo {
                    size,
                    format,
                    dimension,
                    label: None,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::COPY_SRC,
                    view_formats: vec![],
                },
                modifications_counter: 0,
            },
            cache_index: Default::default(),
        }
    }

    pub fn from_dynamic(dyn_img: DynamicImage, import_options: TextureImportOptions) -> Texture {
        let is_srgb = import_options.is_srgb;

        use bytemuck::cast_slice;
        let width;
        let height;

        let data: Vec<u8>;
        let format: TextureFormat;

        match dyn_img {
            DynamicImage::ImageLuma8(image) => {
                let i = DynamicImage::ImageLuma8(image).into_rgba8();
                width = i.width();
                height = i.height();
                format = if is_srgb {
                    TextureFormat::Rgba8UnormSrgb
                } else {
                    TextureFormat::Rgba8Unorm
                };

                data = i.into_raw();
            }
            DynamicImage::ImageLumaA8(image) => {
                let i = DynamicImage::ImageLumaA8(image).into_rgba8();
                width = i.width();
                height = i.height();
                format = if is_srgb {
                    TextureFormat::Rgba8UnormSrgb
                } else {
                    TextureFormat::Rgba8Unorm
                };

                data = i.into_raw();
            }
            DynamicImage::ImageRgb8(image) => {
                let i = DynamicImage::ImageRgb8(image).into_rgba8();
                width = i.width();
                height = i.height();
                format = if is_srgb {
                    TextureFormat::Rgba8UnormSrgb
                } else {
                    TextureFormat::Rgba8Unorm
                };

                data = i.into_raw();
            }
            DynamicImage::ImageRgba8(image) => {
                width = image.width();
                height = image.height();
                format = if is_srgb {
                    TextureFormat::Rgba8UnormSrgb
                } else {
                    TextureFormat::Rgba8Unorm
                };

                data = image.into_raw();
            }
            DynamicImage::ImageLuma16(image) => {
                width = image.width();
                height = image.height();
                format = TextureFormat::R16Uint;

                let raw_data = image.into_raw();

                data = cast_slice(&raw_data).to_owned();
            }
            DynamicImage::ImageLumaA16(image) => {
                width = image.width();
                height = image.height();
                format = TextureFormat::Rg16Uint;

                let raw_data = image.into_raw();

                data = cast_slice(&raw_data).to_owned();
            }
            DynamicImage::ImageRgb16(image) => {
                let i = DynamicImage::ImageRgb16(image).into_rgba16();
                width = i.width();
                height = i.height();
                format = TextureFormat::Rgba16Unorm;

                let raw_data = i.into_raw();

                data = cast_slice(&raw_data).to_owned();
            }
            DynamicImage::ImageRgba16(image) => {
                width = image.width();
                height = image.height();
                format = TextureFormat::Rgba16Unorm;

                let raw_data = image.into_raw();

                data = cast_slice(&raw_data).to_owned();
            }
            DynamicImage::ImageRgb32F(image) => {
                width = image.width();
                height = image.height();
                format = TextureFormat::Rgba32Float;

                let mut local_data =
                    Vec::with_capacity(width as usize * height as usize * format.pixel_size());

                for pixel in image.into_raw().chunks_exact(3) {
                    // TODO: use the array_chunks method once stabilized
                    // https://github.com/rust-lang/rust/issues/74985
                    let r = pixel[0];
                    let g = pixel[1];
                    let b = pixel[2];
                    let a = 1f32;

                    local_data.extend_from_slice(&r.to_le_bytes());
                    local_data.extend_from_slice(&g.to_le_bytes());
                    local_data.extend_from_slice(&b.to_le_bytes());
                    local_data.extend_from_slice(&a.to_le_bytes());
                }

                data = local_data;
            }
            DynamicImage::ImageRgba32F(image) => {
                width = image.width();
                height = image.height();
                format = TextureFormat::Rgba32Float;

                let raw_data = image.into_raw();

                data = cast_slice(&raw_data).to_owned();
            }
            // DynamicImage is now non exhaustive, catch future variants and convert them
            _ => {
                let image = dyn_img.into_rgba8();
                width = image.width();
                height = image.height();
                format = TextureFormat::Rgba8UnormSrgb;

                data = image.into_raw();
            }
        }

        Texture::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            format,
            import_options.sampler_info,
        )
    }

    pub fn load_from_memory(
        data: &[u8],
        import_options: TextureImportOptions,
    ) -> Result<Self, TextureError> {
        let dyn_img = image::load_from_memory(data)
            // Try to load as TGA, this is needed because TGA is badly designed format and does not
            // have an identifier in the beginning of the file (so called "magic") that allows quickly
            // check if the file is really contains expected data.
            .or_else(|_| image::load_from_memory_with_format(data, ImageFormat::Tga))?;

        Ok(Self::from_dynamic(dyn_img, import_options))
    }

    pub(crate) async fn load_from_file<P: AsRef<Path>>(
        path: P,
        io: &dyn ResourceIo,
        import_options: TextureImportOptions,
    ) -> Result<Self, TextureError> {
        let data = io.load_file(path.as_ref()).await?;
        Self::load_from_memory(&data, import_options)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.image.bytes
    }
}

impl ResourceData for Texture {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, _path: &Path) -> Result<(), Box<dyn Error>> {
        //todo
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

#[derive(Default, Clone, Reflect)]
pub struct TextureBytes(Vec<u8>);

impl Visit for TextureBytes {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.0.visit(name, visitor)
    }
}

impl Debug for TextureBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture has {} bytes", self.0.len())
    }
}

impl From<Vec<u8>> for TextureBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Deref for TextureBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
