use array::Array;
use bytes::Bytes;
use ducc::Ducc;
use error::{Error, Result};
use function::Function;
use object::Object;
use std::collections::{BTreeMap, HashMap, BTreeSet, HashSet};
use std::cmp::{Eq, Ord};
use std::hash::{BuildHasher, Hash};
use std::string::String as StdString;
use string::String;
use value::{FromValue, FromValues, ToValue, ToValues, Value, Values, Variadic};

impl<'ducc> ToValue<'ducc> for Value<'ducc> {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(self)
    }
}

impl<'ducc> FromValue<'ducc> for Value<'ducc> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        Ok(value)
    }
}

impl<'ducc> ToValue<'ducc> for () {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::Undefined)
    }
}

impl<'ducc> FromValue<'ducc> for () {
    fn from_value(_value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        Ok(())
    }
}

impl<'ducc, T: ToValue<'ducc>> ToValue<'ducc> for Option<T> {
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        match self {
            Some(val) => val.to_value(ducc),
            None => Ok(Value::Null),
        }
    }
}

impl<'ducc, T: FromValue<'ducc>> FromValue<'ducc> for Option<T> {
    fn from_value(value: Value<'ducc>, ducc: &'ducc Ducc) -> Result<Self> {
        match value {
            Value::Null | Value::Undefined => Ok(None),
            value => Ok(Some(T::from_value(value, ducc)?)),
        }
    }
}

impl<'ducc> ToValue<'ducc> for String<'ducc> {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::String(self))
    }
}

impl<'ducc> FromValue<'ducc> for String<'ducc> {
    fn from_value(value: Value<'ducc>, ducc: &'ducc Ducc) -> Result<String<'ducc>> {
        ducc.coerce_string(value)
    }
}

impl<'ducc> ToValue<'ducc> for Function<'ducc> {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::Function(self))
    }
}

impl<'ducc> FromValue<'ducc> for Function<'ducc> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Function<'ducc>> {
        match value {
            Value::Function(f) => Ok(f),
            value => Err(Error::from_js_conversion(value.type_name(), "Function")),
        }
    }
}

impl<'ducc> ToValue<'ducc> for Array<'ducc> {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::Array(self))
    }
}

impl<'ducc> FromValue<'ducc> for Array<'ducc> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Array<'ducc>> {
        match value {
            Value::Array(a) => Ok(a),
            value => Err(Error::from_js_conversion(value.type_name(), "Array")),
        }
    }
}

impl<'ducc> ToValue<'ducc> for Object<'ducc> {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::Object(self))
    }
}

impl<'ducc> FromValue<'ducc> for Object<'ducc> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Object<'ducc>> {
        match value {
            Value::Object(o) => Ok(o),
            value => Err(Error::from_js_conversion(value.type_name(), "Object")),
        }
    }
}

impl<'ducc> ToValue<'ducc> for Bytes<'ducc> {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::Bytes(self))
    }
}

impl<'ducc> FromValue<'ducc> for Bytes<'ducc> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Bytes<'ducc>> {
        match value {
            Value::Bytes(b) => Ok(b),
            value => Err(Error::from_js_conversion(value.type_name(), "Bytes")),
        }
    }
}

impl<'ducc, K, V, S> ToValue<'ducc> for HashMap<K, V, S>
where
    K: Eq + Hash + ToValue<'ducc>,
    V: ToValue<'ducc>,
    S: BuildHasher,
{
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        let object = ducc.create_object();
        for (k, v) in self.into_iter() {
            object.set(k, v)?;
        }
        Ok(Value::Object(object))
    }
}

impl<'ducc, K, V, S> FromValue<'ducc> for HashMap<K, V, S>
where
    K: Eq + Hash + FromValue<'ducc>,
    V: FromValue<'ducc>,
    S: BuildHasher + Default,
{
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        match value {
            Value::Object(o) => o.properties().collect(),
            value => Err(Error::from_js_conversion(value.type_name(), "HashMap")),
        }
    }
}

