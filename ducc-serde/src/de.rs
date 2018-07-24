use ducc::{Elements, Properties, Value};
use error::{Error, Result};
use serde;
use serde::de::IntoDeserializer;

pub struct Deserializer<'ducc> {
    pub value: Value<'ducc>,
}

impl<'ducc, 'de> serde::Deserializer<'de> for Deserializer<'ducc> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        match self.value {
            Value::Undefined => visitor.visit_unit(),
            Value::Null => visitor.visit_none(),
            Value::Boolean(v) => visitor.visit_bool(v),
            Value::Number(v) => visitor.visit_f64(v),
            Value::String(v) => visitor.visit_str(&v.to_string()?),
            Value::Array(v) => {
                let len = v.len()?;
                let mut deserializer = SeqDeserializer(v.elements());
                let seq = visitor.visit_seq(&mut deserializer)?;
                let remaining = deserializer.0.count();
                if remaining == 0 {
                    Ok(seq)
                } else {
                    Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
                }
            },
            Value::Object(v) => {
                let len = v.len()? as usize;
                let mut deserializer = MapDeserializer(v.properties(), None);
                let map = visitor.visit_map(&mut deserializer)?;
                let remaining = deserializer.0.count();
                if remaining == 0 {
                    Ok(map)
                } else {
                    Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
                }
            },
            Value::Bytes(v) => visitor.visit_bytes(&v.to_vec()),
            // TODO: Should values that cannot be deserialized be treated as errors? Perhaps this
            // should be a configurable behavior.
            _ => visitor.visit_unit(),
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        match self.value {
            Value::Null | Value::Undefined => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        let (variant, value) = match self.value {
            Value::Object(value) => {
                let mut iter = value.properties::<String, Value>();
                let (variant, value) = match iter.next() {
                    Some(v) => v?,
                    None => return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Map,
                        &"map with a single key",
                    )),
                };

                if iter.next().is_some() {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Map,
                        &"map with a single key",
                    ))
                }
                (variant, Some(value))
            }
            Value::String(variant) => (variant.to_string()?, None),
            _ => return Err(serde::de::Error::custom("bad enum value")),
        };

        visitor.visit_enum(EnumDeserializer { variant, value })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}


struct SeqDeserializer<'ducc>(Elements<'ducc, Value<'ducc>>);

impl<'ducc, 'de> serde::de::SeqAccess<'de> for SeqDeserializer<'ducc> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>
    {
        match self.0.next() {
            Some(value) => seed.deserialize(Deserializer { value: value? }).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.0.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}


struct MapDeserializer<'ducc>(
    Properties<'ducc, Value<'ducc>, Value<'ducc>>,
    Option<Value<'ducc>>
);

impl<'ducc, 'de> serde::de::MapAccess<'de> for MapDeserializer<'ducc> {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>
    {
        match self.0.next() {
            Some(item) => {
                let (key, value) = item?;
                self.1 = Some(value);
                let key_de = Deserializer { value: key };
                seed.deserialize(key_de).map(Some)
            },
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>
    {
        match self.1.take() {
            Some(value) => seed.deserialize(Deserializer { value }),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.0.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}


struct EnumDeserializer<'ducc> {
    variant: String,
    value: Option<Value<'ducc>>,
}

impl<'ducc, 'de> serde::de::EnumAccess<'de> for EnumDeserializer<'ducc> {
    type Error = Error;
    type Variant = VariantDeserializer<'ducc>;

    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant)>
    where
        T: serde::de::DeserializeSeed<'de>
    {
        let variant = self.variant.into_deserializer();
        let variant_access = VariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, variant_access))
    }
}


struct VariantDeserializer<'ducc> {
    value: Option<Value<'ducc>>,
}

impl<'ducc, 'de> serde::de::VariantAccess<'de> for VariantDeserializer<'ducc> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        match self.value {
            Some(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::NewtypeVariant,
                &"unit variant",
            )),
            None => Ok(())
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>
    {
        match self.value {
            Some(value) => seed.deserialize(Deserializer { value }),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"newtype variant",
            ))
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>
    {
        match self.value {
            Some(value) => serde::Deserializer::deserialize_seq(
                Deserializer { value },
                visitor,
            ),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"tuple variant",
            ))
        }
    }

    fn struct_variant<V>(
        self, _fields: &'static [&'static str], visitor: V
    ) -> Result<V::Value>
        where V: serde::de::Visitor<'de>
    {
        match self.value {
            Some(value) => serde::Deserializer::deserialize_map(
                Deserializer { value },
                visitor,
            ),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"struct variant",
            ))
        }
    }
}
