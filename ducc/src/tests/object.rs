use ducc::Ducc;
use error::Result;
use function::Invocation;
use object::{Object, PropertyDescriptor};
use value::Value;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

#[test]
fn contains_key() {
    let ducc = Ducc::new();
    let globals = ducc.globals();
    assert!(globals.contains_key("Array").unwrap());
    assert!(!globals.contains_key("~NOT-EXIST~").unwrap());
}

#[test]
fn len() {
    let ducc = Ducc::new();
    let object = ducc.create_object();
    assert_eq!(object.len().unwrap(), 0);
}

#[test]
fn set_get() {
    let ducc = Ducc::new();

    let object = ducc.create_object();
    object.set("a", 123).unwrap();
    object.set(123, "a").unwrap();
    let parent = ducc.create_object();
    parent.set("obj", object).unwrap();
    let object: Object = parent.get("obj").unwrap();
    assert_eq!(object.get::<_, i8>("a").unwrap(), 123);
    assert_eq!(object.get::<_, String>("a").unwrap(), "123");
    assert_eq!(object.get::<_, String>("123").unwrap(), "a");
    assert_eq!(object.get::<_, String>(123).unwrap(), "a");
}

#[test]
fn define_prop() {
    let mut ducc = Ducc::new();
    let object = ducc.create_object();

    object.define_prop("a", PropertyDescriptor {
        value: Some(123),
        ..Default::default()
    }).unwrap();
    assert_eq!(object.get::<_, i8>("a").unwrap(), 123);

    let get = ducc.create_function(|inv| Ok(24));
    object.define_prop("b", PropertyDescriptor::<()> {
        get: Some(get),
        ..Default::default()
    }).unwrap();
    assert_eq!(object.get::<_, i8>("b").unwrap(), 24);

    let mut v = Arc::new(Mutex::new(0));
    let mut v_c = Arc::clone(&v);
    let set = ducc.create_function(move|inv| {
        let mut c = Arc::clone(&v_c);
        let (a,): (i8,) = inv.args.into(inv.ducc)?;
        *c.lock().unwrap() = a;
        Ok(())
    });
    object.define_prop("c", PropertyDescriptor::<()> {
        set: Some(set),
        ..Default::default()
    }).unwrap();
    object.set::<_, i8>("c", 24).unwrap();
    assert_eq!(*v.lock().unwrap(), 24);
}

#[test]
fn remove() {
    let ducc = Ducc::new();
    let globals = ducc.globals();
    assert!(globals.contains_key("Object").unwrap());
    globals.remove("Object").unwrap();
    assert!(!globals.contains_key("Object").unwrap());
    // Removing keys that don't exist does nothing:
    globals.remove("Object").unwrap();
    assert!(!globals.contains_key("Object").unwrap());
}

#[test]
fn call_prop() {
    fn add(inv: Invocation) -> Result<usize> {
        let this: Object = inv.this.into(inv.ducc)?;
        let (acc,): (usize,) = inv.args.into(inv.ducc)?;
        return Ok(this.get::<_, usize>("base").unwrap() + acc);
    }

    let ducc = Ducc::new();
    let object = ducc.create_object();
    object.set("base", 123).unwrap();
    object.set("add", ducc.create_function(add)).unwrap();
    let number: f64 = object.call_prop("add", (456,)).unwrap();
    assert_eq!(number, 579.0f64);
}

#[test]
fn properties() {
    let ducc = Ducc::new();

    let object = ducc.create_object();
    object.set("a", 123).unwrap();
    object.set(4, Value::Undefined).unwrap();
    object.set(123, "456").unwrap();

    let list = object.properties().map(|property| {
        let result: (String, usize) = property.unwrap();
        result
    }).collect::<Vec<_>>();

    assert_eq!(list, vec![("4".to_string(), 0), ("123".to_string(), 456), ("a".to_string(), 123)]);
}