impl<'ducc, K, V> ToValue<'ducc> for BTreeMap<K, V>
where
    K: Ord + ToValue<'ducc>,
    V: ToValue<'ducc>,
{
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        let object = ducc.create_object();
        for (k, v) in self.into_iter() {
            object.set(k, v)?;
        }
        Ok(Value::Object(object))
    }
}

impl<'ducc, K, V> FromValue<'ducc> for BTreeMap<K, V>
where
    K: Ord + FromValue<'ducc>,
    V: FromValue<'ducc>,
{
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        match value {
            Value::Object(o) => o.properties().collect(),
            value => Err(Error::from_js_conversion(value.type_name(), "BTreeMap")),
        }
    }
}

impl<'ducc, V: ToValue<'ducc>> ToValue<'ducc> for BTreeSet<V> {
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        let array = ducc.create_array();
        for v in self.into_iter() {
            array.push(v)?;
        }
        Ok(Value::Array(array))
    }
}

impl<'ducc, V: FromValue<'ducc> + Ord> FromValue<'ducc> for BTreeSet<V> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        match value {
            Value::Array(a) => a.elements().collect(),
            value => Err(Error::from_js_conversion(value.type_name(), "BTreeSet")),
        }
    }
}

impl<'ducc, V: ToValue<'ducc>> ToValue<'ducc> for HashSet<V> {
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        let array = ducc.create_array();
        for v in self.into_iter() {
            array.push(v)?;
        }
        Ok(Value::Array(array))
    }
}

impl<'ducc, V: FromValue<'ducc> + Hash + Eq> FromValue<'ducc> for HashSet<V> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        match value {
            Value::Array(a) => a.elements().collect(),
            value => Err(Error::from_js_conversion(value.type_name(), "HashSet")),
        }
    }
}

impl<'ducc, V: ToValue<'ducc>> ToValue<'ducc> for Vec<V> {
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        let array = ducc.create_array();
        for v in self.into_iter() {
            array.push(v)?;
        }
        Ok(Value::Array(array))
    }
}

impl<'ducc, V: FromValue<'ducc>> FromValue<'ducc> for Vec<V> {
    fn from_value(value: Value<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        match value {
            Value::Array(a) => a.elements().collect(),
            value => Err(Error::from_js_conversion(value.type_name(), "Vec")),
        }
    }
}

impl<'ducc> ToValue<'ducc> for bool {
    fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::Boolean(self))
    }
}

impl<'ducc> FromValue<'ducc> for bool {
    fn from_value(value: Value, ducc: &'ducc Ducc) -> Result<Self> {
        Ok(ducc.coerce_boolean(value))
    }
}

impl<'ducc> ToValue<'ducc> for StdString {
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::String(ducc.create_string(&self)?))
    }
}

impl<'ducc> FromValue<'ducc> for StdString {
    fn from_value(value: Value<'ducc>, ducc: &'ducc Ducc) -> Result<Self> {
        Ok(ducc.coerce_string(value)?.to_string()?)
    }
}

impl<'ducc, 'a> ToValue<'ducc> for &'a str {
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
        Ok(Value::String(ducc.create_string(self)?))
    }
}

macro_rules! convert_number {
    ($prim_ty: ty) => {
        impl<'ducc> ToValue<'ducc> for $prim_ty {
            fn to_value(self, _ducc: &'ducc Ducc) -> Result<Value<'ducc>> {
                Ok(Value::Number(self as f64))
            }
        }

        impl<'ducc> FromValue<'ducc> for $prim_ty {
            fn from_value(value: Value<'ducc>, ducc: &'ducc Ducc) -> Result<Self> {
                Ok(ducc.coerce_number(value)? as $prim_ty)
            }
        }
    }
}

convert_number!(i8);
convert_number!(u8);
convert_number!(i16);
convert_number!(u16);
convert_number!(i32);
convert_number!(u32);
convert_number!(i64);
convert_number!(u64);
convert_number!(isize);
convert_number!(usize);
convert_number!(f32);
convert_number!(f64);

impl<'ducc> ToValues<'ducc> for Values<'ducc> {
    fn to_values(self, _ducc: &'ducc Ducc) -> Result<Values<'ducc>> {
        Ok(self)
    }
}

impl<'ducc> FromValues<'ducc> for Values<'ducc> {
    fn from_values(values: Values<'ducc>, _ducc: &'ducc Ducc) -> Result<Self> {
        Ok(values)
    }
}

