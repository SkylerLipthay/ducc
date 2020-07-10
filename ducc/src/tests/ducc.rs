use ducc::{Ducc, ExecSettings};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use value::Value;

#[test]
#[should_panic]
fn value_cross_contamination() {
    let ducc_1 = Ducc::new();
    let str_1 = ducc_1.create_string("123").unwrap();
    let ducc_2 = Ducc::new();
    let _str_2 = ducc_2.create_string("456").unwrap();
    let _ = ducc_2.coerce_number(Value::String(str_1));
}

#[test]
fn timeout() {
    let ducc = Ducc::new();
    let start = Instant::now();
    let cancel_fn = move || Instant::now().duration_since(start) > Duration::from_millis(500);
    let settings = ExecSettings { cancel_fn: Some(Box::new(cancel_fn)) };
    let result: Result<(), _> = ducc.exec("for (;;) {}", None, settings);
    assert!(result.is_err());
}

#[test]
fn no_duktape_global() {
    let ducc = Ducc::new();
    let globals = ducc.globals();
    assert!(!globals.contains_key("Duktape").unwrap());
}

#[test]
fn inspect_callstack() {
    let ducc = Ducc::new();
    ducc.globals().set("fun", ducc.create_function(|inv| {
        inv.ducc.inspect_callstack_entry(-1).expect("callstack entry was None");
        Ok(())
    })).unwrap();
    ducc.globals().get::<_, Function>("fun").unwrap().call::<(), ()>(()).unwrap();
}

#[test]
fn user_data_drop() {
    let mut ducc = Ducc::new();
    let (count, data) = make_test_user_data();
    ducc.set_user_data("data", data);
    drop(ducc);
    assert_eq!(*count.borrow(), 1000);
}

#[test]
fn user_data_get() {
    let mut ducc = Ducc::new();
    let (_, data) = make_test_user_data();
    ducc.set_user_data("data", data);
    assert!(ducc.get_user_data::<TestUserData>("no-exist").is_none());
    assert!(ducc.get_user_data::<usize>("data").is_none());

    {
        let data = ducc.get_user_data::<TestUserData>("data").unwrap();
        assert_eq!(data.get(), 0);
        data.increase();
        assert_eq!(data.get(), 1);
    }
}

#[test]
fn user_data_remove() {
    let mut ducc = Ducc::new();
    let (count, data) = make_test_user_data();
    ducc.set_user_data("data", data);
    assert_eq!(*count.borrow(), 0);
    let data = ducc.remove_user_data("data").unwrap();
    assert_eq!(*count.borrow(), 0);
    data.downcast_ref::<TestUserData>().unwrap().increase();
    assert_eq!(*count.borrow(), 1);
    drop(data);
    assert_eq!(*count.borrow(), 1000);
}

struct TestUserData {
    count: Rc<RefCell<usize>>,
}

impl TestUserData {
    fn increase(&self) {
        *self.count.borrow_mut() += 1;
    }

    fn get(&self) -> usize {
        *self.count.borrow()
    }
}

impl Drop for TestUserData {
    fn drop(&mut self) {
        *self.count.borrow_mut() = 1000;
    }
}

fn make_test_user_data() -> (Rc<RefCell<usize>>, TestUserData) {
    let count = Rc::new(RefCell::new(0));
    (count.clone(), TestUserData { count })
}
