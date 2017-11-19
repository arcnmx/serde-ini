#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_ini;

use serde::Deserialize;
use serde_ini::{Deserializer, Parser};

#[test]
fn test_de_basic_ok() {
    #[derive(Deserialize, Clone, PartialEq, Debug)]
    struct Model {
        key1: String,
        key2: u32,
    };

    let input = "
; Ignored comment
key1=value1
key2=255
";

    let expected = Model {
        key1: "value1".into(),
        key2: 255,
    };

    assert_eq!(expected.clone(), Model::deserialize(&mut Deserializer::new(Parser::from_str(input))).unwrap());

    assert_eq!(expected, Model::deserialize(&mut Deserializer::new(Parser::from_read(input.as_bytes()))).unwrap());
}
