use ducc::Ducc;
use error::{Error, Result};
use function::{Function, Invocation};
use value::Value;

#[test]
fn js_function() {
    let ducc = Ducc::new();
    let value: bool = ducc.compile("true", None).unwrap().call(()).unwrap();
    assert_eq!(true, value);
    let func: Value = ducc.compile("(function(x, y) { return x + y; })", None).unwrap()
        .call(()).unwrap();
    assert!(func.is_function());
    let func = if let Value::Function(f) = func { f } else { unreachable!(); };
    let value: f64 = func.call((1, 2)).unwrap();
    assert_eq!(3.0f64, value);
}

#[test]
fn rust_function() {
    fn add(inv: Invocation) -> Result<usize> {
        let (a, b): (usize, usize) = inv.args.into(inv.ducc)?;
        return Ok(a + b);
    }

    let ducc = Ducc::new();
    let func = ducc.create_function(add);
    let value: f64 = func.call((1, 2)).unwrap();
    assert_eq!(3.0f64, value);

    ducc.globals().set("add", func).unwrap();
    let value: f64 = ducc.exec("add(4, 5)", None, None).unwrap();
    assert_eq!(9.0f64, value);
}

#[test]
fn rust_function_error() {
    fn add(_inv: Invocation) -> Result<usize> {
        Err(Error::RecursiveMutCallback) // TODO: Use ExternalError
    }

    let ducc = Ducc::new();
    let func = ducc.create_function(add);
    assert!(func.call::<_, usize>((1, 2)).is_err());
}

#[test]
fn rust_closure() {
    let ducc = Ducc::new();
    let func = ducc.create_function(|inv| {
        let (a, b): (usize, usize) = inv.args.into(inv.ducc)?;
        Ok(a + b)
    });
    let value: f64 = func.call((1, 2)).unwrap();
    assert_eq!(3.0f64, value);
}

#[test]
fn double_drop_rust_function() {
    let ducc = Ducc::new();
    let func = ducc.create_function(|_| Ok(()));
    let _func_dup = func.clone();
    // The underlying boxed closure is only dropped once, by means of a Duktape finalizer.
}

#[test]
fn return_unit() {
    let ducc = Ducc::new();
    let func = ducc.create_function(|_| Ok(()));
    let _: () = func.call(()).unwrap();
    let _: () = func.call((123,)).unwrap();
    let number_cast: usize = func.call(()).unwrap();
    assert_eq!(number_cast, 0);
}

#[test]
fn rust_closure_mut_callback_error() {
    let ducc = Ducc::new();

    let mut v = Some(Box::new(123));
    let f = ducc.create_function_mut(move |inv| {
        let ducc = inv.ducc;
        let (mutate,) = inv.args.into(ducc)?;
        if mutate {
            v = None;
        } else {
            // Produce a mutable reference:
            let r = v.as_mut().unwrap();
            // Whoops, this will recurse into the function and produce another mutable reference!
            ducc.globals().get::<_, Function>("f")?.call((true,))?;
            println!("Should not get here, mutable aliasing has occurred!");
            println!("value at {:p}", r as *mut _);
            println!("value is {}", r);
        }

        Ok(())
    });

    ducc.globals().set("f", f).unwrap();
    match ducc.globals().get::<_, Function>("f").unwrap().call::<_, ()>((false,)) {
        // TODO:
        // Err(Error::RecursiveMutCallback) => { },
        Err(Error::RuntimeError { .. }) => { },
        other => panic!("incorrect result: {:?}", other),
    };
}

#[test]
fn number_this() {
    fn add(inv: Invocation) -> Result<usize> {
        let this: usize = inv.this.into(inv.ducc)?;
        let (acc,): (usize,) = inv.args.into(inv.ducc)?;
        return Ok(this + acc);
    }

    let ducc = Ducc::new();
    let func = ducc.create_function(add);

    let value: f64 = func.call_method(10, (20,)).unwrap();
    assert_eq!(30.0f64, value);
    let value: f64 = func.call((1,)).unwrap();
    assert_eq!(1.0f64, value);

    ducc.globals().set("add", func).unwrap();
    let value: f64 = ducc.exec("add.call(12, 13)", None, None).unwrap();
    assert_eq!(25.0f64, value);
    let value: f64 = ducc.exec("add(5)", None, None).unwrap();
    assert_eq!(5.0f64, value);
}
