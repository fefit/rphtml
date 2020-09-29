use crate::config::{IJsParseOptions, IJsRenderOptions, JsParseOptions, JsRenderOptions};
use crate::parser::{Doc, RefNode};
use serde::{Deserialize, Serialize};
use serde_json;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const IJS_STRING: &'static str = r#"
export type IJsString = string;
"#;

#[wasm_bindgen(typescript_custom_section)]
const IJS_NODE_TREE: &'static str = r#"
export type IJsNodeTree = {
  tag_index: number;
  depth: number;
  node_type: NodeType;
  begin_at: CodePosAt;
  end_at: CodePosAt;
  end_tag?: IJsNodeTree;
  meta?: IJsNodeTagMeta;
  childs?: Array<IJsNodeTree>;
};
"#;

#[wasm_bindgen(typescript_custom_section)]
const IJS_NODE_ATTR_DATA: &'static str = r#"
export type IJsNodeAttrData = {
  content: string;
  begin_at: CodePosAt;
  end_at: CodePosAt;
};
"#;

#[wasm_bindgen(typescript_custom_section)]
const IJS_NODE_ATTR: &'static str = r#"
export type IJsNodeAttr = {
  key?: IJsNodeAttrData;
  value?: IJsNodeAttrData;
  quote: string;
  need_quote: boolean;
};
"#;

#[wasm_bindgen(typescript_custom_section)]
const IJS_NODE_TAG_META: &'static str = r#"
export type IJsNodeTagMeta = {
  self_closed: boolean;
  auto_fix: boolean;
  name: string;
  attrs: Array<IJsNodeAttr>;
};
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJsString")]
  pub type IJsString;
  #[wasm_bindgen(typescript_type = "IJsNodeTree")]
  pub type IJsNodeTree;
  #[wasm_bindgen(typescript_type = "IJsNodeTagMeta")]
  pub type IJsNodeTagMeta;
  #[wasm_bindgen(typescript_type = "IJsNodeAttr")]
  pub type IJsNodeAttr;
  #[wasm_bindgen(typescript_type = "IJsNodeAttrData")]
  pub type IJsNodeAttrData;
}

#[wasm_bindgen(catch)]
pub fn parse(content: &str, options: Option<IJsParseOptions>) -> Result<IJsNode, JsValue> {
  let options: JsParseOptions = options.map_or(Default::default(), |options| options.into());
  let mut doc =
    Doc::parse(content, options.into()).map_err(|e| JsValue::from_str(&e.to_string()))?;
  doc.into_json();
  let result = IJsNode::from(doc.root).into();
  Ok(result)
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IJsNode {
  #[wasm_bindgen(skip)]
  pub root: RefNode,
}

#[wasm_bindgen]
impl IJsNode {
  /* methods */
  #[wasm_bindgen(js_name = toJson)]
  pub fn to_json(&self) -> Result<IJsNodeTree, JsValue> {
    let result = JsValue::from_serde(&self.root).map_err(|e| JsValue::from(&e.to_string()))?;
    Ok(result.into())
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> Result<IJsString, JsValue> {
    let result = serde_json::to_string(&self.root).map_err(|e| JsValue::from(&e.to_string()))?;
    Ok(JsValue::from_str(&result).into())
  }

  #[wasm_bindgen]
  pub fn render(&self, options: Option<IJsRenderOptions>) -> Result<IJsString, JsValue> {
    let options: JsRenderOptions = options.map_or(Default::default(), |options| options.into());
    let result = Doc::render_js_tree(Rc::clone(&self.root), &options.into());
    Ok(JsValue::from_str(result.as_str()).into())
  }
}

impl From<IJsNode> for RefNode {
  fn from(node: IJsNode) -> Self {
    node.root
  }
}

impl From<RefNode> for IJsNode {
  fn from(node: RefNode) -> Self {
    IJsNode {
      root: Rc::clone(&node),
    }
  }
}
