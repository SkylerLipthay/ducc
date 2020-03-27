use ducc::{Ducc, ExecSettings};
use ffi;
use value::Value;
use util::*;

#[test]
fn test_assert_stack() {
    let ducc = Ducc::new();
    unsafe {
        assert_stack!(ducc.ctx, 0, {
            ffi::duk_push_undefined(ducc.ctx);
            ffi::duk_pop(ducc.ctx);
        });

        assert_stack!(ducc.ctx, 2, {
            ffi::duk_push_undefined(ducc.ctx);
            ffi::duk_push_undefined(ducc.ctx);
        });

        ffi::duk_push_undefined(ducc.ctx);
        assert_stack!(ducc.ctx, -1, {
            ffi::duk_pop(ducc.ctx);
        });
    }
}

#[test]
#[should_panic]
fn test_assert_stack_panic() {
    let ducc = Ducc::new();
    unsafe {
        assert_stack!(ducc.ctx, 0, {
            ffi::duk_push_undefined(ducc.ctx);
        });
    }
}

#[test]
fn test_protect_duktape_closure() {
    let ducc = Ducc::new();
    unsafe {
        ffi::duk_push_int(ducc.ctx, 123);
        ffi::duk_push_int(ducc.ctx, 456);
        assert_stack!(ducc.ctx, -1, {
            let dummy_result = protect_duktape_closure(ducc.ctx, 2, 1, |ctx| {
                ffi::duk_concat(ctx, 2);
                789
            });
            assert_eq!(dummy_result.unwrap(), 789);
            assert!(ffi::duk_is_string(ducc.ctx, -1) != 0);
            assert_eq!(ffi::duk_get_length(ducc.ctx, -1), 6);
        });
    }
}

#[test]
fn test_protect_duktape_closure_err() {
    let ducc = Ducc::new();
    unsafe {
        assert_stack!(ducc.ctx, 0, {
            let result = protect_duktape_closure(ducc.ctx, 0, 2, |ctx| {
                ffi::duk_dup(ctx, 999);
            });
            assert!(result.is_err());
        });

        assert_stack!(ducc.ctx, 0, {
            let result = protect_duktape_closure(ducc.ctx, 0, 0, |ctx| {
                ffi::duk_dup(ctx, 999);
            });
            assert!(result.is_err());
        });
    }
}

#[test]
fn test_throw_non_object_error() {
    let ducc = Ducc::new();
    assert!(ducc.exec::<Value>("throw 'test'", None, ExecSettings::default()).is_err());
    assert!(ducc.exec::<Value>("throw 1", None, ExecSettings::default()).is_err());
    assert!(ducc.exec::<Value>("throw true", None, ExecSettings::default()).is_err());
    assert!(ducc.exec::<Value>("throw undefined", None, ExecSettings::default()).is_err());
    assert!(ducc.exec::<Value>("throw null", None, ExecSettings::default()).is_err());
}
