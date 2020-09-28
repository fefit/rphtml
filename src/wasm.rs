use crate::config::{IJsParseOptions, IJsRenderOptions, JsParseOptions, JsRenderOptions};
use crate::parser::{CodePosAt, Doc, Node, NodeType, RefNode, SpecialTag, TagMeta};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
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

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IJsNode {
  #[wasm_bindgen(skip)]
  pub tag_index: usize,

  #[wasm_bindgen(skip)]
  pub depth: usize,

  #[wasm_bindgen(skip)]
  pub node_type: NodeType,

  #[wasm_bindgen(skip)]
  pub begin_at: CodePosAt,

  #[wasm_bindgen(skip)]
  pub end_at: CodePosAt,

  #[wasm_bindgen(skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end_tag: Option<RefCell<Box<IJsNode>>>,

  #[wasm_bindgen(skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<Vec<char>>,

  #[wasm_bindgen(skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub childs: Option<Vec<RefCell<Box<IJsNode>>>>,

  #[wasm_bindgen(skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<RefCell<TagMeta>>,

  #[wasm_bindgen(skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub special: Option<SpecialTag>,
}

#[wasm_bindgen]
impl IJsNode {
  /* methods */
  #[wasm_bindgen(js_name = toJson)]
  pub fn to_json(&self) -> Result<IJsNodeTree, JsValue> {
    let result = JsValue::from_serde(self).map_err(|e| JsValue::from(&e.to_string()))?;
    Ok(result.into())
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> Result<IJsString, JsValue> {
    let result = serde_json::to_string(self).map_err(|e| JsValue::from(&e.to_string()))?;
    Ok(JsValue::from_str(&result).into())
  }

  #[wasm_bindgen]
  pub fn render(&self, options: Option<IJsRenderOptions>) -> Result<IJsString, JsValue> {
    let root: Node = self.clone().into();
    let options: JsRenderOptions = options.map_or(Default::default(), |options| options.into());
    let result = Doc::render_js_tree(Rc::new(RefCell::new(root)), &options.into());
    Ok(JsValue::from_str(result.as_str()).into())
  }
}

impl<'a> From<IJsNode> for Node<'a> {
  fn from(node: IJsNode) -> Self {
    let IJsNode {
      tag_index,
      depth,
      node_type,
      begin_at,
      end_at,
      end_tag,
      content,
      childs,
      meta,
      special,
    } = node;
    let end_tag = end_tag.map(|tag| {
      let tag = tag.into_inner();
      let last_tag = Node::from(*tag);
      Rc::new(RefCell::new(last_tag))
    });
    let childs: Option<Vec<RefNode<'_>>> = childs.map(|child| {
      child
        .into_iter()
        .map(|tag| {
          let tag = tag.into_inner();
          Rc::new(RefCell::new(Node::from(*tag)))
        })
        .collect()
    });
    Node {
      tag_index,
      depth,
      node_type,
      begin_at,
      end_at,
      content,
      meta,
      special,
      parent: None,
      end_tag,
      childs,
    }
  }
}

impl<'a> From<Node<'a>> for IJsNode {
  fn from(node: Node<'a>) -> Self {
    let Node {
      tag_index,
      depth,
      node_type,
      begin_at,
      end_at,
      end_tag,
      content,
      childs,
      meta,
      special,
      ..
    } = node;
    let end_tag = end_tag
      .as_ref()
      .map(|v| RefCell::new(Box::new(v.borrow_mut().clone().into())));
    let childs = childs.as_ref().map(|childs| {
      childs
        .into_iter()
        .map(|v| RefCell::new(Box::new(v.borrow_mut().clone().into())))
        .collect()
    });
    IJsNode {
      tag_index,
      depth,
      node_type,
      begin_at,
      end_at,
      end_tag,
      childs,
      meta,
      special,
      content,
    }
  }
}
