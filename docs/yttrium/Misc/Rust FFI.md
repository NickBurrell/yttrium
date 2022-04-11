# `cxx` FFI Cheat Sheet 
## Callbacks
### Rust Callbacks
Rust callbacks are handled primarily by `cxx`. More specifically, there is a type provided for us, with documentation found [here](https://cxx.rs/binding/fn.html). Included below is an overview of the callback type. 
```cpp

template<typename Ret, typename... Args>
class Fn<Ret(Args...)> final {
public:
	Ret operator()(Args... args) const noexcept;
	Fn Operator*() const noexcept;
}
```
