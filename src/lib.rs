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

pub use de::{from_bufread, from_read, from_str, Deserializer};
pub use parse::{Item, Parser};
pub use ser::{to_string, to_vec, to_writer, Serializer};
pub use write::{LineEnding, Writer};
