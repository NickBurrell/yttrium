use cxx::type_id;
use cxx::ExternType;
use ffi::UnityRenderingExtEventType;
use std::marker::PhantomData;

pub mod graphics;
pub mod interface;
pub mod rendering_extensions;

extern "Rust" {
    fn __unity_rs_entry_point(lib: &mut Unity);
}

pub struct Unity {
    _not_send: PhantomData<*const ()>,
    instances: interface::InterfaceManager,
}

impl Unity {
    fn new(interfaces: *mut ffi::IUnityInterfaces) -> Unity {
        Unity {
            _not_send: PhantomData,
            instances: interface::InterfaceManager::new(interfaces),
        }
    }
}

#[cxx::bridge]
pub(crate) mod ffi {
    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub(crate) enum UnityRenderingExtEventType {
        kUnityRenderingExtEventSetStereoTarget = 0,
        kUnityRenderingExtEventSetStereoEye = 1,
        kUnityRenderingExtEventStereoRenderingDone = 2,
        kUnityRenderingExtEventBeforeDrawCall = 3,
        kUnityRenderingExtEventAfterDrawCall = 4,
        kUnityRenderingExtEventCustomGrab = 5,
        kUnityRenderingExtEventCustomBlit = 6,
        kUnityRenderingExtEventUpdateTextureBegin = 7,
        kUnityRenderingExtEventUpdateTextureEnd = 8,
        kUnityRenderingExtEventUpdateTextureBeginV2 = 9,
        kUnityRenderingExtEventUpdateTextureEndV2 = 10,
        kUnityRenderingExtEventCount = 11,
    }

    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub(crate) enum UnityRenderingExtTextureFormat {
        kUnityRenderingExtFormatNone = 0,

        // sRGB formats
        kUnityRenderingExtFormatR8_SRGB,
        kUnityRenderingExtFormatR8G8_SRGB,
        kUnityRenderingExtFormatR8G8B8_SRGB,
        kUnityRenderingExtFormatR8G8B8A8_SRGB,

        // 8 bit integer formats
        kUnityRenderingExtFormatR8_UNorm,
        kUnityRenderingExtFormatR8G8_UNorm,
        kUnityRenderingExtFormatR8G8B8_UNorm,
        kUnityRenderingExtFormatR8G8B8A8_UNorm,
        kUnityRenderingExtFormatR8_SNorm,
        kUnityRenderingExtFormatR8G8_SNorm,
        kUnityRenderingExtFormatR8G8B8_SNorm,
        kUnityRenderingExtFormatR8G8B8A8_SNorm,
        kUnityRenderingExtFormatR8_UInt,
        kUnityRenderingExtFormatR8G8_UInt,
        kUnityRenderingExtFormatR8G8B8_UInt,
        kUnityRenderingExtFormatR8G8B8A8_UInt,
        kUnityRenderingExtFormatR8_SInt,
        kUnityRenderingExtFormatR8G8_SInt,
        kUnityRenderingExtFormatR8G8B8_SInt,
        kUnityRenderingExtFormatR8G8B8A8_SInt,

        // 16 bit integer formats
        kUnityRenderingExtFormatR16_UNorm,
        kUnityRenderingExtFormatR16G16_UNorm,
        kUnityRenderingExtFormatR16G16B16_UNorm,
        kUnityRenderingExtFormatR16G16B16A16_UNorm,
        kUnityRenderingExtFormatR16_SNorm,
        kUnityRenderingExtFormatR16G16_SNorm,
        kUnityRenderingExtFormatR16G16B16_SNorm,
        kUnityRenderingExtFormatR16G16B16A16_SNorm,
        kUnityRenderingExtFormatR16_UInt,
        kUnityRenderingExtFormatR16G16_UInt,
        kUnityRenderingExtFormatR16G16B16_UInt,
        kUnityRenderingExtFormatR16G16B16A16_UInt,
        kUnityRenderingExtFormatR16_SInt,
        kUnityRenderingExtFormatR16G16_SInt,
        kUnityRenderingExtFormatR16G16B16_SInt,
        kUnityRenderingExtFormatR16G16B16A16_SInt,

        // 32 bit integer formats
        kUnityRenderingExtFormatR32_UInt,
        kUnityRenderingExtFormatR32G32_UInt,
        kUnityRenderingExtFormatR32G32B32_UInt,
        kUnityRenderingExtFormatR32G32B32A32_UInt,
        kUnityRenderingExtFormatR32_SInt,
        kUnityRenderingExtFormatR32G32_SInt,
        kUnityRenderingExtFormatR32G32B32_SInt,
        kUnityRenderingExtFormatR32G32B32A32_SInt,

        // HDR formats
        kUnityRenderingExtFormatR16_SFloat,
        kUnityRenderingExtFormatR16G16_SFloat,
        kUnityRenderingExtFormatR16G16B16_SFloat,
        kUnityRenderingExtFormatR16G16B16A16_SFloat,
        kUnityRenderingExtFormatR32_SFloat,
        kUnityRenderingExtFormatR32G32_SFloat,
        kUnityRenderingExtFormatR32G32B32_SFloat,
        kUnityRenderingExtFormatR32G32B32A32_SFloat,

        // Luminance and Alpha format
        kUnityRenderingExtFormatL8_UNorm,
        kUnityRenderingExtFormatA8_UNorm,
        kUnityRenderingExtFormatA16_UNorm,

        // BGR formats
        kUnityRenderingExtFormatB8G8R8_SRGB,
        kUnityRenderingExtFormatB8G8R8A8_SRGB,
        kUnityRenderingExtFormatB8G8R8_UNorm,
        kUnityRenderingExtFormatB8G8R8A8_UNorm,
        kUnityRenderingExtFormatB8G8R8_SNorm,
        kUnityRenderingExtFormatB8G8R8A8_SNorm,
        kUnityRenderingExtFormatB8G8R8_UInt,
        kUnityRenderingExtFormatB8G8R8A8_UInt,
        kUnityRenderingExtFormatB8G8R8_SInt,
        kUnityRenderingExtFormatB8G8R8A8_SInt,

        // 16 bit packed formats
        kUnityRenderingExtFormatR4G4B4A4_UNormPack16,
        kUnityRenderingExtFormatB4G4R4A4_UNormPack16,
        kUnityRenderingExtFormatR5G6B5_UNormPack16,
        kUnityRenderingExtFormatB5G6R5_UNormPack16,
        kUnityRenderingExtFormatR5G5B5A1_UNormPack16,
        kUnityRenderingExtFormatB5G5R5A1_UNormPack16,
        kUnityRenderingExtFormatA1R5G5B5_UNormPack16,

        // Packed formats
        kUnityRenderingExtFormatE5B9G9R9_UFloatPack32,
        kUnityRenderingExtFormatB10G11R11_UFloatPack32,

        kUnityRenderingExtFormatA2B10G10R10_UNormPack32,
        kUnityRenderingExtFormatA2B10G10R10_UIntPack32,
        kUnityRenderingExtFormatA2B10G10R10_SIntPack32,
        kUnityRenderingExtFormatA2R10G10B10_UNormPack32,
        kUnityRenderingExtFormatA2R10G10B10_UIntPack32,
        kUnityRenderingExtFormatA2R10G10B10_SIntPack32,
        kUnityRenderingExtFormatA2R10G10B10_XRSRGBPack32,
        kUnityRenderingExtFormatA2R10G10B10_XRUNormPack32,
        kUnityRenderingExtFormatR10G10B10_XRSRGBPack32,
        kUnityRenderingExtFormatR10G10B10_XRUNormPack32,
        kUnityRenderingExtFormatA10R10G10B10_XRSRGBPack32,
        kUnityRenderingExtFormatA10R10G10B10_XRUNormPack32,

        // ARGB formats... TextureFormat legacy
        kUnityRenderingExtFormatA8R8G8B8_SRGB,
        kUnityRenderingExtFormatA8R8G8B8_UNorm,
        kUnityRenderingExtFormatA32R32G32B32_SFloat,

        // Depth Stencil for formats
        kUnityRenderingExtFormatD16_UNorm,
        kUnityRenderingExtFormatD24_UNorm,
        kUnityRenderingExtFormatD24_UNorm_S8_UInt,
        kUnityRenderingExtFormatD32_SFloat,
        kUnityRenderingExtFormatD32_SFloat_S8_Uint,
        kUnityRenderingExtFormatS8_Uint,

        // Compression formats
        kUnityRenderingExtFormatRGBA_DXT1_SRGB,
        kUnityRenderingExtFormatRGBA_DXT1_UNorm,
        kUnityRenderingExtFormatRGBA_DXT3_SRGB,
        kUnityRenderingExtFormatRGBA_DXT3_UNorm,
        kUnityRenderingExtFormatRGBA_DXT5_SRGB,
        kUnityRenderingExtFormatRGBA_DXT5_UNorm,
        kUnityRenderingExtFormatR_BC4_UNorm,
        kUnityRenderingExtFormatR_BC4_SNorm,
        kUnityRenderingExtFormatRG_BC5_UNorm,
        kUnityRenderingExtFormatRG_BC5_SNorm,
        kUnityRenderingExtFormatRGB_BC6H_UFloat,
        kUnityRenderingExtFormatRGB_BC6H_SFloat,
        kUnityRenderingExtFormatRGBA_BC7_SRGB,
        kUnityRenderingExtFormatRGBA_BC7_UNorm,

        kUnityRenderingExtFormatRGB_PVRTC_2Bpp_SRGB,
        kUnityRenderingExtFormatRGB_PVRTC_2Bpp_UNorm,
        kUnityRenderingExtFormatRGB_PVRTC_4Bpp_SRGB,
        kUnityRenderingExtFormatRGB_PVRTC_4Bpp_UNorm,
        kUnityRenderingExtFormatRGBA_PVRTC_2Bpp_SRGB,
        kUnityRenderingExtFormatRGBA_PVRTC_2Bpp_UNorm,
        kUnityRenderingExtFormatRGBA_PVRTC_4Bpp_SRGB,
        kUnityRenderingExtFormatRGBA_PVRTC_4Bpp_UNorm,

        kUnityRenderingExtFormatRGB_ETC_UNorm,
        kUnityRenderingExtFormatRGB_ETC2_SRGB,
        kUnityRenderingExtFormatRGB_ETC2_UNorm,
        kUnityRenderingExtFormatRGB_A1_ETC2_SRGB,
        kUnityRenderingExtFormatRGB_A1_ETC2_UNorm,
        kUnityRenderingExtFormatRGBA_ETC2_SRGB,
        kUnityRenderingExtFormatRGBA_ETC2_UNorm,

        kUnityRenderingExtFormatR_EAC_UNorm,
        kUnityRenderingExtFormatR_EAC_SNorm,
        kUnityRenderingExtFormatRG_EAC_UNorm,
        kUnityRenderingExtFormatRG_EAC_SNorm,

        kUnityRenderingExtFormatRGBA_ASTC4X4_SRGB,
        kUnityRenderingExtFormatRGBA_ASTC4X4_UNorm,
        kUnityRenderingExtFormatRGBA_ASTC5X5_SRGB,
        kUnityRenderingExtFormatRGBA_ASTC5X5_UNorm,
        kUnityRenderingExtFormatRGBA_ASTC6X6_SRGB,
        kUnityRenderingExtFormatRGBA_ASTC6X6_UNorm,
        kUnityRenderingExtFormatRGBA_ASTC8X8_SRGB,
        kUnityRenderingExtFormatRGBA_ASTC8X8_UNorm,
        kUnityRenderingExtFormatRGBA_ASTC10X10_SRGB,
        kUnityRenderingExtFormatRGBA_ASTC10X10_UNorm,
        kUnityRenderingExtFormatRGBA_ASTC12X12_SRGB,
        kUnityRenderingExtFormatRGBA_ASTC12X12_UNorm,

        // Video formats
        kUnityRenderingExtFormatYUV2,

        // Automatic formats, back-end decides
        kUnityRenderingExtFormatDepthAuto,
        kUnityRenderingExtFormatShadowAuto,
        kUnityRenderingExtFormatVideoAuto,

        // ASTC hdr profile
        kUnityRenderingExtFormatRGBA_ASTC4X4_UFloat,
        kUnityRenderingExtFormatRGBA_ASTC5X5_UFloat,
        kUnityRenderingExtFormatRGBA_ASTC6X6_UFloat,
        kUnityRenderingExtFormatRGBA_ASTC8X8_UFloat,
        kUnityRenderingExtFormatRGBA_ASTC10X10_UFloat,
        kUnityRenderingExtFormatRGBA_ASTC12X12_UFloat,
    }

    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub(crate) enum UnityGfxRenderer {
        //kUnityGfxRendererOpenGL            =  0, // Legacy OpenGL, removed
        //kUnityGfxRendererD3D9              =  1, // Direct3D 9, removed
        kUnityGfxRendererD3D11 = 2,       // Direct3D 11
        kUnityGfxRendererNull = 4,        // "null" device (used in batch mode)
        kUnityGfxRendererOpenGLES20 = 8,  // OpenGL ES 2.0
        kUnityGfxRendererOpenGLES30 = 11, // OpenGL ES 3.0
        //kUnityGfxRendererGXM               = 12, // PlayStation Vita, removed
        kUnityGfxRendererPS4 = 13,                // PlayStation 4
        kUnityGfxRendererXboxOne = 14,            // Xbox One
        kUnityGfxRendererMetal = 16,              // iOS Metal
        kUnityGfxRendererOpenGLCore = 17,         // OpenGL core
        kUnityGfxRendererD3D12 = 18,              // Direct3D 12
        kUnityGfxRendererVulkan = 21,             // Vulkan
        kUnityGfxRendererNvn = 22,                // Nintendo Switch NVN API
        kUnityGfxRendererXboxOneD3D12 = 23,       // MS XboxOne Direct3D 12
        kUnityGfxRendererGameCoreXboxOne = 24,    // GameCore Xbox One
        kUnityGfxRendererGameCoreXboxSeries = 25, // GameCore XboxSeries
        kUnityGfxRendererPS5 = 26,                // PS5
        kUnityGfxRendererPS5NGGC = 27,            // PS5 NGGC
    }

    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub(crate) enum UnityGfxDeviceEventType {
        kUnityGfxDeviceEventInitialize = 0,
        kUnityGfxDeviceEventShutdown = 1,
        kUnityGfxDeviceEventBeforeReset = 2,
        kUnityGfxDeviceEventAfterReset = 3,
    }

