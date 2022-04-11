# Overview
Currently, data ownership is handled by a single, overarching object, where references to data (mutable or otherwise) is shared as the need arises. This works well on paper, but given that after library initialization, the reference to the library is absent, we need a way to properly handle access after.

## Preliminary Idea
The simplest alternative is a `lazy_static`, and a getter method for the global instance. This allows us to handle synchronization in an easy way; rather than storing the object itself in a static object, we store an `Arc<Mutex>` of it, allowing proper parallelism. This approach has the same drawbacks as any other static variable, but it simplifies several key issues from the above.
## Alternative Strategy
Since startup is initialized Unity-side, it makes sense to have some management `MonoBehavior` own the data. In effect, this amounts to an opaque pointer type passed into the `extern "C"` methods exported by Yttrium. While this does solve the issue of the reference disappearing, another issue presents itself, which is multiple references. Multi-threading would require much more overhead, as well as such considerations as who handles synchronization: Rust or C#?

## Additional Considerations
Several key design choices from Unity force us to make difficult concessions on strict ownership. The first one that arose is Graphics, although, graphics is an entirely separate area of concern, as the resources are merely passed to use temporarily. While this is not too dissimilar to several other API features (even the `IUnityInterfaces` type works this way), Rust's relationship with graphics makes this all the more tumultuous.

### Event Callbacks
As referenced by [[Event Queue]], Events present a small issue. Since we cannot make new event types from Rust that are compatible with C++ without much issue, we need to create our own event layer on top of Unity's. This alone presents numerous concerns, but foremost for us is ownership of data. Ideally, we structure it as follows:
- `EventShim` class, representing a single, unique event type for our entire application. 
	- We will likely add more event types for specific library concerns, such as startup and cleanup, but those will not be addressed here.
- To prevent unnecessary crossing of the FFI-boundary, we store all callbacks in Rust. This has the added benefit of flexibility for handling.
	- Namely, we wish to allow users to have both static functional callbacks as well as stateful `struct`-based callbacks. This means that internally, our map must differentiate between the two, which can be handled using the `Result` type.