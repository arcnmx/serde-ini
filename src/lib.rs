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

pub use de::{Deserializer, from_str, from_bufread, from_read};
pub use ser::{Serializer, to_string, to_vec, to_writer};
pub use parse::{Parser, Item};
pub use write::{Writer, LineEnding};
