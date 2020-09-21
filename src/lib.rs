use parser::Doc;
use wasm_bindgen::prelude::*;
pub mod config;
pub mod parser;
fn create_instance() -> Doc<'static> {
  Doc::new()
}

#[wasm_bindgen(catch)]
pub fn parse(content: &str) -> Result<JsValue, JsValue> {
  let mut doc = create_instance();
  match doc.parse(content) {
    Ok(_) => {}
    Err(e) => {
      return Err(JsValue::from_str(&e.to_string()));
    }
  }
  doc.to_json();
  match JsValue::from_serde(&doc.root) {
    Ok(result) => Ok(result),
    Err(e) => Err(JsValue::from_str(&e.to_string())),
  }
}
