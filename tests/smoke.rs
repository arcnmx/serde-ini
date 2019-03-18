#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_ini;

use serde::Deserialize;
use serde_ini::{Deserializer, Parser};

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
struct TestModel {
    key1: String,
    key2: u32,
    key3: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    map1: Option<Box<TestModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    map2: Option<Box<TestModel>>,
}

const TEST_INPUT: &'static str = "
; Ignored comment
key1=value1
key2=255
 key3 = value3

[map1]
key2=256
key1=value2
key3=

# We also treat hash as a comment character.
[map2]
key1=value3
key2=257
key3=
";

fn expected() -> TestModel {
    TestModel {
        key1: "value1".into(),
        key2: 255,
        key3: "value3".into(),
        map1: Some(Box::new(TestModel {
            key1: "value2".into(),
            key2: 256,
            .. Default::default()
        })),
        map2: Some(Box::new(TestModel {
            key1: "value3".into(),
            key2: 257,
            .. Default::default()
        })),
    }
}

#[test]
fn smoke_de() {

    // Parser
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::new(Parser::from_bufread(TEST_INPUT.as_bytes()))).unwrap());
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::new(Parser::from_read(TEST_INPUT.as_bytes()))).unwrap());
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::new(Parser::from_str(TEST_INPUT))).unwrap());

    // Deserializer
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::from_bufread(TEST_INPUT.as_bytes())).unwrap());
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::from_read(TEST_INPUT.as_bytes())).unwrap());
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::from_str(TEST_INPUT)).unwrap());

    // Static methods
    assert_eq!(expected(), serde_ini::from_bufread::<_, TestModel>(TEST_INPUT.as_bytes()).unwrap());
    assert_eq!(expected(), serde_ini::from_read::<_, TestModel>(TEST_INPUT.as_bytes()).unwrap());
    assert_eq!(expected(), serde_ini::from_str::<TestModel>(TEST_INPUT).unwrap());
}

#[test]
fn smoke_en() {
    let model = expected();

    let data = serde_ini::to_vec(&model).unwrap();

    assert_eq!(model, serde_ini::from_read::<_, TestModel>(&data[..]).unwrap());
}
