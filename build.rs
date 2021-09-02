fn main() {
    let _build = cxx_build::bridges(vec!["src/plugin/mod.rs"])
        .flag_if_supported("--std=c++17") /* Clang and G++ flag */
        .flag_if_supported("/std:c++17") /* MSVC Flag */
        .includes(vec!["cpp/lib", "cpp/include"])
        .file("cpp/src/unity_cxx_wrapper.cpp")
        .compile("yttrium");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
