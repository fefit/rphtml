#![cfg(target_arch = "wasm32")]
use crate::config::RenderOptions;
use crate::parser::{Doc, NodeType, RefNode};
use crate::wasm_config::{IJsParseOptions, IJsRenderOptions, JsParseOptions, JsRenderOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const IJS_STRING: &'static str = r#"
export type IJsString = string;
"#;

#[wasm_bindgen(typescript_custom_section)]
const IJS_NODE_TREE: &'static str = r#"
export type IJsNodeTree = {
  uuid?: string;
  depth: number;
  node_type: number;
  begin_at: CodePosAt;
  end_at: CodePosAt;
  content?: Array<string>;
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

pub use htmlentity::entity::{EncodeType, EntitySet};
pub use htmlentity::wasm::contains;

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
  doc.prepare_to_json();
  let result = IJsNode::from(doc);
  Ok(result)
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct IJsNode {
  #[wasm_bindgen(skip)]
  pub root: RefNode,
  #[wasm_bindgen(skip)]
  pub tags: HashMap<String, RefNode>,
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

  #[wasm_bindgen(js_name = renderEncoder)]
  pub fn render_encoder(
    &self,
    encoder: &js_sys::Function,
    options: Option<IJsRenderOptions>,
  ) -> Result<IJsString, JsValue> {
    let options: JsRenderOptions = options.map_or(Default::default(), |options| options.into());
    let mut options: RenderOptions = options.into();
    let callback = Box::new(|ch: char| -> Option<EncodeType> {
      let value: u8 = encoder
        .call1(&JsValue::null(), &JsValue::from_str(&ch.to_string()))
        .ok()?
        .as_f64()
        .unwrap_or(0.0) as u8;
      let encode_type: EncodeType = value.into();
      match encode_type {
        EncodeType::Ignore => None,
        _ => Some(encode_type),
      }
    }) as Box<dyn Fn(char) -> Option<EncodeType>>;
    options.encoder = Some(callback);
    let result = Doc::render_js_tree(Rc::clone(&self.root), &options);
    Ok(JsValue::from_str(result.as_str()).into())
  }

  #[wasm_bindgen(js_name = getTagByUuid)]
  pub fn get_tag_by_uuid(&self, uuid: &str) -> Result<Option<IJsNode>, JsValue> {
    Ok(self.tags.get(uuid).map(|node| Rc::clone(&node).into()))
  }

  #[wasm_bindgen(js_name = isAloneTag)]
  pub fn is_alone_tag(&self) -> bool {
    let cur_node_type = self.root.borrow().node_type;
    if let Some(childs) = &self.root.borrow().childs {
      match childs.len() {
        1 => childs[0].borrow().node_type == NodeType::Text,
        num => {
          if num <= 3 {
            return childs.iter().all(|child| {
              let node_type = child.borrow().node_type;
              node_type == NodeType::SpacesBetweenTag || node_type == NodeType::Text
            });
          }
          false
        }
      }
    } else {
      cur_node_type == NodeType::Tag
    }
  }
}

impl From<Doc> for IJsNode {
  fn from(doc: Doc) -> Self {
    IJsNode {
      root: Rc::clone(&doc.root),
      tags: doc.tags,
    }
  }
}

fn map_tags(cur: RefNode, tags: &mut HashMap<String, RefNode>) {
  if let Some(childs) = &cur.borrow().childs {
    tags.insert(cur.borrow().uuid.as_ref().unwrap().clone(), Rc::clone(&cur));
    if !childs.is_empty() {
      for child in childs {
        map_tags(Rc::clone(&child), tags);
      }
    }
  }
}

impl From<RefNode> for IJsNode {
  fn from(root: RefNode) -> Self {
    let mut tags: HashMap<String, RefNode> = HashMap::new();
    map_tags(Rc::clone(&root), &mut tags);
    IJsNode {
      root: Rc::clone(&root),
      tags,
    }
  }
}
