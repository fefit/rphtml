use config::RenderOptions;
use parser::{Doc, Node};
use std::cell::RefCell;
use std::rc::Rc;
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
  doc.into_json();
  match JsValue::from_serde(&doc.root) {
    Ok(result) => Ok(result),
    Err(e) => Err(JsValue::from_str(&e.to_string())),
  }
}

#[wasm_bindgen(catch)]
pub fn render(tree: JsValue, options: JsValue) -> Result<JsValue, JsValue> {
  let root: Node<'_> = match tree.into_serde() {
    Err(e) => return Err(JsValue::from_str(&e.to_string())),
    Ok(node) => node,
  };
  let options: RenderOptions = match options.into_serde() {
    Err(e) => return Err(JsValue::from_str(&e.to_string())),
    Ok(options) => options,
  };
  let output = format!("root is: {:?}", root);
  Ok(JsValue::from_str(output.as_str()))
  /*Ok(JsValue::from_str(
    Doc::render_js_tree(Rc::new(RefCell::new(root)), &options).as_str(),
  ))*/
}
