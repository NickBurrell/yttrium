fn main() {
    let _build = cxx_build::bridges(["src/lib.rs"])
        .flag_if_supported("--std=c++17") /* Clang and G++ flag */
        .flag_if_supported("/std:c++17") /* MSVC Flag */
        .includes([
                "cpp/lib",
                "cpp/include"
            ])
        .file("cpp/src/yttrium.cpp")
        .compile("yttrium");
    println!("cargo:rerun-if-changed=src/lib.rs");
}