    unsafe extern "C++" {
        include!("Unity/IUnityInterface.h");
        pub(crate) type IUnityInterfaces;
        pub(crate) type IUnityInterface;

        include!("Unity/IUnityGraphics.h");
        pub(crate) type IUnityGraphics;
        pub(crate) type UnityGfxRenderer;
        pub(crate) type UnityGfxDeviceEventType;

        include!("Unity/IUnityRenderingExtensions.h");
        pub(crate) type UnityRenderingExtTextureFormat;
        pub(crate) type UnityRenderingExtEventType;
        pub(crate) type UnityRenderingExtTextureUpdateParamsV2;

        include!("unity_cxx_wrapper.h");

        type UnityGraphicsDeviceEventCallback = super::UnityGraphicsDeviceEventCallback;

        // IUnityInterface.h wrapper methods
        pub(crate) unsafe fn IUnityInterfaces_GetInterface(
            interfaces: *const IUnityInterfaces,
            high: u64,
            low: u64,
        ) -> UniquePtr<IUnityInterface>;
        pub(crate) unsafe fn IUnityInterfaces_RegisterInterface(
            interfaces: *const IUnityInterfaces,
            high: u64,
            low: u64,
            interface: *mut IUnityInterface,
        );

        // IUnityRenderingExtensions.h wrapper methods
        pub(crate) unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_texData(
            data: *const UnityRenderingExtTextureUpdateParamsV2,
        ) -> *mut u32;
        pub(crate) unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_userData(
            data: *const UnityRenderingExtTextureUpdateParamsV2,
        ) -> u32;
        pub(crate) unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_textureId(
            data: *const UnityRenderingExtTextureUpdateParamsV2,
        ) -> *const i32;
        pub(crate) unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_format(
            data: *const UnityRenderingExtTextureUpdateParamsV2,
        ) -> UnityRenderingExtTextureFormat;
        pub(crate) unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_width(
            data: *const UnityRenderingExtTextureUpdateParamsV2,
        ) -> u32;
        pub(crate) unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_height(
            data: *const UnityRenderingExtTextureUpdateParamsV2,
        ) -> u32;
        pub(crate) unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_bpp(
            data: *const UnityRenderingExtTextureUpdateParamsV2,
        ) -> u32;

        // IUnityGraphics.h
        pub(crate) unsafe fn IUnityGraphics_GetRenderer(
            graphics: *const IUnityGraphics,
        ) -> UnityGfxRenderer;
        pub(crate) unsafe fn IUnityGraphics_RegisterDeviceEventCallback(
            graphics: *const IUnityGraphics,
            callback: UnityGraphicsDeviceEventCallback,
        );
        pub(crate) unsafe fn IUnityGraphics_UnregisterDeviceEventCallback(
            graphics: *const IUnityGraphics,
            callback: UnityGraphicsDeviceEventCallback,
        );
        pub(crate) unsafe fn IUnityGraphics_ReserveEventIDRange(
            graphics: *const IUnityGraphics,
            count: i32,
        ) -> i32;
    }
}

