use ducc::{Array, Ducc, Object, String as DuccString, Value};
use serde;
use super::{Error, Result, to_value};

pub struct Serializer<'ducc> {
    pub ducc: &'ducc Ducc,
}

impl<'ducc> serde::Serializer for Serializer<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    type SerializeSeq = SerializeVec<'ducc>;
    type SerializeTuple = SerializeVec<'ducc>;
    type SerializeTupleStruct = SerializeVec<'ducc>;
    type SerializeTupleVariant  = SerializeTupleVariant<'ducc>;
    type SerializeMap = SerializeMap<'ducc>;
    type SerializeStruct = SerializeMap<'ducc>;
    type SerializeStructVariant = SerializeStructVariant<'ducc>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Value<'ducc>> {
        Ok(Value::Boolean(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Value<'ducc>> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Value<'ducc>> {
        Ok(Value::Number(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Value<'ducc>> {
        let mut string = String::new();
        string.push(value);
        self.serialize_str(&string)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Value<'ducc>> {
        Ok(Value::String(self.ducc.create_string(value)?))
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<Value<'ducc>> {
        Ok(Value::Bytes(self.ducc.create_bytes(value)?))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Value<'ducc>> {
        Ok(Value::Undefined)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value<'ducc>> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value<'ducc>> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value<'ducc>>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value<'ducc>>
    where
        T: ?Sized + serde::Serialize,
    {
        let object = self.ducc.create_object();
        let variant = self.ducc.create_string(variant)?;
        let value = to_value(self.ducc, value)?;
        object.set(variant, value)?;
        Ok(Value::Object(object))
    }

    #[inline]
    fn serialize_none(self) -> Result<Value<'ducc>> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Value<'ducc>>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        let array = self.ducc.create_array();
        Ok(SerializeVec {
            ducc: self.ducc,
            array,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct>
    {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let name = self.ducc.create_string(variant)?;
        let array = self.ducc.create_array();
        Ok(SerializeTupleVariant {
            ducc: self.ducc,
            array,
            name,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        let object = self.ducc.create_object();
        Ok(SerializeMap {
            ducc: self.ducc,
            object,
            next_key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let name = self.ducc.create_string(variant)?;
        let object = self.ducc.create_object();
        Ok(SerializeStructVariant {
            ducc: self.ducc,
            object,
            name,
        })
    }
}

pub struct SerializeVec<'ducc> {
    ducc: &'ducc Ducc,
    array: Array<'ducc>,
}

impl<'ducc> serde::ser::SerializeSeq for SerializeVec<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.array.push(to_value(self.ducc, value)?)?;
        Ok(())
    }

    fn end(self) -> Result<Value<'ducc>> {
        Ok(Value::Array(self.array))
    }
}

impl<'ducc> serde::ser::SerializeTuple for SerializeVec<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value<'ducc>> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'ducc> serde::ser::SerializeTupleStruct for SerializeVec<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value<'ducc>> {
        serde::ser::SerializeSeq::end(self)
    }
}

pub struct SerializeTupleVariant<'ducc> {
    ducc: &'ducc Ducc,
    name: DuccString<'ducc>,
    array: Array<'ducc>,
}

impl<'ducc> serde::ser::SerializeTupleVariant for SerializeTupleVariant<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.array.push(to_value(self.ducc, value)?)?;
        Ok(())
    }

    fn end(self) -> Result<Value<'ducc>> {
        let object = self.ducc.create_object();
        object.set(self.name, self.array)?;
        Ok(Value::Object(object))
    }
}

pub struct SerializeMap<'ducc> {
    ducc: &'ducc Ducc,
    object: Object<'ducc>,
    next_key: Option<Value<'ducc>>
}

impl<'ducc> serde::ser::SerializeMap for SerializeMap<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.next_key = Some(to_value(self.ducc, key)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let key = self.next_key.take().expect("serialize_value called before serialize_key");
        self.object.set(key, to_value(self.ducc, value)?)?;
        Ok(())
    }

    fn end(self) -> Result<Value<'ducc>> {
        Ok(Value::Object(self.object))
    }
}

impl<'ducc> serde::ser::SerializeStruct for SerializeMap<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        serde::ser::SerializeMap::serialize_key(self, key)?;
        serde::ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> Result<Value<'ducc>> {
        serde::ser::SerializeMap::end(self)
    }
}

pub struct SerializeStructVariant<'ducc> {
    ducc: &'ducc Ducc,
    object: Object<'ducc>,
    name: DuccString<'ducc>,
}

impl<'ducc> serde::ser::SerializeStructVariant for SerializeStructVariant<'ducc> {
    type Ok = Value<'ducc>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.object.set(key, to_value(self.ducc, value)?)?;
        Ok(())
    }

    fn end(self) -> Result<Value<'ducc>> {
        let object = self.ducc.create_object();
        object.set(self.name, self.object)?;
        Ok(Value::Object(object))
    }
}
