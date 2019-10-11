// This is a wrapper around the stock `duk_config.h` (renamed
// `duk_config_default.h` in this project) that allows for users of this crate
// to modify various configuration flags at compile-time (by means of Cargo
// features).

#if !defined(CUSTOM_DUK_CONFIG_H_INCLUDED)
#define CUSTOM_DUK_CONFIG_H_INCLUDED

#include "duk_config_default.h"

// Setting this flag tells Duktape to always compile using Win32's
// `GetSystemTime` instead of `GetSystemTimePreciseAsFileTime`. The latter API
// is only available after Windows 7. Unfortunately we have to sacrifice
// sub-millisecond timer resolution, but it's worth it for the sake of a simple
// interface and increased compatibility.
#if defined(DUK_F_WINDOWS)
#define DUK_USE_DATE_NOW_WINDOWS
#endif

#ifdef RUST_DUK_USE_EXEC_TIMEOUT_CHECK
#define DUK_USE_INTERRUPT_COUNTER
#define DUK_USE_EXEC_TIMEOUT_CHECK(udata) \
  (ducc_get_exec_timeout_function())((udata))
typedef duk_bool_t (*ducc_exec_timeout_function)(void *udata);
void ducc_set_exec_timeout_function(ducc_exec_timeout_function func);
ducc_exec_timeout_function ducc_get_exec_timeout_function();
#endif

#endif // CUSTOM_DUK_CONFIG_H_INCLUDED
