#![allow(unused)]
use serde::Serialize;
use std::marker::PhantomData;
use uint::construct_uint;

trait Encode {
    fn encode<T: Serialize>(val: T) -> String;
}

struct User<T: Encode> {
    name: String,
    age: u32,
    _marker: PhantomData<T>,
}

struct Json;

impl Encode for Json {
    fn encode<T: Serialize>(val: T) -> String {
        serde_json::to_string(&val).unwrap()
    }
}

construct_uint! {
    pub struct U256(4);
}

fn main() {
    let target = U256([0xFF000000, 0, 0, 7]); // Difficulty target
    let block_hash = U256([0x00000000, 0xDEADBEEF, 0, 8]);

    if block_hash <= target {
        println!("Block is valid!");
    } else {
        println!("Block is invalid!");
    }
}
