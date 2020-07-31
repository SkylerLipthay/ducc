extern crate bindgen;

fn main() {
    let mut builder = bindgen::Builder::default()
        .header("duktape/wrapper.h")
        .clang_arg("-Iduktape")
        .clang_arg("-std=c99");

    if let Ok(sdk_path) = std::env::var("DUCC_SYSTEM_SDK_PATH") {
        builder = builder.clang_args(&["-isysroot", &sdk_path]);
    }

    builder.trust_clang_mangling(false)
        .rust_target(bindgen::RustTarget::Stable_1_25)
        .whitelist_type("^(?:rust_)?duk_.*")
        .whitelist_function("^(?:rust_)?duk_.*")
        .whitelist_function("^(?:rust_)?ducc_.*")
        .whitelist_var("^DUK_.*")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("failed to write bindings");
}