impl<'ducc, T: ToValue<'ducc>> ToValues<'ducc> for Variadic<T> {
    fn to_values(self, ducc: &'ducc Ducc) -> Result<Values<'ducc>> {
        self.0.into_iter().map(|value| value.to_value(ducc)).collect()
    }
}

impl<'ducc, T: FromValue<'ducc>> FromValues<'ducc> for Variadic<T> {
    fn from_values(values: Values<'ducc>, ducc: &'ducc Ducc) -> Result<Self> {
        values.into_iter()
            .map(|value| T::from_value(value, ducc))
            .collect::<Result<Vec<T>>>()
            .map(Variadic)
    }
}

impl<'ducc> ToValues<'ducc> for () {
    fn to_values(self, _ducc: &'ducc Ducc) -> Result<Values<'ducc>> {
        Ok(Values::new())
    }
}

impl<'ducc> FromValues<'ducc> for () {
    fn from_values(_values: Values, _ducc: &'ducc Ducc) -> Result<Self> {
        Ok(())
    }
}

macro_rules! impl_tuple {
    ($($name:ident),*) => (
        impl<'ducc, $($name),*> ToValues<'ducc> for ($($name,)*)
        where
            $($name: ToValue<'ducc>,)*
        {
            #[allow(non_snake_case)]
            fn to_values(self, ducc: &'ducc Ducc) -> Result<Values<'ducc>> {
                let ($($name,)*) = self;
                let reservation = $({ &$name; 1 } +)* 0;
                let mut results = Vec::with_capacity(reservation);
                $(results.push($name.to_value(ducc)?);)*
                Ok(Values::from_vec(results))
            }
        }

        impl<'ducc, $($name),*> FromValues<'ducc> for ($($name,)*)
        where
            $($name: FromValue<'ducc>,)*
        {
            #[allow(non_snake_case, unused_mut, unused_variables)]
            fn from_values(values: Values<'ducc>, ducc: &'ducc Ducc) -> Result<Self> {
                let mut iter = values.into_vec().into_iter();
                Ok(($({
                    let $name = ();
                    FromValue::from_value(iter.next().unwrap_or(Value::Undefined), ducc)?
                },)*))
            }
        }

        impl<'ducc, $($name,)* VAR> ToValues<'ducc> for ($($name,)* Variadic<VAR>)
        where
            $($name: ToValue<'ducc>,)*
            VAR: ToValue<'ducc>,
        {
            #[allow(non_snake_case)]
            fn to_values(self, ducc: &'ducc Ducc) -> Result<Values<'ducc>> {
                let ($($name,)* variadic) = self;
                let reservation = $({ &$name; 1 } +)* 1;
                let mut results = Vec::with_capacity(reservation);
                $(results.push($name.to_value(ducc)?);)*
                if results.is_empty() {
                    Ok(variadic.to_values(ducc)?)
                } else {
                    results.append(&mut variadic.to_values(ducc)?.into_vec());
                    Ok(Values::from_vec(results))
                }
            }
        }

        impl<'ducc, $($name,)* VAR> FromValues<'ducc> for ($($name,)* Variadic<VAR>)
        where
            $($name: FromValue<'ducc>,)*
            VAR: FromValue<'ducc>,
        {
            #[allow(non_snake_case, unused_mut, unused_variables)]
            fn from_values(values: Values<'ducc>, ducc: &'ducc Ducc) -> Result<Self> {
                let mut values = values.into_vec();
                let len = values.len();
                let split = $({ let $name = (); 1 } +)* 0;

                if len < split {
                    values.reserve(split - len);
                    for _ in len..split {
                        values.push(Value::Undefined);
                    }
                }

                let last_values = Values::from_vec(values.split_off(split));
                let variadic = FromValues::from_values(last_values, ducc)?;

                let mut iter = values.into_iter();
                let ($($name,)*) = ($({ let $name = (); iter.next().unwrap() },)*);

                Ok(($(FromValue::from_value($name, ducc)?,)* variadic))
            }
        }
    )
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
