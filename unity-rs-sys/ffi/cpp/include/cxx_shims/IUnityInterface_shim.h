//
// Created by Nicholas Burrell on 4/11/2022.
//

#ifndef YTTRIUM_IUNITYINTERFACE_SHIM_H
#define YTTRIUM_IUNITYINTERFACE_SHIM_H

#include <cstdint>

#include "../Unity/IUnityInterface.h"

#include <memory>
[[maybe_unused]] std::unique_ptr<IUnityInterface>
IUnityInterfaces_GetInterface(const IUnityInterfaces*, uint64_t, uint64_t);
[[maybe_unused]] void IUnityInterfaces_RegisterInterface(const IUnityInterfaces*, uint64_t, uint64_t, IUnityInterface*);

#endif //YTTRIUM_IUNITYINTERFACE_SHIM_H
