#![allow(unused)]

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

// Generate definitions with typeshare using:
// typeshare ./ --lang=typescript --output-file=types.ts

#[typeshare]
struct GenericStruct<N, A> {
    name: N,
    age: A,
}

#[typeshare]
struct MyStruct {
    name: String,
    age: u32,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
enum MyEnum {
    Variant(bool),
    OtherVariant,
    Number(u32),
}
