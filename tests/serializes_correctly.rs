use std::collections::HashMap;

use dyson::{Json, JsonPretty};
use quickcheck::Arbitrary;
use quickcheck_macros::quickcheck;
use serde_derive::Serialize;

#[quickcheck]
fn serializes_correctly(complex: Complex) {
    let json = Json(&complex);
    let json_pretty = JsonPretty(&complex);

    assert_eq!(json.to_string(), serde_json::to_string(&complex).unwrap());
    assert_eq!(
        json_pretty.to_string(),
        serde_json::to_string_pretty(&complex).unwrap()
    );
}

#[derive(Serialize, Debug, Clone)]
pub enum SomeEnum {
    A(f32),
    B(u8),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    some_float: f64,
    some_map: HashMap<u8, SomeEnum>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Complex {
    name: String,
    properties: Vec<Properties>,
}

impl Arbitrary for SomeEnum {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // Consider that `true` generates `SomeEnum::A`
        // The unwrap is OK since `choose` only returns None on empty slices
        let is_a = *g.choose(&[true, false]).unwrap();

        if is_a {
            SomeEnum::A(Arbitrary::arbitrary(g))
        } else {
            SomeEnum::B(Arbitrary::arbitrary(g))
        }
    }
}

impl Arbitrary for Properties {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            some_float: f64::arbitrary(g),
            some_map: HashMap::arbitrary(g),
        }
    }
}

impl Arbitrary for Complex {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            name: Arbitrary::arbitrary(g),
            properties: Vec::arbitrary(g),
        }
    }
}