#[repr(transparent)]
pub struct UnityGraphicsDeviceEventCallback(
    pub extern "stdcall" fn(event: UnityRenderingExtEventType),
);

unsafe impl ExternType for UnityGraphicsDeviceEventCallback {
    type Id = type_id!("UnityGraphicsDeviceEventCallback");
    type Kind = cxx::kind::Trivial;
}

impl std::convert::Into<i32> for UnityRenderingExtEventType {
    fn into(self) -> i32 {
        use ffi::UnityRenderingExtEventType;
        match self {
            UnityRenderingExtEventType::kUnityRenderingExtEventSetStereoTarget => 0,
            UnityRenderingExtEventType::kUnityRenderingExtEventSetStereoEye => 1,
            UnityRenderingExtEventType::kUnityRenderingExtEventStereoRenderingDone => 2,
            UnityRenderingExtEventType::kUnityRenderingExtEventBeforeDrawCall => 3,
            UnityRenderingExtEventType::kUnityRenderingExtEventAfterDrawCall => 4,
            UnityRenderingExtEventType::kUnityRenderingExtEventCustomGrab => 5,
            UnityRenderingExtEventType::kUnityRenderingExtEventCustomBlit => 6,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureBegin => 7,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureEnd => 8,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureBeginV2 => 9,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureEndV2 => 10,
            UnityRenderingExtEventType::kUnityRenderingExtEventCount => 11,
            _ => -1,
        }
    }
}

pub extern "stdcall" fn UnityPluginLoad(interfaces: *mut ffi::IUnityInterfaces) {
    let mut lib = Unity::new(interfaces);
    unsafe {
        __unity_rs_entry_point(&mut lib);
    }
}
