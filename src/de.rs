use std::fmt::{self, Display};
use std::str::FromStr;
use std::mem::replace;
use std::{error, io, num, result, str};
use serde::de::{self, Deserialize, DeserializeOwned, DeserializeSeed, Visitor, MapAccess, IntoDeserializer};
use parse::{self, Item};

pub trait Trait {
    fn next(&mut self) -> Option<result::Result<Item, Error>>;
}

impl<E, T: Iterator<Item=result::Result<Item, E>>> Trait for T where Error: From<E> {
    fn next(&mut self) -> Option<result::Result<Item, Error>> {
        Iterator::next(self).map(|v| v.map_err(Into::into))
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Custom(String),
    UnexpectedEof,
    InvalidState,
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::Custom(e.to_string())
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::Custom(e.to_string())
    }
}

impl<E: error::Error> From<parse::Error<E>> for Error {
    fn from(e: parse::Error<E>) -> Self {
        Error::Custom(e.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "INI deserialization error"
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = result::Result<T, Error>;

enum PeekKind {
    Value,
    Section,
}

#[derive(Debug, Copy, Clone)]
enum Next<T> {
    Init,
    Eof,
    Some(T),
}

#[derive(Debug)]
pub struct Deserializer<T> {
    input: T,
    next: Next<Result<Item>>,
}

impl<T> Deserializer<T> {
    pub fn new(input: T) -> Self {
        Deserializer {
            input: input,
            next: Next::Init,
        }
    }
}

impl<T: Trait> Deserializer<T> {
    fn populate(&mut self) {
        while let Next::Init = self.next {
            let next = self.input.next();
            self.next = match next {
                Some(Ok(Item::Comment { .. })) => Next::Init,
                Some(Ok(Item::Empty)) => Next::Init,
                Some(v) => Next::Some(v),
                None => Next::Eof,
            };
        }
    }

    fn next_item(&mut self) -> Result<Item> {
        let next = match self.next {
            Next::Eof | Next::Some(Err(..)) => Next::Eof,
            _ => Next::Init,
        };
        let next = replace(&mut self.next, next);
        match next {
            Next::Some(v) => v,
            Next::Eof => Err(Error::UnexpectedEof),
            Next::Init => unreachable!(),
        }
    }

    fn peek_item(&mut self) -> Result<Option<&mut Item>> {
        match &mut self.next {
            &mut Next::Some(Ok(ref mut v)) => Ok(Some(v)),
            e @ &mut Next::Some(Err(..)) => {
                if let Next::Some(Err(e)) = replace(e, Next::Eof) {
                    Err(e)
                } else {
                    unreachable!()
                }
            },
            &mut Next::Eof => Ok(None),
            &mut Next::Init => unreachable!(),
        }
    }

    fn peek_kind(&mut self) -> Result<Option<PeekKind>> {
        self.populate();
        Ok(match self.peek_item()? {
            Some(&mut Item::Value { .. }) => Some(PeekKind::Value),
            Some(&mut Item::Section { .. }) => Some(PeekKind::Section),
            None => None,
            Some(..) => unreachable!(),
        })
    }

    fn next_key(&mut self) -> Result<String> {
        self.populate();
        match self.peek_item()? {
            Some(&mut Item::Value { ref mut key, .. }) => Ok(replace(key, String::new())),
            Some(..) => Err(Error::InvalidState),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn next_value(&mut self) -> Result<String> {
        self.populate();
        match self.next_item()? {
            Item::Value { value, .. } => Ok(value),
            _ => Err(Error::InvalidState),
        }
    }

    fn next_section(&mut self) -> Result<String> {
        self.populate();
        match self.peek_item()? {
            Some(&mut Item::Section { ref mut name }) => Ok(replace(name, String::new())),
            Some(..) => Err(Error::InvalidState),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn next_section_commit(&mut self) -> Result<String> {
        self.populate();
        match self.next_item()? {
            Item::Section { name } => Ok(name),
            _ => Err(Error::InvalidState),
        }
    }

    fn assert_eof(&mut self) -> Result<()> {
        self.populate();
        match self.peek_item()? {
            Some(..) => Err(Error::InvalidState),
            None => Ok(()),
        }
    }
}

impl<'de, 'a, T: Trait> de::Deserializer<'de> for &'a mut Deserializer<T> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(MapAccessTop(self))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
        map struct identifier ignored_any enum
    }
}

impl<R: io::BufRead> Deserializer<parse::Parser<io::Lines<R>>> {
    /// Creates an INI deserializer from an `io::BufRead`.
    pub fn from_bufread(reader: R) -> Self {
        Deserializer::new(parse::Parser::from_bufread(reader))
    }
}

impl<R: io::Read> Deserializer<parse::Parser<io::Lines<io::BufReader<R>>>> {
    /// Creates an INI deserializer from a reader.
    pub fn from_read(reader: R) -> Self {
        Deserializer::new(parse::Parser::from_read(reader))
    }
}

impl<'a> Deserializer<parse::Parser<parse::OkIter<str::Lines<'a>>>> {
    /// Creates an INI deserializer from a `&str`.
    pub fn from_str(s: &'a str) -> Self {
        Deserializer::new(parse::Parser::from_str(s))
    }
}

pub struct SectionDeserializer<'a, T: 'a>(&'a mut Deserializer<T>);

impl<'de, 'a, T: Trait> de::Deserializer<'de> for &'a mut SectionDeserializer<'a, T> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(MapAccessSection(self.0))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
        map struct identifier ignored_any enum
    }
}

pub struct ValueDeserializer<'a, T: 'a>(&'a mut Deserializer<T>);

impl<'de, 'a, T: Trait> de::Deserializer<'de> for &'a mut ValueDeserializer<'a, T> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match (self.0).peek_kind()? {
            Some(PeekKind::Value) => self.deserialize_str(visitor),
            None | Some(PeekKind::Section) => Err(Error::InvalidState),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(FromStr::from_str(&(self.0).next_value()?)?)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let value = (self.0).next_value()?;
        let mut chars = value.chars();
        if let Some(c) = chars.next() {
            if chars.next().is_some() {
                // >1 char long
                visitor.visit_str(&value)
            } else {
                visitor.visit_char(c)
            }
        } else {
            // 0 chars long
            visitor.visit_str(&value)
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_str(&(self.0).next_value()?)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string((self.0).next_value()?)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value> {
        match (self.0).peek_kind()? {
            Some(PeekKind::Value) => visitor.visit_enum((self.0).next_value()?.into_deserializer()),
            None | Some(PeekKind::Section) => Err(Error::InvalidState),
        }
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }
}

struct MapAccessTop<'a, T: Trait + 'a>(&'a mut Deserializer<T>);

impl<'de, 'a, T: Trait + 'a> MapAccess<'de> for MapAccessTop<'a, T> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match (self.0).peek_kind()? {
            Some(PeekKind::Value) => seed.deserialize((self.0).next_key()?.into_deserializer()),
            Some(PeekKind::Section) => seed.deserialize((self.0).next_section()?.into_deserializer()),
            None => return Ok(None),
        }.map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        match (self.0).peek_kind()? {
            Some(PeekKind::Value) => seed.deserialize(&mut ValueDeserializer(self.0)),
            Some(PeekKind::Section) => {
                (self.0).next_section_commit()?;
                seed.deserialize(&mut SectionDeserializer(self.0))
            },
            None => Err(Error::UnexpectedEof),
        }
    }
}

struct MapAccessSection<'a, T: Trait + 'a>(&'a mut Deserializer<T>);

impl<'de, 'a, T: Trait + 'a> MapAccess<'de> for MapAccessSection<'a, T> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match (self.0).peek_kind()? {
            Some(PeekKind::Value) => seed.deserialize((self.0).next_key()?.into_deserializer()).map(Some),
            None | Some(PeekKind::Section) => Ok(None),
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut ValueDeserializer(self.0))
    }
}

/*
struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de: de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        // The `deserialize_enum` method parsed a `{` character so we are
        // currently inside of a map. The seed will be deserializing itself from
        // the key of the map.
        let val = seed.deserialize(&mut *self.de)?;
        // Parse the colon separating map key from value.
        if (self.0).next_char()? == ':' {
            Ok((val, self))
        } else {
            Err(Error::ExpectedMapColon)
        }
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<()> {
        Err(Error::ExpectedString)
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(self.de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}*/

/// Deserialize an instance of type `T` from a string of INI text.
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    let mut de = Deserializer::new(parse::Parser::from_str(s.as_ref()));
    let value = Deserialize::deserialize(&mut de)?;

    de.assert_eof()?;
    Ok(value)
}

/// Deserialize an instance of type `T` from a buffered IO stream of INI.
pub fn from_bufread<R: io::BufRead, T: DeserializeOwned>(reader: R) -> Result<T> {
    let mut de = Deserializer::new(parse::Parser::from_bufread(reader));
    let value = Deserialize::deserialize(&mut de)?;

    de.assert_eof()?;
    Ok(value)
}

/// Deserialize an instance of type `T` from a stream of INI data.
pub fn from_read<R: io::Read, T: DeserializeOwned>(reader: R) -> Result<T> {
    let mut de = Deserializer::new(parse::Parser::from_read(reader));
    let value = Deserialize::deserialize(&mut de)?;

    de.assert_eof()?;
    Ok(value)
}
