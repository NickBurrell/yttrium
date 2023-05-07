#include "../include/cxx_shims/IUnityRenderingExtensions_shim.h"

#include <cassert>

#include <mutex>
#include <memory>

#include <atomic>

static std::shared_ptr<IUnityInterfaces> _unity_interfaces = nullptr;
static std::shared_ptr<IUnityGraphics>   _unity_graphics   = nullptr;

static std::mutex _interfaces_mutex = std::mutex();
static std::mutex _graphics_mutex   = std::mutex();

static std::atomic_bool _has_init = false;

namespace detail::init {
    void _register_interfaces(IUnityInterfaces *unity_interfaces) {
        assert(!_has_init);
        std::lock(_interfaces_mutex, _graphics_mutex);
        _unity_interfaces = std::shared_ptr<IUnityInterfaces>
                (unity_interfaces);
        _unity_graphics = std::shared_ptr<IUnityGraphics>
                (unity_interfaces->Get<IUnityGraphics>());
        //_unity_graphics->RegisterDeviceEventCallback(_graphics_device_callback);
        _has_init = true;
    }
}

[[maybe_unused]] uint32_t* UnityRenderingExtTextureUpdateParamsV2_get_texData(const UnityRenderingExtTextureUpdateParamsV2* data) {
    return (uint32_t*) ((UnityRenderingExtTextureUpdateParamsV2*)data)->texData;
}

[[maybe_unused]] uint32_t UnityRenderingExtTextureUpdateParamsV2_get_userData(const UnityRenderingExtTextureUpdateParamsV2* data) {
    return ((UnityRenderingExtTextureUpdateParamsV2*)data)->userData;
}

[[maybe_unused]] const int32_t* UnityRenderingExtTextureUpdateParamsV2_get_textureId(const UnityRenderingExtTextureUpdateParamsV2* data) {
    return (int32_t*) ((UnityRenderingExtTextureUpdateParamsV2*)data)->textureID;
}

[[maybe_unused]] UnityRenderingExtTextureFormat UnityRenderingExtTextureUpdateParamsV2_get_format(const UnityRenderingExtTextureUpdateParamsV2* data) {
    return ((UnityRenderingExtTextureUpdateParamsV2*)data)->format;
}

[[maybe_unused]] uint32_t UnityRenderingExtTextureUpdateParamsV2_get_width(const UnityRenderingExtTextureUpdateParamsV2* data) {
    return ((UnityRenderingExtTextureUpdateParamsV2*)data)->width;
}

[[maybe_unused]] uint32_t UnityRenderingExtTextureUpdateParamsV2_get_height(const UnityRenderingExtTextureUpdateParamsV2* data) {
    return ((UnityRenderingExtTextureUpdateParamsV2*)data)->height;
}

[[maybe_unused]] uint32_t UnityRenderingExtTextureUpdateParamsV2_get_bpp(const UnityRenderingExtTextureUpdateParamsV2* data) {
    return ((UnityRenderingExtTextureUpdateParamsV2*)data)->bpp;
}

[[maybe_unused]] UnityGfxRenderer IUnityGraphics_GetRenderer(const IUnityGraphics* g) {
    return g->GetRenderer();
}

void IUnityGraphics_RegisterDeviceEventCallback(const IUnityGraphics* g, IUnityGraphicsDeviceEventCallback c) {
    g->RegisterDeviceEventCallback(c);
}
void IUnityGraphics_UnregisterDeviceEventCallback(const IUnityGraphics* g, IUnityGraphicsDeviceEventCallback c) {
    g->UnregisterDeviceEventCallback(c);
}

[[maybe_unused]] int32_t IUnityGraphics_ReserveEventIDRange(const IUnityGraphics* g, int32_t c) {
    return g->ReserveEventIDRange(c);
}