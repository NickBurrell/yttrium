# Overview
Currently, data ownership is handled by a single, overarching object, where references to data (mutable or otherwise) is shared as the need arises. This works well on paper, but given that after library initialization, the reference to the library is absent, we need a way to properly handle access after.

## Preliminary Idea
Since startup is initialized Unity-side, it makes sense to have some management `MonoBehavior` own the data. In effect, this amounts to an opaque pointer type passed into the `extern "C"` methods exported by Yttrium. While this does solve the issue of the reference disappearing, another issue presents itself, which is multiple references. Multi-threading would require much more overhead, as well as such considerations as who handles syncronization: Rust or C#?

## Alternative Strategy
The simplest alternative is a `lazy_static`, and a getter method for the global instance. This allows us to handle syncronization in an easy way; rather than storing the object itself in a static object, we store an `Arc<Mutex>` of it, allowing proper parallelism. This approach has the same drawbacks as any other static variable, but it simplifies several key issues from the above.