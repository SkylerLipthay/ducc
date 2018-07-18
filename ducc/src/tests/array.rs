use ducc::Ducc;
use value::Value;

#[test]
fn set_get() {
    let ducc = Ducc::new();

    let array = ducc.create_array();
    array.set(0, 123).unwrap();
    array.set(2, 456).unwrap();
    assert_eq!(array.get::<String>(0).unwrap(), "123");
    assert!(array.get::<Value>(1).unwrap().is_undefined());
    assert_eq!(array.get::<String>(2).unwrap(), "456");
    assert!(array.get::<Value>(3).unwrap().is_undefined());
}

#[test]
fn len() {
    let ducc = Ducc::new();

    let array = ducc.create_array();
    assert_eq!(array.len().unwrap(), 0);
    array.set(0, 123).unwrap();
    assert_eq!(array.len().unwrap(), 1);
    array.set(2, 456).unwrap();
    assert_eq!(array.len().unwrap(), 3);
}

#[test]
fn push() {
    let ducc = Ducc::new();

    let array = ducc.create_array();
    array.push(0).unwrap();
    array.push(1).unwrap();
    array.set(3, 3).unwrap();
    array.push(4).unwrap();
    assert_eq!(array.get::<usize>(0).unwrap(), 0);
    assert_eq!(array.get::<usize>(1).unwrap(), 1);
    assert!(array.get::<Value>(2).unwrap().is_undefined());
    assert_eq!(array.get::<usize>(3).unwrap(), 3);
    assert_eq!(array.get::<usize>(4).unwrap(), 4);
    assert_eq!(array.len().unwrap(), 5);
}

#[test]
fn elements() {
    let ducc = Ducc::new();

    let array = ducc.create_array();
    array.push(0).unwrap();
    array.push(1).unwrap();
    array.set(3, 3).unwrap();
    array.push(4).unwrap();

    let list: Result<Vec<usize>, _> = array.elements().collect();
    assert_eq!(list.unwrap(), vec![0, 1, 0, 3, 4]);
}
