#include "../include/yttrium.h"

#include <mutex>
#include <memory>

#include <atomic>

#include "yttrium.h"

std::shared_ptr<IUnityInterfaces> _unity_interfaces = nullptr;
std::shared_ptr<IUnityGraphics>   _unity_graphics   = nullptr;

std::mutex _interfaces_mutex = std::mutex();
std::mutex _graphics_mutex   = std::mutex();

static std::atomic_bool _has_init = false;

namespace detail::init {
    void _register_interfaces(IUnityInterfaces *unity_interfaces) {
        if(_has_init)
            throw std::exception("'_register_interfaces' has already run.");
        std::lock(_interfaces_mutex, _graphics_mutex);
        _unity_interfaces = std::shared_ptr<IUnityInterfaces>
                (unity_interfaces);
        _unity_graphics   = std::shared_ptr<IUnityGraphics>
                (unity_interfaces->Get<IUnityGraphics>());

    }
}