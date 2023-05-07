fn main() {
    let _build = cxx_build::bridges(vec!["src/lib.rs", "src/gfx/mod.rs"])
        .flag_if_supported("--std=c++17") /* Clang and G++ flag */
        .flag_if_supported("/std:c++17") /* MSVC Flag */
        .include("ffi/cpp/include")
        .include("ffi/cpp/include/cxx_shims")
        .files(vec![
            "ffi/cpp/src/IUnityInterface_shim.cpp",
            "ffi/cpp/src/IUnityEventQueue_shim.cpp",
            "ffi/cpp/src/IUnityRenderingExtensions_shim.cpp",
        ])
        .compile("yttrium");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
