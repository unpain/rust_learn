use wasm_bindgen::prelude::*;

//计算两个数相加
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn main() {
    println!("{}", add(1, 2))
}
