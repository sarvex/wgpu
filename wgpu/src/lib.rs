//! A cross-platform graphics and compute library based on [WebGPU](https://gpuweb.github.io/gpuweb/).
//!
//! To start using the API, create an [`Instance`].
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!
//! ### Feature Aliases
//!
//! These features aren't actually features on the crate itself, but a convenient shorthand for
//! complicated cases.
//!
//! - **`wgpu_core`** --- Enabled when there is any non-webgpu backend enabled on the platform.
//! - **`naga`** ---- Enabled when any non-wgsl shader input is enabled.
//!

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc(html_logo_url = "https://raw.githubusercontent.com/gfx-rs/wgpu/trunk/logo.png")]
#![warn(
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    missing_docs,
    rust_2018_idioms,
    unsafe_op_in_unsafe_fn
)]
// NOTE: Keep this in sync with `wgpu-core`.
#![cfg_attr(not(send_sync), allow(clippy::arc_with_non_send_sync))]
#![cfg_attr(not(any(wgpu_core, webgpu)), allow(unused))]

extern crate alloc;
#[cfg(std)]
extern crate std;
#[cfg(wgpu_core)]
pub extern crate wgpu_core as wgc;
#[cfg(wgpu_core)]
pub extern crate wgpu_hal as hal;
pub extern crate wgpu_types as wgt;

//
//
// Modules
//
//

mod api;
mod backend;
mod cmp;
mod dispatch;
mod macros;
pub mod util;

//
//
// Private re-exports
//
//

//
//
// Public re-exports
//
//

#[cfg(custom)]
pub use backend::custom;

pub use api::*;
pub use wgt::{
    AdapterInfo, AddressMode, AllocatorReport, AstcBlock, AstcChannel, Backend, BackendOptions,
    Backends, BindGroupLayoutEntry, BindingType, BlendComponent, BlendFactor, BlendOperation,
    BlendState, BufferAddress, BufferBindingType, BufferSize, BufferTextureCopyInfo,
    BufferTransition, BufferUsages, BufferUses, Color, ColorTargetState, ColorWrites,
    CommandBufferDescriptor, CompareFunction, CompositeAlphaMode, CopyExternalImageDestInfo,
    CoreCounters, DepthBiasState, DepthStencilState, DeviceLostReason, DeviceType,
    DownlevelCapabilities, DownlevelFlags, DownlevelLimits, Dx12BackendOptions, Dx12Compiler,
    DxcShaderModel, DynamicOffset, Extent3d, Face, Features, FeaturesWGPU, FeaturesWebGPU,
    FilterMode, FrontFace, GlBackendOptions, GlFenceBehavior, Gles3MinorVersion, HalCounters,
    ImageSubresourceRange, IndexFormat, InstanceDescriptor, InstanceFlags, InternalCounters,
    Limits, MemoryBudgetThresholds, MemoryHints, MultisampleState, NoopBackendOptions, Origin2d,
    Origin3d, PipelineStatisticsTypes, PollError, PollStatus, PolygonMode, PowerPreference,
    PredefinedColorSpace, PresentMode, PresentationTimestamp, PrimitiveState, PrimitiveTopology,
    PushConstantRange, QueryType, RenderBundleDepthStencil, RequestAdapterError,
    SamplerBindingType, SamplerBorderColor, ShaderLocation, ShaderModel, ShaderRuntimeChecks,
    ShaderStages, StencilFaceState, StencilOperation, StencilState, StorageTextureAccess,
    SurfaceCapabilities, SurfaceStatus, TexelCopyBufferLayout, TextureAspect, TextureDimension,
    TextureFormat, TextureFormatFeatureFlags, TextureFormatFeatures, TextureSampleType,
    TextureTransition, TextureUsages, TextureUses, TextureViewDimension, Trace, VertexAttribute,
    VertexFormat, VertexStepMode, WasmNotSend, WasmNotSendSync, WasmNotSync, COPY_BUFFER_ALIGNMENT,
    COPY_BYTES_PER_ROW_ALIGNMENT, MAP_ALIGNMENT, PUSH_CONSTANT_ALIGNMENT,
    QUERY_RESOLVE_BUFFER_ALIGNMENT, QUERY_SET_MAX_QUERIES, QUERY_SIZE, VERTEX_STRIDE_ALIGNMENT,
};
// wasm-only types, we try to keep as many types non-platform
// specific, but these need to depend on web-sys.
#[cfg(web)]
pub use wgt::{CopyExternalImageSourceInfo, ExternalImageSource};

/// Re-export of our `naga` dependency.
///
#[cfg(wgpu_core)]
#[cfg_attr(docsrs, doc(cfg(any(wgpu_core, naga))))]
// We re-export wgpu-core's re-export of naga, as we may not have direct access to it.
pub use ::wgc::naga;
/// Re-export of our `naga` dependency.
///
#[cfg(all(not(wgpu_core), naga))]
#[cfg_attr(docsrs, doc(cfg(any(wgpu_core, naga))))]
// If that's not available, we re-export our own.
pub use naga;

/// Re-export of our `raw-window-handle` dependency.
///
pub use raw_window_handle as rwh;

/// Re-export of our `web-sys` dependency.
///
#[cfg(web)]
pub use web_sys;

#[doc(hidden)]
pub use macros::helpers as __macro_helpers;
