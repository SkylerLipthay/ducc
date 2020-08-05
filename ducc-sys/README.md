# ducc-sys

Low-level Duktape FFI for the `ducc` crate.

## Helper extensions

This FFI exposes all of the stock Duktape items (all prefixed with `duk_` or
`DUK_`). In addition to these items, the FFI also offers helper extensions
specifically for dealing with Duktape from Rust. These are all prefixed with
`ducc_` and are listed below:

### `ducc_push_c_function_nothrow`

Like `duk_push_c_function`, but negative values from `func` are handled
differently. Instead of being able to return `DUK_RET_xxx`, `func` can return
`-1` (all negatively values are handled the same currently) to have an error
object pushed to the top of the stack be thrown.

When `func` returns a non-negative integer, this function is handled equivalent
to how it is handled with `duk_push_c_function`.

This function assigns a hidden property named `"__NOTHROWFUNC"` on the newly
created function (`DUK_HIDDEN_SYMBOL("__NOTHROWFUNC")`).

### `ducc_set_exec_timeout_function`

Sets the global timeout callback. This should be set only once per application,
as it is shared between all contexts. See `DUK_USE_EXEC_TIMEOUT_CHECK` for more
information on how this callback should function.

Execution timeout are only enabled if the `use-exec-timeout-check` Cargo feature
is set.

### `ducc_exec_timeout_function`

The callback type for `duk_set_exec_timeout_function`.

## Upgrading Duktape

1. Download a release from https://duktape.org/download.html
2. Copy files from `src/` to `ducc-sys/duktape/` (except `duk_config.h`, which should be copied to `duk_config_default.h`)
3. Generate FFI bindings: `cargo run --features="build-ffi-gen" --bin ffi-gen`
    - If on OS X and having trouble (e.g. missing header files), try setting `DUCC_SYSTEM_SDK_PATH=$(xcrun --sdk macosx --show-sdk-path)`