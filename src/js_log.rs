use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = "console")]
extern "C" {
  #[wasm_bindgen(js_name = log)]
  pub fn log(s: &str);
}
