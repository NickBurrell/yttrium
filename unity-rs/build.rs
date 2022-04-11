fn main() {
    let _build = cxx_build::bridges(vec!["src/native/mod.rs"])
        .flag_if_supported("--std=c++17") /* Clang and G++ flag */
        .flag_if_supported("/std:c++17") /* MSVC Flag */
        .include("ffi/cpp/include")
        .files(vec![
            "ffi/cpp/src/unity_shim.cpp",
            "ffi/cpp/src/event_shim.cpp",
        ])
        .compile("yttrium");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
