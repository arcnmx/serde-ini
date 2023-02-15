#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_ini;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_ini::{Deserializer, Parser};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
enum TestModel {
    Person(BTreeMap<String, String>),
}

const TEST_INPUT: &'static str = "
[Person]
name=Ana
likes=pickles

[Person]
name=Fred
dislikes=Ana
";

fn expected() -> Vec<TestModel> {
    let mut ana = BTreeMap::new();

    ana.insert("name".into(), "Ana".into());
    ana.insert("likes".into(), "pickles".into());

    let mut fred = BTreeMap::new();

    fred.insert("name".into(), "Fred".into());
    fred.insert("dislikes".into(), "Ana".into());

    vec![
        TestModel::Person(ana),
        TestModel::Person(fred),
    ]
}

#[test]
fn enum_map_seq_de() {
    assert_eq!(expected(), serde_ini::from_str::<Vec<TestModel>>(TEST_INPUT).unwrap());
}

#[test]
fn enum_map_seq_en() {
    let model = expected();

    let data = serde_ini::to_vec(&model).unwrap();

    assert_eq!(model, serde_ini::from_read::<_, Vec<TestModel>>(&data[..]).unwrap());
}
