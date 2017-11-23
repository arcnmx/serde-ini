//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/serde-ini/")]

//! Windows INI format serialization for serde

extern crate result;
extern crate void;
#[macro_use]
extern crate serde;

pub mod de;
pub mod error;
pub mod parse;
pub mod ser;
pub mod write;

pub use de::Deserializer;
pub use ser::Serializer;
pub use parse::{Parser, Item};
pub use write::{Writer, LineEnding};

use error::*;

/// Deserialize an instance of type `T` from a string of INI text.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
    where
    T: serde::Deserialize<'a>,
{
    let mut de = Deserializer::new(parse::Parser::from_str(s.as_ref()));
    let value = serde::Deserialize::deserialize(&mut de)?;

    // Make sure the whole stream has been consumed.
    de.end()?;
    Ok(value)
}

/// Deserialize an instance of type `T` from a buffered IO stream of INI.
pub fn from_bufread<'a, R, T>(reader: R) -> Result<T>
    where
    R: std::io::BufRead,
    T: serde::Deserialize<'a>,
{
    let mut de = Deserializer::new(parse::Parser::from_bufread(reader));
    let value = serde::Deserialize::deserialize(&mut de)?;

    // Make sure the whole stream has been consumed.
    de.end()?;
    Ok(value)

}
