use array::Array;
use ducc::Ducc;
use object::Object;
use std::collections::{BTreeMap, HashMap, BTreeSet, HashSet};
use value::{FromValue, FromValues, ToValue, ToValues, Value, Variadic};

#[test]
fn option() {
    let ducc = Ducc::new();

    let none_val = None::<usize>.to_value(&ducc).unwrap();
    assert!(none_val.is_null());
    let num_val = Some(123).to_value(&ducc).unwrap();
    assert!(num_val.is_number());

    let none: Option<usize> = FromValue::from_value(none_val.clone(), &ducc).unwrap();
    assert_eq!(none, None::<usize>);
    let undefined: Option<usize> = FromValue::from_value(Value::Undefined, &ducc).unwrap();
    assert_eq!(undefined, None::<usize>);
    let some_num: Option<usize> = FromValue::from_value(num_val.clone(), &ducc).unwrap();
    assert_eq!(some_num, Some(123));
    let num: usize = FromValue::from_value(num_val.clone(), &ducc).unwrap();
    assert_eq!(num, 123);
    let num_zero: usize = FromValue::from_value(none_val.clone(), &ducc).unwrap();
    assert_eq!(num_zero, 0);
}

#[test]
fn variadic() {
    let ducc = Ducc::new();
    let values = (true, false, true).to_values(&ducc).unwrap();

    let var: Variadic<bool> = FromValues::from_values(values.clone(), &ducc).unwrap();
    assert_eq!(*var, vec![true, false, true]);

    let values = (true, Variadic::from_vec(vec![false, true])).to_values(&ducc).unwrap();
    let var: Variadic<bool> = FromValues::from_values(values.clone(), &ducc).unwrap();
    assert_eq!(*var, vec![true, false, true]);
}

#[test]
fn tuple() {
    let ducc = Ducc::new();
    let values = (true, false, true).to_values(&ducc).unwrap();

    let out: (bool, bool, bool) = FromValues::from_values(values.clone(), &ducc).unwrap();
    assert_eq!((true, false, true), out);

    let out: (bool, bool) = FromValues::from_values(values.clone(), &ducc).unwrap();
    assert_eq!((true, false), out);

    type Overflow<'a> = (bool, bool, bool, Value<'a>, Value<'a>);
    let (a, b, c, d, e): Overflow = FromValues::from_values(values.clone(), &ducc).unwrap();
    assert_eq!((true, false, true), (a, b, c));
    assert!(d.is_undefined());
    assert!(e.is_undefined());

    type VariadicTuple = (bool, Variadic<bool>);
    let (a, var): VariadicTuple = FromValues::from_values(values.clone(), &ducc).unwrap();
    assert_eq!(true, a);
    assert_eq!(*var, vec![false, true]);

    type VariadicOver = (bool, bool, bool, bool, Variadic<bool>);
    let (a, b, c, d, var): VariadicOver = FromValues::from_values(values.clone(), &ducc).unwrap();
    assert_eq!((true, false, true, false), (a, b, c, d));
    assert_eq!(*var, vec![]);
}

#[test]
fn hash_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    let ducc = Ducc::new();
    let list = map.to_value(&ducc).unwrap().into::<Object>(&ducc).unwrap().properties().map(|p| {
        let result: (usize, usize) = p.unwrap();
        result
    }).collect::<Vec<_>>();
    assert_eq!(list, vec![(1, 2), (3, 4), (5, 6)]);
}

#[test]
fn btree_map() {
    let mut map = BTreeMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    let ducc = Ducc::new();
    let list = map.to_value(&ducc).unwrap().into::<Object>(&ducc).unwrap().properties().map(|p| {
        let result: (usize, usize) = p.unwrap();
        result
    }).collect::<Vec<_>>();
    assert_eq!(list, vec![(1, 2), (3, 4), (5, 6)]);
}

#[test]
fn vec() {
    let vec = vec![1, 2, 3];
    let ducc = Ducc::new();
    let list: Result<Vec<usize>, _> = vec.to_value(&ducc).unwrap().into::<Array>(&ducc)
        .unwrap().elements().collect();
    assert_eq!(list.unwrap(), vec![1, 2, 3]);
}

#[test]
fn btree_set() {
    let btree_set: BTreeSet<_> = vec![1, 2, 3].into_iter().collect();
    let ducc = Ducc::new();
    let list: Result<BTreeSet<usize>, _> = btree_set.to_value(&ducc).unwrap().into::<Array>(&ducc)
        .unwrap().elements().collect();
    assert_eq!(list.unwrap(), vec![1, 2, 3].into_iter().collect());
}

#[test]
fn hash_set() {
    let hash_set: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let ducc = Ducc::new();
    let list: Result<HashSet<usize>, _> = hash_set.to_value(&ducc).unwrap().into::<Array>(&ducc)
        .unwrap().elements().collect();
    assert_eq!(list.unwrap(), vec![1, 2, 3].into_iter().collect());
}
