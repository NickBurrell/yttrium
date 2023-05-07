//
// Created by Nicholas Burrell on 4/11/2022.
//

#include "../include/cxx_shims/IUnityInterface_shim.h"

[[maybe_unused]] UnityInterfaceGUID UnityInterfaceGUID_new(uint64_t high, uint64_t low) {
    return UnityInterfaceGUID(high, low);
}

[[maybe_unused]] std::unique_ptr<IUnityInterface> IUnityInterfaces_GetInterface(const IUnityInterfaces* i, uint64_t h, uint64_t l) {
    return std::unique_ptr<IUnityInterface>(i->GetInterfaceSplit(h,l));
}

[[maybe_unused]] void IUnityInterfaces_RegisterInterface(const IUnityInterfaces* i, uint64_t h, uint64_t l, IUnityInterface* r) {
    return i->RegisterInterfaceSplit(h,l,r);
}
