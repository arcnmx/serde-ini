#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_ini;

use serde::{Deserialize, Serialize};
use serde_ini::{Deserializer, Serializer, Parser, Writer, LineEnding};

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
struct TestModel {
    key1: String,
    key2: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    map1: Option<Box<TestModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    map2: Option<Box<TestModel>>,
}

const TEST_INPUT: &'static str = "
; Ignored comment
key1=value1
key2=255

[map1]
key2=256
key1=value2

[map2]
key1=value3
key2=257
";

fn expected() -> TestModel {
    TestModel {
        key1: "value1".into(),
        key2: 255,
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
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::new(Parser::from_read(TEST_INPUT.as_bytes()))).unwrap());
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::new(Parser::from_str(&TEST_INPUT))).unwrap());

    // Deserializer
    let bufrd = std::io::BufReader::new(TEST_INPUT.as_bytes());
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::from_bufread(bufrd)).unwrap());
    assert_eq!(expected(), TestModel::deserialize(&mut Deserializer::from_str(&TEST_INPUT)).unwrap());

    // Static methods
    let bufrd = std::io::BufReader::new(TEST_INPUT.as_bytes());
    let de_bufrd: TestModel = serde_ini::from_bufread(bufrd).unwrap();
    let de_str: TestModel = serde_ini::from_str(&TEST_INPUT).unwrap();
    assert_eq!(expected(), de_bufrd);
    assert_eq!(expected(), de_str);
}

#[test]
fn smoke_en() {
    let model = expected();

    let mut data = Vec::<u8>::new();
    model.serialize(&mut Serializer::new(Writer::new(&mut data, LineEnding::default()))).unwrap();

    assert_eq!(model, TestModel::deserialize(&mut Deserializer::new(Parser::from_read(&data[..]))).unwrap());
}
