use parser::{Doc, ParserType};
use wasm_bindgen::prelude::*;
pub mod config;
pub mod parser;
fn create_instance(xml: bool) -> Doc<'static> {
  let parser_type = if xml {
    ParserType::XML
  } else {
    ParserType::HTML
  };
  Doc::new(parser_type)
}

#[wasm_bindgen(catch)]
pub fn parse(content: &str, xml: bool) -> Result<JsValue, JsValue> {
  let mut doc = create_instance(xml);
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
