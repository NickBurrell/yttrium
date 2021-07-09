#ifndef YTTRIUM_UNITY_CXX_WRAPPER_H
#define YTTRIUM_UNITY_CXX_WRAPPER_H

#include <Unity/IUnityInterface.h>
#include <Unity/IUnityGraphics.h>

namespace detail::init {
    void _register_interfaces(IUnityInterfaces *);
}

#endif //YTTRIUM_UNITY_CXX_WRAPPER_H
