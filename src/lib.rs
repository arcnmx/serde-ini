//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/serde-ini/")]

//! Windows INI format serialization for serde

extern crate result;
extern crate void;
#[macro_use]
extern crate serde;

pub mod de;
pub mod ser;

pub mod parse;
pub mod write;

pub use de::Deserializer;
pub use ser::Serializer;
pub use parse::{Parser, Item};
pub use write::{Writer, LineEnding};
