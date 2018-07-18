use cesu8::{from_cesu8, to_cesu8};
use error::{Error, ErrorKind, Result, RuntimeErrorCode};
use ffi;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::{process, ptr, slice};
use std::sync::{Once, ONCE_INIT};
use std::time::{Duration, Instant};
use types::AnyMap;

// Throws an error if `$body` results in a change of `$ctx`'s stack size that isn't exactly equal to
// `$diff`. Must be used in an `unsafe` block.
macro_rules! assert_stack {
    ($ctx:expr, $diff:expr, $body:block) => {
        {
            let initial_stack_height = $crate::ffi::duk_get_top($ctx);
            let result = $body;
            assert_eq!(initial_stack_height + $diff, $crate::ffi::duk_get_top($ctx));
            result
        }
    }
}

// Returns a C-string representation of a `&'static str` to be passed into a Duktape function
// (automatically appends a nul byte).
macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0")
            as *const str
            as *const [::std::os::raw::c_char]
            as *const ::std::os::raw::c_char
    )
}

// Returns a C-string representation of a list of i8 to be passed into a Duktape function
// (automatically appends a nul byte).
macro_rules! i8str {
    ($($b:expr),*) => ([$($b as i8),*, 0])
}

// Returns a C-string representation of a list of i8 to be passed into a Duktape function
// (automatically prepends a `-1` byte and appends a nul byte).
macro_rules! hidden_i8str {
    ($($b:expr),*) => (i8str!(-1, $($b as i8),*))
}

// Call a function that calls into the Duktape API and may trigger a Duktape error (longjmp) in a
// safe way. Wraps the inner function in a call to `duk_safe_call`, so the inner function only has
// access to a limited Duktape stack. `num_args` and `num_returns` are similar to the parameters of
// `duk_safe_call` (`nargs` and `nrets` respectively), but the given function return type is not the
// return value count, instead the inner function return value count is assumed to match the
// `num_returns` param. Provided function must *not* panic, and since it will generally be
// `longjmp`ing, should not contain any values that implement `Drop`.
//
// If an `Err` is returned, the stack size remains unchanged. If `Ok` is returned, the stack size
// increases by `num_returns`.
//
// TODO: Should all FFI code be wrapped in `protect`? I've wrapped all FFI calls that seemed like
// they could `longjmp`, but the Duktape documentation isn't entirely clear. Perhaps it should be
// assumed that *any* FFI code will error.
pub(crate) unsafe fn protect_duktape_closure<F, R>(
    ctx: *mut ffi::duk_context,
    num_args: ffi::duk_idx_t,
    num_returns: ffi::duk_idx_t,
    function: F,
) -> Result<R>
where
    F: Fn(*mut ffi::duk_context) -> R,
{
    struct Params<F, R> {
        function: F,
        result: Option<R>,
        num_returns: ffi::duk_idx_t,
    }

    unsafe extern "C" fn do_call<F, R>(
        ctx: *mut ffi::duk_context,
        udata: *mut c_void,
    ) -> ffi::duk_ret_t
    where
        F: Fn(*mut ffi::duk_context) -> R,
    {
        let params = udata as *mut Params<F, R>;
        (*params).result = Some(((*params).function)(ctx));
        (*params).num_returns
    }

    let mut params = Params { function, result: None, num_returns };

    let result = ffi::duk_safe_call(
        ctx,
        Some(do_call::<F, R>),
        &mut params as *mut Params<F, R> as *mut c_void,
        num_args,
        // Must have at least one "superfluous" return value to be able to access a possible error
        // result:
        num_returns + 1
    );

    if result == 0 {
        // Get rid of the unused "superfluous" return value:
        ffi::duk_pop(ctx);
        Ok(params.result.unwrap())
    } else {
        // Remove any `undefined` "return" elements that act as padding:
        ffi::duk_pop_n(ctx, num_returns);
        // The first of our "return" elements (accounted for by the "superfluous" return value) is
        // our error:
        Err(pop_error(ctx))
    }
}

const ERROR_KEY: [i8; 7] = hidden_i8str!('e', 'r', 'r', 'o', 'r');

