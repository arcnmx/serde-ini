use std::io::{self, Write};
use std::{result, fmt};
use serde::ser::{self, Serialize, Impossible};
use write::Writer;
use parse::Item;

#[derive(Copy, Clone, Debug)]
pub enum UnsupportedType {
    Bool,
    Bytes,
    None,
    Unit,
    Seq,
    Map,
}

#[derive(Debug, Clone)]
pub enum Error {
    /// Serialization error
    ///
    /// Passed through error message from the type being serialized.
    Custom(String),

    /// Attempted to serialize a type not supported by the INI format
    ///
    /// INI values can only be strings, or numeric values supported by `FromStr`.
    UnsupportedType(UnsupportedType),

    /// INI section and key names must be a string
    NonStringKey,

    /// An entire INI file can only be serialized from a map or struct type
    TopLevelMap,

    /// Top-level values without a section cannot be serialized after a section has been written
    OrphanValue,

    /// Serializer consistency error
    ///
    /// This error indicates that the `SerializeMap` API was misused.
    MapKeyMissing,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Custom(e.to_string())
    }
}

impl From<UnsupportedType> for Error {
    fn from(t: UnsupportedType) -> Self {
        Error::UnsupportedType(t)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::UnsupportedType(ty) => write!(f, "{:?} cannot be serialized into INI", ty),
            Error::NonStringKey => write!(f, "INI map keys must be a string type"),
            Error::OrphanValue => write!(f, "top-level INI values must be serialized before any map sections"),
            Error::MapKeyMissing => write!(f, "serializer consistency error: attempted to serialize map value without key"),
            Error::TopLevelMap => write!(f, "INI can only represent a map or struct type"),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "INI serialization error"
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = result::Result<T, Error>;

pub struct Serializer<W> {
    writer: Writer<W>,
}

impl<W> Serializer<W> {
    pub fn new(writer: Writer<W>) -> Self {
        Serializer {
            writer: writer,
        }
    }
}

struct ValueSerializer<'a, 'k, W: 'a> {
    writer: &'a mut Writer<W>,
    key: &'k str,
    top_level: bool,
    allow_values: &'a mut bool,
}

pub struct MapSerializer<'a, W: 'a> {
    writer: &'a mut Writer<W>,
    key: Option<String>,
    top_level: bool,
    allow_values: bool,
}

impl<'a, 'k, W: Write> ValueSerializer<'a, 'k, W> {
    fn serialize_string(&mut self, s: String) -> Result<()> {
        if !self.top_level || *self.allow_values {
            self.writer.write(&Item::Value {
                key: self.key.into(),
                value: s,
            }).map_err(Into::into)
        } else {
            Err(Error::OrphanValue)
        }
    }

    fn serialize_section(&mut self) -> Result<()> {
        self.writer.write(&Item::Section {
            name: self.key.into(),
        }).map_err(Into::into)
    }
}

impl<'a, 'k, W: Write + 'a> ser::Serializer for ValueSerializer<'a, 'k, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(UnsupportedType::Bool.into())
    }

    fn serialize_i8(mut self, v: i8) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_i16(mut self, v: i16) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_i32(mut self, v: i32) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_i64(mut self, v: i64) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_u8(mut self, v: u8) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_u16(mut self, v: u16) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_u32(mut self, v: u32) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_u64(mut self, v: u64) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_f32(mut self, v: f32) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_f64(mut self, v: f64) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_char(mut self, v: char) -> Result<()> {
        self.serialize_string(v.to_string())
    }

    fn serialize_str(mut self, v: &str) -> Result<()> {
        self.serialize_string(v.into())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(UnsupportedType::Bytes.into())
    }

    fn serialize_none(self) -> Result<()> {
        Err(UnsupportedType::None.into())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(UnsupportedType::Unit.into())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(UnsupportedType::Unit.into())
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()> {
        Err(UnsupportedType::Unit.into())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(UnsupportedType::Seq.into())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
        Err(UnsupportedType::Seq.into())
    }

    fn serialize_map(mut self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        if self.top_level {
            *self.allow_values = false;
            self.serialize_section().map(move |_| MapSerializer {
                writer: self.writer,
                key: None,
                top_level: false,
                allow_values: false,
            })
        } else {
            Err(UnsupportedType::Map.into())
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        Err(UnsupportedType::Map.into())
    }
}

#[derive(Default)]
struct KeySerializer {
    key: String,
}

impl<'a> ser::Serializer for &'a mut KeySerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_u32(self, _v: u32) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        Ok(self.key = v.into())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()> {
        Err(Error::NonStringKey)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::NonStringKey)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
        Err(Error::NonStringKey)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::NonStringKey)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        Err(Error::NonStringKey)
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_u32(self, _v: u32) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_str(self, _v: &str) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()> {
        Err(Error::TopLevelMap)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::TopLevelMap)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
        Err(Error::TopLevelMap)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer {
            writer: &mut self.writer,
            key: None,
            top_level: true,
            allow_values: true,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        Err(Error::TopLevelMap)
    }
}

impl<'a, W: Write> ser::SerializeMap for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        let mut k = KeySerializer::default();
        key.serialize(&mut k)?;
        self.key = Some(k.key);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        let writer = &mut self.writer;
        let allow_values = &mut self.allow_values;
        let top_level = self.top_level;
        self.key.as_ref().ok_or_else(|| Error::MapKeyMissing).and_then(move |key| value.serialize(ValueSerializer {
            writer: writer,
            key: key,
            top_level: top_level,
            allow_values: allow_values,
        }))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
        value.serialize(ValueSerializer {
            writer: &mut self.writer,
            key: key,
            top_level: self.top_level,
            allow_values: &mut self.allow_values,
        })
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

pub fn to_writer<W: Write, T: Serialize + ?Sized>(writer: W, value: &T) -> Result<()> {
    let mut ser = Serializer::new(Writer::new(writer, Default::default()));

    value.serialize(&mut ser)
}

pub fn to_vec<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value).map(|_| writer)
}

pub fn to_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    let vec = to_vec(value)?;

    // does not emit invalid utf8
    Ok(unsafe { String::from_utf8_unchecked(vec) })
}
