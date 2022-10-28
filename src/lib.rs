mod utils;

use rand::Rng;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

extern "C" {
    fn sum(a: u8, b: u8) -> u8;

    fn add_to_vector_and_sum(element: u32) -> u32;
}

#[wasm_bindgen]
pub fn greet() {
    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen();
    log(&format!(
        "Hello, hello-wasm ass! {} + 4 is {}",
        random,
        unsafe { sum(random, 4) }
    ));
    log(&format!("sum of [1, 2, 3, 4, {random}] is {}", unsafe {
        add_to_vector_and_sum(random as u32)
    }));
}
