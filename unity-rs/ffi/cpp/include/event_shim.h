#ifndef YTTRIUM_EVENT_SHIM_H
#define YTTRIUM_EVENT_SHIM_H

#include <mutex>
#include <thread>
#include <shared_mutex>
#include <memory>
#include <map>

#include <Unity/IUnityEventQueue.h>
#include <cxx.h>

class EventShim {
public:

private:
    class EventDispatcher {
        static std::shared_ptr<std::map<uint64_t, rust::Fn<void()>>> handler;
    };

};

#endif /* YTTRIUM_EVENT_SHIM_H_ */
