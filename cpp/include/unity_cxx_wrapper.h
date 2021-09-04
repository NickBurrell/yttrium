#ifndef YTTRIUM_UNITY_CXX_WRAPPER_H
#define YTTRIUM_UNITY_CXX_WRAPPER_H

#include <cstdint>

#include <Unity/IUnityRenderingExtensions.h>
#include <Unity/IUnityInterface.h>
#include <Unity/IUnityGraphics.h>

namespace detail::init {
    void _register_interfaces(IUnityInterfaces *);
}

[[maybe_unused]] uint32_t*                      UnityRenderingExtTextureUpdateParamsV2_get_texData(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_userData(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] const int32_t*                 UnityRenderingExtTextureUpdateParamsV2_get_textureId(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] UnityRenderingExtTextureFormat UnityRenderingExtTextureUpdateParamsV2_get_format(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_width(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_height(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_bpp(const UnityRenderingExtTextureUpdateParamsV2*);

#endif //YTTRIUM_UNITY_CXX_WRAPPER_H
