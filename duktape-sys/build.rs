extern crate cc;

fn main() {
    let mut builder = cc::Build::new();

    builder.include("duktape")
        .flag("-std=c99")
        .file("duktape/duktape.c")
        .file("duktape/wrapper.c");

    if cfg!(feature = "use-exec-timeout-check") {
        builder.define("RUST_DUK_USE_EXEC_TIMEOUT_CHECK", None);
    }

    builder.compile("libduktape.a");
}
