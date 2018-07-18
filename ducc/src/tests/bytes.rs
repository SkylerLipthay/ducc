use ducc::Ducc;

#[test]
fn to_vec() {
    let ducc = Ducc::new();
    let bytes = ducc.create_bytes(&[1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(bytes.to_vec(), vec![1, 2, 3, 4, 5, 6]);
}
