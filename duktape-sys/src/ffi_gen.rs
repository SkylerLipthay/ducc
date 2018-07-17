extern crate bindgen;

fn main() {
    bindgen::Builder::default()
        .header("duktape/wrapper.h")
        .clang_arg("-Iduktape")
        .clang_arg("-std=c99")
        .rust_target(bindgen::RustTarget::Stable_1_25)
        .whitelist_type("^(?:rust_)?duk_.*")
        .whitelist_function("^(?:rust_)?duk_.*")
        .whitelist_var("^DUK_.*")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("failed to write bindings");
}
