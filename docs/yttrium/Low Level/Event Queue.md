- Event Queue operates on classes/template specializations
	- More specifically, there is a pair of C macros, namely `REGISTER_EVENT_ID` and `REGISTER_EVENT_ID_WITH_CLEANUP`, which in do the following. Note that `TType` is an arbitrary type that we will use as our Event henceforth.
```cpp
namespace UnityEventQueue {
	template<> EventId GetEventId<TType>() { /* ... */}
}
```
 - There is currently no effective way to emulate this behavior using Rusts type system, nor `cxx`'s library, aside from manually name mangling, which is a bad idea for fairly clear reasons.
 - The only option is a secondary event layer on top of the Unity layer
	 - We create some type, hidden from the user, called `EventShim`. 
	 - Internally, `EventShim` holds the sole reference to a thread-safe singleton, holding a set of Rust callbacks mapped to some corresponding UUID (likely, some form of hash or GUID similar to Unity's)
	 - The biforcation of the EventQueue is suboptimal, as it makes debugging much more difficult.
		 - Suppose there is an error in a callback. We cannot simple check the event directly as we could in Unity, instead, we have an added layer of indirection.
		 - With this in mind, we need to ensure Rust-side, we have full unwind safety, as having the full unwound errors is pivotal for future debugging endeavors.
 - Need to investigate connections with [[Ownership Model#Event Callbacks]]. Since C++ needs to own the function pointers statically, it complicates ownership all the more.