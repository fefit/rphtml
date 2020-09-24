use config::{IJsParseOptions, JsParseOptions, JsRenderOptions, RenderOptions};
use parser::{Doc, JsNode, Node};
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
  let options: JsParseOptions = options.map_or(Default::default(), |options| options);
  match doc.parse(content, options.into()) {
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
pub fn render(tree: JsValue, options: Option<JsRenderOptions>) -> Result<JsValue, JsValue> {
  // set hooks
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let root: JsNode = match tree.into_serde() {
    Err(e) => return Err(JsValue::from_str(&e.to_string())),
    Ok(node) => node,
  };
  let root = Node::from(root);
  let options: JsRenderOptions = options.map_or(Default::default(), |options| {
    let result = serde_json::to_string(&options).unwrap();
    serde_json::from_str(result.as_str()).unwrap()
  });
  Ok(JsValue::from_str(
    Doc::render_js_tree(Rc::new(RefCell::new(root)), &options.into()).as_str(),
  ))
}
