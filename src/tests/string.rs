use ducc::Ducc;
use std::borrow::Cow;
use string::String;

fn with_str<F: FnOnce(String)>(s: &str, f: F) {
    let ducc = Ducc::new();
    let string = ducc.create_string(s).unwrap();
    f(string);
}

#[test]
fn compare() {
    with_str("test", |s| assert_eq!(s, "test")); // &str
    with_str("test", |s| assert_eq!(s, b"test")); // &[u8]
    with_str("test", |s| assert_eq!(s, b"test".to_vec())); // Vec<u8>
    with_str("test", |s| assert_eq!(s, "test".to_string())); // String
    with_str("test", |s| assert_eq!(s, Cow::from(b"test".as_ref()))); // Cow (borrowed)
    with_str("test", |s| assert_eq!(s, Cow::from(b"test".to_vec()))); // Cow (owned)
    with_str("test", |s| assert_eq!(s, s)); // ducc::String
}
