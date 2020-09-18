use parser::{Doc, ParserType};
use wasm_bindgen::prelude::*;
pub mod parser;

fn create_instance(xml: bool) -> Doc<'static> {
  let parser_type = if xml {
    ParserType::XML
  } else {
    ParserType::HTML
  };
  Doc::new(parser_type)
}

#[wasm_bindgen]
pub fn parse(content: &str, xml: bool) -> JsValue {
  let mut doc = create_instance(xml);
  let _ = doc.parse(content);
  doc.to_json();
  JsValue::from_serde(&doc.root).unwrap()
}

#[wasm_bindgen]
pub fn parse_file(file: &str, xml: bool) -> JsValue {
  let mut doc = create_instance(xml);
  let _ = doc.parse_file(file);
  doc.to_json();
  JsValue::from_serde(&doc.root).unwrap()
}