unsafe extern "C" fn error_finalizer(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t {
    ffi::duk_require_stack(ctx, 1);
    ffi::duk_get_prop_string(ctx, 0, ERROR_KEY.as_ptr());
    Box::from_raw(ffi::duk_get_pointer(ctx, -1) as *mut Error);
    ffi::duk_pop(ctx);
    ffi::duk_push_undefined(ctx);
    ffi::duk_put_prop_string(ctx, 0, ERROR_KEY.as_ptr());
    0
}

pub(crate) unsafe fn push_error(ctx: *mut ffi::duk_context, error: Error) {
    assert_stack!(ctx, 1, {
        let desc = error.into_runtime_error_desc();
        let cstr_msg = match desc.message {
            Some(ref msg) => match CString::new(to_cesu8(msg)) {
                Ok(msg) => Some(msg),
                Err(_) => None,
            },
            None => None,
        };
        let cstr_name = match CString::new(to_cesu8(&desc.name)) {
            Ok(name) => name,
            Err(_) => CString::new("Error").unwrap(),
        };

        ffi::duk_require_stack(ctx, 2);
        ffi::duk_push_error_object_raw(
            ctx,
            desc.code.to_duk_errcode(),
            // TODO: Line number and file name:
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        );

        ffi::duk_push_lstring(ctx, cstr_name.as_ptr(), cstr_name.as_bytes().len());
        ffi::duk_put_prop_string(ctx, -2, cstr!("name"));
        if let Some(cstr_msg) = cstr_msg {
            ffi::duk_push_lstring(ctx, cstr_msg.as_ptr(), cstr_msg.as_bytes().len());
            ffi::duk_put_prop_string(ctx, -2, cstr!("message"));
        }
        ffi::duk_push_pointer(ctx, Box::into_raw(desc.cause) as *mut _);
        ffi::duk_put_prop_string(ctx, -2, ERROR_KEY.as_ptr());
        ffi::duk_push_c_function(ctx, Some(error_finalizer), 1);
        ffi::duk_set_finalizer(ctx, -2);
    })
}

pub(crate) unsafe fn pop_error(ctx: *mut ffi::duk_context) -> Error {
    assert_stack!(ctx, -1, {
        ffi::duk_require_stack(ctx, 1);

        ffi::duk_get_prop_string(ctx, -1, ERROR_KEY.as_ptr());
        let error_ptr = ffi::duk_get_pointer(ctx, -1) as *mut Error;
        ffi::duk_pop(ctx);
        ffi::duk_push_undefined(ctx);
        ffi::duk_put_prop_string(ctx, -2, ERROR_KEY.as_ptr());
        ffi::duk_push_undefined(ctx);
        ffi::duk_set_finalizer(ctx, -2);
        if !error_ptr.is_null() {
            ffi::duk_pop(ctx);
            return *Box::from_raw(error_ptr);
        }

        let code = RuntimeErrorCode::from_duk_errcode(ffi::duk_get_error_code(ctx, -1));
        ffi::duk_get_prop_string(ctx, -1, cstr!("name"));
        let name = get_string(ctx, -1);
        ffi::duk_pop(ctx);
        ffi::duk_get_prop_string(ctx, -1, cstr!("message"));
        let message = get_string(ctx, -1);
        ffi::duk_pop(ctx);

        let name = match name.is_empty() {
            false => name,
            true => "Error".to_string(),
        };

        let message = match message.is_empty() {
            false => Some(message),
            true => None,
        };

        ffi::duk_pop(ctx);

        Error {
            kind: ErrorKind::RuntimeError { code, name },
            context: message,
        }
    })
}

pub(crate) unsafe fn push_bytes(ctx: *mut ffi::duk_context, value: &[u8]) -> Result<()> {
    assert_stack!(ctx, 1, {
        protect_duktape_closure(ctx, 0, 1, |ctx| {
            ffi::duk_require_stack(ctx, 1);
            let len = value.len();
            let data = ffi::duk_push_fixed_buffer(ctx, len);
            ptr::copy(value.as_ptr(), data as *mut u8, len);
        })
    })
}

// Converts a UTF-8 Rust string to a CESU-8 string and pushes it onto the Duktape stack. Returns an
// error if the conversion failed.
pub(crate) unsafe fn push_str(ctx: *mut ffi::duk_context, value: &str) -> Result<()> {
    let string = CString::new(to_cesu8(value))
        .map_err(|_| Error::to_js_conversion("&str", "string"))?;

    assert_stack!(ctx, 1, {
        protect_duktape_closure(ctx, 0, 1, |ctx| {
            ffi::duk_require_stack(ctx, 1);
            ffi::duk_push_lstring(ctx, string.as_ptr(), string.as_bytes().len());
        })
    })
}

// Returns the string value at the given stack index. If the value is not a string or failed to be
// converted, an empty `String` is returned.
unsafe fn get_string(ctx: *mut ffi::duk_context, idx: ffi::duk_idx_t) -> String {
    let mut len = 0;
    let string = ffi::duk_get_lstring_default(ctx, idx, &mut len, cstr!(""), 0);
    if string.is_null() {
        return String::new();
    }

    let bytes = slice::from_raw_parts(string as *const u8, len as usize);
    match from_cesu8(bytes) {
        Ok(string) => string.into_owned(),
        Err(_) => String::new(),
    }
}

const UDATA: [i8; 7] = hidden_i8str!('u', 'd', 'a', 't', 'a');
const ANYMAP: [i8; 8] = hidden_i8str!('a', 'n', 'y', 'm', 'a', 'p');

pub(crate) unsafe fn create_heap() -> *mut ffi::duk_context {
    if cfg!(feature = "timeout") {
        ensure_exec_timeout_check_exists();
    }

    let udata = Box::into_raw(Box::new(Udata { timeout: None }));
    let ctx = ffi::duk_create_heap(None, None, None, udata as *mut _, Some(fatal_handler));
    assert!(!ctx.is_null());

    ffi::duk_require_stack(ctx, 1);

    ffi::duk_push_pointer(ctx, udata as *mut _);
    ffi::duk_put_global_string(ctx, UDATA.as_ptr());

    let any_map = Box::into_raw(Box::new(AnyMap::new()));
    ffi::duk_push_pointer(ctx, any_map as *mut _);
    ffi::duk_put_global_string(ctx, ANYMAP.as_ptr());

    ffi::duk_push_global_object(ctx);
    ffi::duk_del_prop_string(ctx, -1, cstr!("Duktape"));
    ffi::duk_pop(ctx);

    ctx
}

pub(crate) unsafe fn get_udata(ctx: *mut ffi::duk_context) -> *mut Udata {
    let _sg = StackGuard::new(ctx);
    ffi::duk_require_stack(ctx, 1);
    ffi::duk_get_global_string(ctx, UDATA.as_ptr());
    ffi::duk_get_pointer(ctx, -1) as *mut Udata
}

pub(crate) unsafe fn get_any_map(ctx: *mut ffi::duk_context) -> *mut AnyMap {
    let _sg = StackGuard::new(ctx);
    ffi::duk_require_stack(ctx, 1);
    ffi::duk_get_global_string(ctx, ANYMAP.as_ptr());
    ffi::duk_get_pointer(ctx, -1) as *mut AnyMap
}

unsafe extern "C" fn fatal_handler(_udata: *mut c_void, msg: *const c_char) {
    let msg = from_cesu8(CStr::from_ptr(msg).to_bytes())
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| "failed to decode message".to_string());
    eprintln!("fatal error from duktape: {}", msg);
    // Unfortunately I don't think there's a clean way to unwind normally, so we'll have to
    // abort the entire process without destructing its threads.
    process::abort();
}

