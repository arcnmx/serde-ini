#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_ini;

use serde::Deserialize;
use serde_ini::{Deserializer, Parser};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
enum TestModel {
    Person {
        name: String,
    }
}

const TEST_INPUT: &'static str = "
[Person]
name=Ana

[Person]
name=Box
";

fn expected() -> Vec<TestModel> {
    vec![
        TestModel::Person {
            name: "Ana".into(),
        },
        TestModel::Person {
            name: "Box".into(),
        },
    ]
}

#[test]
fn enum_seq_de() {
    assert_eq!(expected(), serde_ini::from_str::<Vec<TestModel>>(TEST_INPUT).unwrap());
}

#[test]
#[ignore = "not yet implemented"]
fn enum_seq_en() {
    let model = expected();

    let data = serde_ini::to_vec(&model).unwrap();

    assert_eq!(model, serde_ini::from_read::<_, Vec<TestModel>>(&data[..]).unwrap());
}
