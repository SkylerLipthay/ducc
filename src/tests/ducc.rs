use ducc::Ducc;
use std::time::Duration;
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
    let result: Result<(), _> = ducc.exec("for (;;) {}", None, Some(Duration::from_millis(500)));
    assert!(result.is_err());
}
