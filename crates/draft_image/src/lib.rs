mod loader;

use std::path::Path;

use image::DynamicImage;
pub use loader::*;

use fyrox_core::{
    TypeUuidProvider, Uuid,
    io::FileError,
    reflect::*,
    uuid,
    visitor::{pod::PodVecView, *},
};
use fyrox_resource::{ResourceData, io::ResourceIo, options::ImportOptions};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wgpu_types::{Extent3d, TextureDimension, TextureFormat};

#[derive(Debug, Default, Reflect, Clone, Deserialize, Serialize)]
pub struct ImageImportOptions {
    pub is_srgb: bool,
}

impl ImportOptions for ImageImportOptions {}

/// An error that occurs when accessing specific pixels in a texture.
#[derive(Error, Debug)]
pub enum TextureAccessError {
    /// Attempted to access a pixel outside the texture bounds.
    #[error("out of bounds (x: {x}, y: {y}, z: {z})")]
    OutOfBounds {
        /// The pixel x-coordinate.
        x: u32,
        /// The pixel y-coordinate.
        y: u32,
        /// The pixel z-coordinate.
        z: u32,
    },
    /// Attempted to perform an image operation on an unsupported texture format.
    ///
    /// Most often this is returned when attempting to access pixel data of compressed textures.
    #[error("unsupported texture format: {0:?}")]
    UnsupportedTextureFormat(TextureFormat),
    /// Attempted to access the data of an image before it was initialized, or after it was moved
    /// to the GPU.
    ///
    /// See [`RenderAssetUsages`] for more information about when an asset's data is moved, and
    /// how to retain it if necessary.
    #[error("image data is not initialized")]
    Uninitialized,
    /// The texture's dimension was different than indicated by the accessor used.
    #[error("attempt to access texture with different dimension")]
    WrongDimension,
}

/// Extends the wgpu [`TextureFormat`] with information about the pixel.
pub trait TextureFormatPixelInfo {
    /// Returns the size of a pixel in bytes of the format.
    /// error with `TextureAccessError::UnsupportedTextureFormat` if the format is compressed.
    fn pixel_size(&self) -> Result<usize, TextureAccessError>;
}

impl TextureFormatPixelInfo for TextureFormat {
    fn pixel_size(&self) -> Result<usize, TextureAccessError> {
        let info = self;
        match info.block_dimensions() {
            (1, 1) => Ok(info.block_copy_size(None).unwrap() as usize),
            _ => Err(TextureAccessError::UnsupportedTextureFormat(*self)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ImageError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    FileLoadError(#[from] FileError),
    #[error("invalid image extension: {0}")]
    InvalidImageExtension(String),
    #[error("failed to load an image: {0}")]
    ImageError(#[from] image::ImageError),
}

#[derive(Debug, Reflect, Clone)]
pub struct Image {
    pub data: Vec<u8>,
}

pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
}

impl ImageFormat {
    pub fn from_extension(extension: &str) -> Option<ImageFormat> {
        match extension.to_ascii_uppercase().as_str() {
            "png" => Some(ImageFormat::Png),
            "jpeg" => Some(ImageFormat::Jpeg),
            "bmp" => Some(ImageFormat::Bmp),
            _ => None,
        }
    }

    pub fn to_image_format(self) -> image::ImageFormat {
        match self {
            ImageFormat::Bmp => image::ImageFormat::Bmp,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Png => image::ImageFormat::Png,
        }
    }
}

/// Calculates the total number of pixels in the item.
fn pixel_count(item: Extent3d) -> usize {
    (item.width * item.height * item.depth_or_array_layers) as usize
}

impl Image {
    pub fn new(
        size: Extent3d,
        dimension: TextureDimension,
        data: Vec<u8>,
        format: TextureFormat,
    ) -> Self {
        if let Ok(pixel_size) = format.pixel_size() {
            debug_assert_eq!(
                pixel_count(size) * pixel_size,
                data.len(),
                "Pixel data, size and format have to match",
            );
        }
        let mut image = Image::new_uninit(size, dimension, format);
        image.data = data;
        image
    }

    pub fn new_uninit(size: Extent3d, dimension: TextureDimension, format: TextureFormat) -> Self {
        Image {
            data: vec![],
            // texture_descriptor: TextureDescriptor {
            //     size,
            //     format,
            //     dimension,
            //     label: None,
            //     mip_level_count: 1,
            //     sample_count: 1,
            //     usage: TextureUsages::TEXTURE_BINDING
            //         | TextureUsages::COPY_DST
            //         | TextureUsages::COPY_SRC,
            //     view_formats: &[],
            // },
        }
    }

    pub fn from_dynamic(dyn_img: DynamicImage, is_srgb: bool) -> Image {
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

                let mut local_data = Vec::with_capacity(
                    width as usize * height as usize * format.pixel_size().unwrap_or(0),
                );

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

        Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            format,
        )
    }

    pub fn load_from_memory(
        data: &[u8],
        import_options: ImageImportOptions,
        image_format: ImageFormat,
    ) -> Result<Self, ImageError> {
        let mut reader = image::ImageReader::new(std::io::Cursor::new(data));
        reader.set_format(image_format.to_image_format());
        reader.no_limits();
        let dyn_img = reader.decode()?;

        Ok(Self::from_dynamic(dyn_img, import_options.is_srgb))
    }

    pub(crate) async fn load_from_file<P: AsRef<Path>>(
        path: P,
        io: &dyn ResourceIo,
        import_options: ImageImportOptions,
    ) -> Result<Self, ImageError> {
        let ext = path.as_ref().extension().unwrap().to_str().unwrap();

        let image_format = ImageFormat::from_extension(ext)
            .ok_or_else(|| ImageError::InvalidImageExtension(ext.to_string()))?;

        let data = io.load_file(path.as_ref()).await?;
        Self::load_from_memory(&data, import_options, image_format)
    }
}

impl Visit for Image {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut region = visitor.enter_region(name)?;

        let mut bytes_view = PodVecView::from_pod_vec(&mut self.data);
        bytes_view.visit("Data", &mut region)?;

        Ok(())
    }
}

impl TypeUuidProvider for Image {
    fn type_uuid() -> Uuid {
        uuid!("f41402e3-19d7-4209-b14f-e26603344e24")
    }
}

impl ResourceData for Image {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
