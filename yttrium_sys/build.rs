fn main() {
    let _build = cxx_build::bridge("src/lib.rs")
        .file("cxx/src/unity_cxx_wrapper.cpp");

    println!("cargo:rerun-if-changed=src/lib.rs");
}