use crate::*;

/// Handle to a texture on the GPU.
///
/// It can be created with [`Device::create_texture`].
///
/// Corresponds to [WebGPU `GPUTexture`](https://gpuweb.github.io/gpuweb/#texture-interface).
#[derive(Debug, Clone)]
pub struct Texture {
    pub(crate) inner: dispatch::DispatchTexture,
    pub(crate) descriptor: TextureDescriptor<'static>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(Texture: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(Texture => .inner);

impl Texture {
    /// Returns the inner hal Texture using a callback. The hal texture will be `None` if the
    /// backend type argument does not match with this wgpu Texture
    ///
    /// # Safety
    ///
    /// - The raw handle obtained from the hal Texture must not be manually destroyed
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal<A: wgc::hal_api::HalApi, F: FnOnce(Option<&A::Texture>) -> R, R>(
        &self,
        hal_texture_callback: F,
    ) -> R {
        if let Some(tex) = self.inner.as_core_opt() {
            unsafe {
                tex.context
                    .texture_as_hal::<A, F, R>(tex, hal_texture_callback)
            }
        } else {
            hal_texture_callback(None)
        }
    }

    #[cfg(custom)]
    /// Returns custom implementation of Texture (if custom backend and is internally T)
    pub fn as_custom<T: custom::TextureInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }

    /// Creates a view of this texture, specifying an interpretation of its texels and
    /// possibly a subset of its layers and mip levels.
    ///
    /// Texture views are needed to use a texture as a binding in a [`BindGroup`]
    /// or as an attachment in a [`RenderPass`].
    pub fn create_view(&self, desc: &TextureViewDescriptor<'_>) -> TextureView {
        let view = self.inner.create_view(desc);

        TextureView { inner: view }
    }

    /// Destroy the associated native resources as soon as possible.
    pub fn destroy(&self) {
        self.inner.destroy();
    }

    /// Make an `TexelCopyTextureInfo` representing the whole texture.
    pub fn as_image_copy(&self) -> TexelCopyTextureInfo<'_> {
        TexelCopyTextureInfo {
            texture: self,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        }
    }

    /// Returns the size of this `Texture`.
    ///
    /// This is always equal to the `size` that was specified when creating the texture.
    pub fn size(&self) -> Extent3d {
        self.descriptor.size
    }

    /// Returns the width of this `Texture`.
    ///
    /// This is always equal to the `size.width` that was specified when creating the texture.
    pub fn width(&self) -> u32 {
        self.descriptor.size.width
    }

    /// Returns the height of this `Texture`.
    ///
    /// This is always equal to the `size.height` that was specified when creating the texture.
    pub fn height(&self) -> u32 {
        self.descriptor.size.height
    }

    /// Returns the depth or layer count of this `Texture`.
    ///
    /// This is always equal to the `size.depth_or_array_layers` that was specified when creating the texture.
    pub fn depth_or_array_layers(&self) -> u32 {
        self.descriptor.size.depth_or_array_layers
    }

    /// Returns the mip_level_count of this `Texture`.
    ///
    /// This is always equal to the `mip_level_count` that was specified when creating the texture.
    pub fn mip_level_count(&self) -> u32 {
        self.descriptor.mip_level_count
    }

    /// Returns the sample_count of this `Texture`.
    ///
    /// This is always equal to the `sample_count` that was specified when creating the texture.
    pub fn sample_count(&self) -> u32 {
        self.descriptor.sample_count
    }

    /// Returns the dimension of this `Texture`.
    ///
    /// This is always equal to the `dimension` that was specified when creating the texture.
    pub fn dimension(&self) -> TextureDimension {
        self.descriptor.dimension
    }

    /// Returns the format of this `Texture`.
    ///
    /// This is always equal to the `format` that was specified when creating the texture.
    pub fn format(&self) -> TextureFormat {
        self.descriptor.format
    }

    /// Returns the allowed usages of this `Texture`.
    ///
    /// This is always equal to the `usage` that was specified when creating the texture.
    pub fn usage(&self) -> TextureUsages {
        self.descriptor.usage
    }
}

/// Describes a [`Texture`].
///
/// For use with [`Device::create_texture`].
///
/// Corresponds to [WebGPU `GPUTextureDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gputexturedescriptor).
pub type TextureDescriptor<'a> = wgt::TextureDescriptor<Label<'a>, &'a [TextureFormat]>;
static_assertions::assert_impl_all!(TextureDescriptor<'_>: Send, Sync);
