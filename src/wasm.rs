#![cfg(target_arch = "wasm32")]
use crate::parser::{Doc, NodeType, RefNode, RootNode};
use crate::wasm_config::{IJsParseOptions, IJsRenderOptions, JsParseOptions, JsRenderOptions};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
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
	pub root: Rc<RefCell<RootNode>>,
}

#[wasm_bindgen]
impl IJsNode {
	/* methods */
	#[wasm_bindgen(js_name = toJson)]
	pub fn to_json(&self) -> Result<IJsNodeTree, JsValue> {
		let result = JsValue::from_serde(&self.root.borrow().get_node())
			.map_err(|e| JsValue::from(&e.to_string()))?;
		Ok(result.into())
	}

	#[wasm_bindgen(js_name = toString)]
	pub fn to_string(&self) -> Result<IJsString, JsValue> {
		let result = serde_json::to_string(&self.root.borrow().get_node())
			.map_err(|e| JsValue::from(&e.to_string()))?;
		Ok(JsValue::from_str(&result).into())
	}

	#[wasm_bindgen]
	pub fn render(&self, options: Option<IJsRenderOptions>) -> Result<IJsString, JsValue> {
		let options: JsRenderOptions = options.map_or(Default::default(), |options| options.into());
		let result = Doc::render_js_tree(Rc::clone(&self.root.borrow().get_node()), &options.into());
		Ok(JsValue::from_str(result.as_str()).into())
	}

	#[wasm_bindgen(js_name = getTagByUuid)]
	pub fn get_tag_by_uuid(&self, uuid: &str) -> Result<Option<IJsNode>, JsValue> {
		Ok(
			self
				.root
				.borrow()
				.get_element_by_uuid(uuid)
				.map(|node| Rc::clone(&node).into()),
		)
	}

	#[wasm_bindgen(js_name = isAloneTag)]
	pub fn is_alone_tag(&self) -> bool {
		self.root.borrow().get_node().borrow().is_alone_tag()
	}
}

impl From<Doc> for IJsNode {
	fn from(doc: Doc) -> Self {
		IJsNode {
			root: Rc::clone(&doc.root),
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

impl From<Rc<RefCell<RootNode>>> for IJsNode {
	fn from(root: Rc<RefCell<RootNode>>) -> Self {
		IJsNode {
			root: Rc::clone(&root),
		}
	}
}

impl From<RefNode> for IJsNode {
	fn from(root: RefNode) -> Self {
		let mut tags: HashMap<String, RefNode> = HashMap::new();
		map_tags(Rc::clone(&root), &mut tags);
		let root = Rc::new(RefCell::new(RootNode {
			node: Some(Rc::clone(&root)),
			tags: Rc::new(RefCell::new(tags)),
			id_tags: Rc::new(RefCell::new(HashMap::new())),
			onerror: Rc::new(RefCell::new(None)),
		}));
		IJsNode { root }
	}
}
