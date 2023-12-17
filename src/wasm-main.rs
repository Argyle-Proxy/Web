use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str)
}

// Just a sample component
#[wasm_bindgen]
pub fn greet() {
  alert("Hi, there! We're glad you landed. If you're feeling comfortable, please consider pressing Ok.")
}
