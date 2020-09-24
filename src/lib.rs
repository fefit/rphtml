use config::{IJsParseOptions, IJsRenderOptions, JsParseOptions, JsRenderOptions};
use parser::{Doc, IJsNode, JsNode, Node};
use serde_json::Error;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
pub mod config;
pub mod parser;
fn create_instance() -> Doc<'static> {
  Doc::new()
}

#[wasm_bindgen(catch)]
pub fn parse(content: &str, options: Option<IJsParseOptions>) -> Result<JsValue, JsValue> {
  let mut doc = create_instance();
  let err_fn = |e: Error| JsValue::from_str(&e.to_string());
  let options: JsParseOptions = options.map_or(Default::default(), |options| options.into());
  doc
    .parse(content, options.into())
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
  doc.into_json();
  let result = JsValue::from_serde(&doc.root).map_err(err_fn)?;
  Ok(result)
}

#[wasm_bindgen(catch)]
pub fn render(tree: IJsNode, options: Option<IJsRenderOptions>) -> Result<JsValue, JsValue> {
  // set hooks
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let root: JsNode = tree
    .into_serde()
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
  let root: Node = Node::from(root);
  let options: JsRenderOptions = options.map_or(Default::default(), |options| options.into());
  Ok(JsValue::from_str(
    Doc::render_js_tree(Rc::new(RefCell::new(root)), &options.into()).as_str(),
  ))
}
