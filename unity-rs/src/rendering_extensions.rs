use std::convert::{From, TryFrom};
use std::ptr::NotNull;

use super::ffi;

pub type TextureFormat = ffi::UnityRenderingExtTextureFormat;

impl From<NotNull<ffi::UnityRenderingExtTextureUpdateParamsV2>> for TextureUpdateParamsV2 {
    fn from(val: NotNull<ffi::UnityRenderingExtTextureUpdateParamsV2>) -> Self {
        let val = val.as_ptr();
        unsafe {
            TextureUpdateParamsV2 {
                texture: ffi::UnityRenderingExtTextureUpdateParamsV2_get_texData(val),
                user_data: ffi::UnityRenderingExtTextureUpdateParamsV2_get_userData(val),
                id: ffi::UnityRenderingExtTextureUpdateParamsV2_get_textureId(val),
                format: ffi::UnityRenderingExtTextureUpdateParamsV2_get_format(val),
                width: ffi::UnityRenderingExtTextureUpdateParamsV2_get_width(val),
                height: ffi::UnityRenderingExtTextureUpdateParamsV2_get_height(val),
                bbp: ffi::UnityRenderingExtTextureUpdateParamsV2_get_bpp(val),
            }
        }
    }
}

pub struct TextureUpdateParamsV2 {
    texture: *mut u32,
    user_data: u32, /* Will likely go unused */
    id: *const i32,
    format: TextureFormat,
    width: u32,
    height: u32,
    bbp: u32,
}
