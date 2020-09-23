use config::{ParseOptions, RenderOptions};
use parser::{Doc, JsNode, Node};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
pub mod config;
pub mod parser;
fn create_instance() -> Doc<'static> {
  Doc::new()
}

#[wasm_bindgen(catch)]
pub fn parse(content: &str, options: JsValue) -> Result<JsValue, JsValue> {
  let mut doc = create_instance();
  let options: ParseOptions = match options.into_serde() {
    Err(e) => return Err(JsValue::from_str(&e.to_string())),
    Ok(options) => options,
  };
  match doc.parse(content, options) {
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
  let root: JsNode = match tree.into_serde() {
    Err(e) => return Err(JsValue::from_str(&e.to_string())),
    Ok(node) => node,
  };
  let root = Node::from(root);
  let options: RenderOptions = match options.into_serde() {
    Err(e) => return Err(JsValue::from_str(&e.to_string())),
    Ok(options) => options,
  };
  Ok(JsValue::from_str(
    Doc::render_js_tree(Rc::new(RefCell::new(root)), &options).as_str(),
  ))
}