struct Timeout {
    start: Instant,
    duration: Duration,
}

pub(crate) struct Udata {
    timeout: Option<Timeout>,
}

impl Udata {
    pub fn set_timeout(&mut self, duration: Duration) {
        self.timeout = Some(Timeout {
            start: Instant::now(),
            duration,
        });
    }

    pub fn clear_timeout(&mut self) {
        self.timeout = None;
    }
}

// Unfortunately `ducc_set_exec_timeout_function` sets a global variable, so this applies to all
// `duk_context`s whether or not they specify a `Udata` pointer as their heap `udata`. This means
// that `timeout_func` will result in undefined behavior (likely a segmentation fault) if a
// `duk_context` is created outside of `Ducc`. Long story short: use `Ducc` and don't use
// `duk_create_heap` directly.
fn ensure_exec_timeout_check_exists() {
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| {
        unsafe { ffi::ducc_set_exec_timeout_function(Some(timeout_func)); }
    });
}

unsafe extern "C" fn timeout_func(udata: *mut c_void) -> ffi::duk_bool_t {
    let udata = udata as *mut Udata;
    assert!(!udata.is_null());

    if let Some(ref timeout) = (*udata).timeout {
        if timeout.start.elapsed() >= timeout.duration {
            return 1;
        }
    }

    return 0;
}

// Creates a `StackGuard` instance with a record of the stack size, and on drop will check the stack
// size and drop any extra elements. If the stack size at the end is *smaller* than at the
// beginning, this is considered a fatal logic error and will result in an abort.
pub(crate) struct StackGuard {
    ctx: *mut ffi::duk_context,
    top: ffi::duk_idx_t,
}

impl StackGuard {
    pub unsafe fn new(ctx: *mut ffi::duk_context) -> StackGuard {
        let top = ffi::duk_get_top(ctx);
        StackGuard { ctx, top }
    }
}

impl Drop for StackGuard {
    fn drop(&mut self) {
        let top = unsafe { ffi::duk_get_top(self.ctx) };

        if top > self.top {
            unsafe { ffi::duk_set_top(self.ctx, self.top); }
        } else if top < self.top {
            panic!("{} too many stack values popped", self.top - top);
        }
    }
}
