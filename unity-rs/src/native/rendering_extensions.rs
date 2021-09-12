use std::convert::{From};
use std::ptr::NonNull;

use super::ffi;

pub type TextureFormat = ffi::UnityRenderingExtTextureFormat;

impl From<NonNull<ffi::UnityRenderingExtTextureUpdateParamsV2>> for TextureUpdateParamsV2 {
    fn from(val: NonNull<ffi::UnityRenderingExtTextureUpdateParamsV2>) -> Self {
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
                _inner: val,
            }
        }
    }
}

impl From<TextureUpdateParamsV2> for *mut ffi::UnityRenderingExtTextureUpdateParamsV2 {
    fn from(val: TextureUpdateParamsV2) -> Self {
        val._inner
    }
}

#[allow(dead_code)]
pub struct TextureUpdateParamsV2 {
    texture: *mut u32,
    user_data: u32, /* Will likely go unused */
    id: *const i32,
    format: TextureFormat,
    width: u32,
    height: u32,
    bbp: u32,
    _inner: *mut ffi::UnityRenderingExtTextureUpdateParamsV2,
}
