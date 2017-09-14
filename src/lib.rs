//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/serde-ini/")]

//! Windows INI format serialization for serde

extern crate result;
#[macro_use]
extern crate serde;

/*mod de;
mod ser;
mod error;

pub use error::{Error, Result};*/

pub mod de;
pub mod ser;

pub mod parse;
pub mod write;

pub use de::Deserializer;
