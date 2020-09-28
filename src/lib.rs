use config::{IJsParseOptions, IJsRenderOptions, JsParseOptions, JsRenderOptions};
use parser::{Doc, IJsNode, Node};
use serde_json::{self, Error};
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
// export mod
pub mod config;
pub mod parser;

// create an instance
fn create_instance() -> Doc<'static> {
  Doc::new()
}

#[wasm_bindgen(catch)]
pub fn parse(content: &str, options: Option<IJsParseOptions>) -> Result<IJsNode, JsValue> {
  let mut doc = create_instance();
  let options: JsParseOptions = options.map_or(Default::default(), |options| options.into());
  doc
    .parse(content, options.into())
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
  doc.into_json();
  let result = doc.root.borrow_mut().clone().into();
  Ok(result)
}

#[wasm_bindgen(typescript_custom_section)]
const IJS_STRING: &'static str = r#"
export type IJsString = string;
"#;
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJsString")]
  pub type IJsString;
}

#[wasm_bindgen(catch)]
pub fn render(tree: IJsNode, options: Option<IJsRenderOptions>) -> Result<IJsString, JsValue> {
  // set hooks
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let root: Node = tree.into();
  let options: JsRenderOptions = options.map_or(Default::default(), |options| options.into());
  let result = Doc::render_js_tree(Rc::new(RefCell::new(root)), &options.into());
  Ok(JsValue::from_str(result.as_str()).into())
}
