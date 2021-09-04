pub mod unity_interface;
pub mod unity_rendering_extensions;

#[cxx::bridge]
pub mod ffi {

    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub enum UnityRenderingExtEventType {
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
    pub enum UnityRenderingExtTextureFormat {
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

    extern "C++" {
        include!("Unity/IUnityInterface.h");
        pub type IUnityInterfaces;

        include!("Unity/IUnityRenderingExtensions.h");
        pub type UnityRenderingExtTextureFormat;
        pub type UnityRenderingExtEventType;
        pub type UnityRenderingExtTextureUpdateParamsV2;

        include!("unity_cxx_wrapper.h");
        pub unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_texData(data: *const UnityRenderingExtTextureUpdateParamsV2) -> *mut u32;
        pub unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_userData(data: *const UnityRenderingExtTextureUpdateParamsV2) -> u32;
        pub unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_textureId(data: *const UnityRenderingExtTextureUpdateParamsV2) -> *const i32;
        pub unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_format(data: *const UnityRenderingExtTextureUpdateParamsV2) -> UnityRenderingExtTextureFormat;
        pub unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_width(data: *const UnityRenderingExtTextureUpdateParamsV2) -> u32;
        pub unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_height(data: *const UnityRenderingExtTextureUpdateParamsV2) -> u32;
        pub unsafe fn UnityRenderingExtTextureUpdateParamsV2_get_bpp(data: *const UnityRenderingExtTextureUpdateParamsV2) -> u32;
    }
}