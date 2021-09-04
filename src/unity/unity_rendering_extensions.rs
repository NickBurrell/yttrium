use std::convert::{Into, TryInto};

use super::ffi;

pub type TextureFormat = ffi::UnityRenderingExtTextureFormat;

impl TryInto<TextureUpdateParamsV2> for *const ffi::UnityRenderingExtTextureUpdateParamsV2 {
    type Error = ();

    fn try_into(self) -> Result<TextureUpdateParamsV2, Self::Error> {
        unsafe {
            if !self.is_null() {
                return Ok(
                    TextureUpdateParamsV2 {
                        texture: ffi::UnityRenderingExtTextureUpdateParamsV2_get_texData(self),
                        user_data: ffi::UnityRenderingExtTextureUpdateParamsV2_get_userData(self),
                        id: ffi::UnityRenderingExtTextureUpdateParamsV2_get_textureId(self),
                        format: ffi::UnityRenderingExtTextureUpdateParamsV2_get_format(self),
                        width: ffi::UnityRenderingExtTextureUpdateParamsV2_get_width(self),
                        height: ffi::UnityRenderingExtTextureUpdateParamsV2_get_height(self),
                        bbp: ffi::UnityRenderingExtTextureUpdateParamsV2_get_bpp(self)
                    }
                )
            }
            Err(())
        }
    }
}

impl Into<TextureUpdateParamsV2> for ffi::UnityRenderingExtTextureUpdateParamsV2 {
    fn into(self) -> TextureUpdateParamsV2 {
        ((&self) as *const Self).try_into().unwrap()
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

