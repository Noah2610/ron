extern crate ron;
#[macro_use]
extern crate serde;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct UnitStruct;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct NewType(f32);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct TupleStruct(UnitStruct, i8);

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct Key(u32);

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum Enum {
    Unit,
    Bool(bool),
    Chars(char, String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Struct {
    tuple: ((), NewType, TupleStruct),
    vec: Vec<Option<UnitStruct>>,
    map: HashMap<Key, Enum>,
}

#[test]
fn roundtrip() {
    let value = Struct {
        tuple: ((), NewType(0.5), TupleStruct(UnitStruct, -5)),
        vec: vec![None, Some(UnitStruct)],
        map: vec![
            (Key(5), Enum::Unit),
            (Key(6), Enum::Bool(false)),
            (Key(7), Enum::Bool(true)),
            (Key(9), Enum::Chars('x', "".to_string())),
        ].into_iter()
            .collect(),
    };

    let serial = ron::ser::to_string(&value).unwrap();

    println!("Serialized: {}", serial);

    let deserial = ron::de::from_str(&serial);

    assert_eq!(Ok(value), deserial);
}

#[test]
fn roundtrip_pretty() {
    let value = Struct {
        tuple: ((), NewType(0.5), TupleStruct(UnitStruct, -5)),
        vec: vec![None, Some(UnitStruct)],
        map: vec![
            (Key(5), Enum::Unit),
            (Key(6), Enum::Bool(false)),
            (Key(7), Enum::Bool(true)),
            (Key(9), Enum::Chars('x', "".to_string())),
        ].into_iter()
            .collect(),
    };

    let pretty = ron::ser::PrettyConfig {
        enumerate_arrays: true,
        ..Default::default()
    };
    let serial = ron::ser::to_string_pretty(&value, pretty).unwrap();

    println!("Serialized: {}", serial);

    let deserial = ron::de::from_str(&serial);

    assert_eq!(Ok(value), deserial);
}

#[test]
fn roundtrip_sep_tuple_members() {
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub enum FileOrMem {
        File(String),
        Memory,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Both {
        a: Struct,
        b: FileOrMem,
    }

    let a = Struct {
        tuple: ((), NewType(0.5), TupleStruct(UnitStruct, -5)),
        vec: vec![None, Some(UnitStruct)],
        map: vec![
            (Key(5), Enum::Unit),
            (Key(6), Enum::Bool(false)),
            (Key(7), Enum::Bool(true)),
            (Key(9), Enum::Chars('x', "".to_string())),
        ].into_iter()
            .collect(),
    };
    let b = FileOrMem::File("foo".to_owned());

    let value = Both {
        a,
        b,
    };

    let pretty = ron::ser::PrettyConfig {
        depth_limit: !0,
        new_line: "\n".to_owned(),
        indentor: "    ".to_owned(),
        separate_tuple_members: true,
        enumerate_arrays: false,
    };
    let serial = ron::ser::to_string_pretty(&value, pretty).unwrap();

    println!("Serialized: {}", serial);

    let deserial = ron::de::from_str(&serial);

    assert_eq!(Ok(value), deserial);
}
