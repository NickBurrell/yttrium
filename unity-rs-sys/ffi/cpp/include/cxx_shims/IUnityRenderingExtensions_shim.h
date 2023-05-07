#ifndef YTTRIUM_IUNITYRENDERINGEXTENSIONS_SHIM_H
#define YTTRIUM_IUNITYRENDERINGEXTENSIONS_SHIM_H

#include <cstdint>

#include "../Unity/IUnityRenderingExtensions.h"
#include "../Unity/IUnityInterface.h"
#include "../Unity/IUnityGraphics.h"

#include <memory>

using UnityGraphicsDeviceEventCallback = IUnityGraphicsDeviceEventCallback;

[[maybe_unused]] uint32_t*                      UnityRenderingExtTextureUpdateParamsV2_get_texData(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_userData(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] const int32_t*                 UnityRenderingExtTextureUpdateParamsV2_get_textureId(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] UnityRenderingExtTextureFormat UnityRenderingExtTextureUpdateParamsV2_get_format(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_width(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_height(const UnityRenderingExtTextureUpdateParamsV2*);
[[maybe_unused]] uint32_t                       UnityRenderingExtTextureUpdateParamsV2_get_bpp(const UnityRenderingExtTextureUpdateParamsV2*);

[[maybe_unused]] UnityGfxRenderer IUnityGraphics_GetRenderer(const IUnityGraphics*);
[[maybe_unused]] void IUnityGraphics_RegisterDeviceEventCallback(const IUnityGraphics*, IUnityGraphicsDeviceEventCallback);
[[maybe_unused]] void IUnityGraphics_UnregisterDeviceEventCallback(const IUnityGraphics*, IUnityGraphicsDeviceEventCallback);
[[maybe_unused]] int32_t IUnityGraphics_ReserveEventIDRange(const IUnityGraphics*, int32_t);


#endif //YTTRIUM_IUNITYRENDERINGEXTENSIONS_SHIM_H
