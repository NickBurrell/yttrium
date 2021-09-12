fn main() {
    let _build = cxx_build::bridges(vec!["src/native/mod.rs"])
        .flag_if_supported("--std=c++17") /* Clang and G++ flag */
        .flag_if_supported("/std:c++17") /* MSVC Flag */
        .includes(vec!["../lib/include", "../wrapper/cpp/include"])
        .file("../wrapper/cpp/src/cxx_wrapper.cpp")
        .compile("yttrium");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
