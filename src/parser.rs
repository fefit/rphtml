use crate::config::{ParseOptions, RenderOptions};
use crate::util::{is_char_available_in_key, is_char_available_in_value, is_identity};

use htmlentity::entity::{decode_chars, EncodeType, Entity};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::rc::{Rc, Weak};
use thiserror::Error;
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
/*
* constants
*/
const TAG_BEGIN_CHAR: char = '<';
const TAG_END_CHAR: char = '>';
const ALLOC_CHAR_CAPACITY: usize = 50;
const ALLOC_NODES_CAPACITY: usize = 20;

pub type RenderEncoderOption = Box<dyn Fn(char) -> Option<EncodeType>>;
#[derive(Debug)]
pub struct ParseError {
	pub position: CodePosAt,
	pub kind: ErrorKind,
}

impl ParseError {
	pub fn new(kind: ErrorKind, position: CodePosAt) -> Box<Self> {
		Box::new(ParseError { position, kind })
	}
}

// display parse error
impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let output = format!("{}{}", self.position, self.kind);
		f.write_str(output.as_str())
	}
}

// impl trait Error
impl Error for ParseError {}

// create parse error
fn create_parse_error(kind: ErrorKind, position: CodePosAt) -> HResult {
	let err = ParseError::new(kind, position);
	Err(err)
}

// make uuid
fn make_uuid() -> String {
	Uuid::new_v4().to_string()
}
#[derive(Error, Debug)]
pub enum ErrorKind {
	#[error("wrong tag <{0}")]
	WrongTag(String),
	#[error("wrong end tag </{0}")]
	WrongEndTag(String),
	#[error("wrong child tag '<{1}' in tag '{}'")]
	ChildInSpecialTag(String, char),
	#[error("unmatched tag '</{0}>'")]
	UnmatchedClosedTag(String),
	#[error("unexpected character '{0}'")]
	UnexpectedCharacter(char),
	#[error("the tag '{0}' is not closed")]
	UnclosedTag(String),
	#[error("the tag's attribute should split by spaces,wrong character '{0}'")]
	NoSpaceBetweenAttr(char),
	#[error("unexpected character '{0}' in html doctype decalaration")]
	WrongHtmlDoctype(char),
	#[error("unrecognized tag '{0}', do you mean '{1}'")]
	UnrecognizedTag(String, String),
	#[error("wrong tag name '{0}'")]
	WrongTagIdentity(String),
	#[error("not allowed text '{0}' in root node")]
	WrongRootTextNode(String),
	#[error("because the config of 'case-sensitive', the tag '{0}' is not matched correctly.")]
	WrongCaseSensitive(String),
	#[error("wrong self-closing tag '{0}',make sure you set the config allow self-closing.")]
	WrongSelfClosing(String),
	#[error("{0}")]
	CommonError(String),
}

#[derive(PartialEq, Eq, Hash)]
pub enum DetectChar {
	Comment,
	DOCTYPE,
	XMLCDATA,
}

lazy_static! {
	static ref DETECT_CHAR_MAP: HashMap<DetectChar, Vec<char>> = {
		use DetectChar::*;
		let mut map = HashMap::new();
		map.insert(Comment, vec!['-', '-']);
		map.insert(DOCTYPE, vec!['D', 'O', 'C', 'T', 'Y', 'P', 'E']);
		map.insert(XMLCDATA, vec!['[', 'C', 'D', 'A', 'T', 'A', '[']);
		map
	};
	static ref VOID_ELEMENTS: Vec<&'static str> = vec![
		"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
		"track", "wbr"
	];
	static ref SPECIAL_TAG_MAP: HashMap<&'static str, SpecialTag> = {
		use SpecialTag::*;
		let mut map = HashMap::new();
		map.insert("pre", Pre);
		map.insert("svg", Svg);
		map.insert("math", MathML);
		map
	};
	static ref MUST_QUOTE_ATTR_CHARS: Vec<char> = vec!['"', '\'', '`', '=', '<', '>'];
}

fn is_void_tag(name: &str) -> bool {
	VOID_ELEMENTS.contains(&name)
}

pub fn allow_insert(name: &str, node_type: NodeType) -> bool {
	if is_void_tag(name) {
		return false;
	}
	use NodeType::*;
	if let Some(special) = SPECIAL_TAG_MAP.get(name) {
		let code_in = match node_type {
			AbstractRoot => CodeTypeIn::AbstractRoot,
			HTMLDOCTYPE => CodeTypeIn::HTMLDOCTYPE,
			Comment => CodeTypeIn::Comment,
			Text | SpacesBetweenTag => CodeTypeIn::TextNode,
			TagEnd => CodeTypeIn::TagEnd,
			XMLCDATA => CodeTypeIn::XMLCDATA,
			Tag => match name {
				"script" => CodeTypeIn::HTMLScript,
				"style" => CodeTypeIn::HTMLStyle,
				_ => CodeTypeIn::Tag,
			},
		};
		return special
			.is_ok(&code_in, name, ' ', CodePosAt::default())
			.is_ok();
	}
	match name {
		"title" | "textarea" => node_type == Text || node_type == SpacesBetweenTag,
		_ => true,
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(PartialEq, Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NodeType {
	AbstractRoot = 0,     // abstract root node
	HTMLDOCTYPE = 1,      // html doctype
	Comment = 2,          // comment
	Text = 3,             // text node
	SpacesBetweenTag = 4, // spaces between tag
	Tag = 5,              // the start tag\self-closing tag\autofix empty tag
	TagEnd = 6,           // the end tag node
	XMLCDATA = 7,         // XML CDATA, IN SVG OR MATHML
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodeTypeIn {
	AbstractRoot,      // abstract root node,the begin node of document
	Unkown,            // wait for detect node
	UnkownTag,         // is a tag begin with '<', but need more diagnosis
	Tag,               // the start tag\self-closing tag\autofix empty tag
	TagEnd,            // the end tag
	ExclamationBegin,  // tag begin with '!' maybe Comment|HTMLDOCTYPE
	Comment,           // comment tag
	HTMLDOCTYPE,       // html doctype
	EscapeableRawText, // escapeable raw text, <title> and <textarea>
	HTMLScript,        // html script
	HTMLStyle,         // html style
	XMLCDATA,          // XMLCDATA section
	TextNode,          // text node
}

fn get_content(content: &Option<Vec<char>>) -> String {
	match content {
		Some(content) => content.iter().collect::<String>(),
		_ => String::from(""),
	}
}

fn get_content_decode(content: &Option<Vec<char>>) -> String {
	match content {
		Some(content) => decode_chars(content).iter().collect::<String>(),
		_ => String::from(""),
	}
}
/**
 * the doc's position
*/
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct CodePosAt {
	pub line_no: usize,
	pub col_no: usize,
	pub index: usize,
}

impl fmt::Display for CodePosAt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let output = format!(
			"[line:{},col:{},index:{}]",
			self.line_no, self.col_no, self.index
		);
		f.write_str(output.as_str())
	}
}

impl CodePosAt {
	// new
	pub fn new(line_no: usize, col_no: usize, index: usize) -> Self {
		CodePosAt {
			line_no,
			col_no,
			index,
		}
	}
	// create a begin position
	pub fn begin() -> Self {
		CodePosAt::new(1, 0, 0)
	}
	// jump to new line
	pub fn set_new_line(&mut self) {
		self.line_no += 1;
		self.col_no = 0;
	}
	// move to next col
	pub fn move_one(&mut self) {
		self.col_no += 1;
		self.index += 1;
	}
	// get the next col position
	pub fn next_col(&self) -> Self {
		CodePosAt {
			line_no: self.line_no,
			col_no: self.col_no + 1,
			index: self.index + 1,
		}
	}
}

/**
 * Attr
 * attribute data
 * if value is None, it's a boolean attribute
 * if key is None,it's a value with quote
 */

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Attr {
	pub key: Option<AttrData>,
	pub value: Option<AttrData>,
	pub quote: Option<char>,
	pub need_quote: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AttrData {
	pub content: String,
	pub begin_at: CodePosAt,
	pub end_at: CodePosAt,
}

impl Attr {
	// build attribute code
	pub fn build(&self, remove_quote: bool) -> String {
		let mut ret = String::with_capacity(ALLOC_CHAR_CAPACITY);
		let mut has_key = false;
		if let Some(AttrData { content, .. }) = &self.key {
			ret.push_str(content);
			has_key = true;
		}
		if let Some(AttrData { content, .. }) = &self.value {
			if has_key {
				ret.push('=');
			}
			let mut use_quote: Option<char> = None;
			if let Some(quote) = self.quote {
				if self.need_quote || !remove_quote {
					ret.push(quote);
					use_quote = Some(quote);
				}
			}
			ret.push_str(content);
			if let Some(quote) = use_quote {
				ret.push(quote);
			}
		}
		ret
	}
	// check if the char need quote
	pub fn need_quoted_char(ch: &char) -> bool {
		ch.is_ascii_whitespace() || MUST_QUOTE_ATTR_CHARS.contains(ch)
	}
	// check if id
	pub fn check_if_id(&self) -> Option<String> {
		if let Some(key) = &self.key {
			if key.content.to_ascii_lowercase() == "id" {
				if let Some(value) = &self.value {
					return Some(value.content.trim().into());
				}
			}
		}
		None
	}
}
/**
 * Tag
 * is_end: if the tag end with '>'
 * self_closed: if the tag is self-closing '/>'
 * auto_fix: if the tag either self-closing nor closed with a end tag, may auto fix by the parser
 * name: the tag name
 * attrs: the attribute list
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct TagMeta {
	#[serde(skip)]
	prev_is_key: bool,

	#[serde(skip)]
	is_in_kv: bool,
	#[serde(skip)]
	is_in_translate: bool,

	#[serde(skip)]
	tag_in: TagCodeIn,

	#[serde(skip)]
	is_end: bool,
	pub self_closed: bool,
	pub auto_fix: bool,
	pub name: String,
	pub attrs: Vec<Attr>,
}

impl TagMeta {
	pub fn get_name(&self, lowercase: bool) -> String {
		if lowercase {
			self.name.to_ascii_lowercase()
		} else {
			self.name.clone()
		}
	}
	pub fn attrs_to_string(&self, remove_quote: bool) -> String {
		let segs: Vec<String> = self
			.attrs
			.iter()
			.map(|attr| attr.build(remove_quote))
			.collect();
		if !segs.is_empty() {
			format!(" {}", segs.join(" "))
		} else {
			String::from("")
		}
	}
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum TagCodeIn {
	Wait,
	Key,
	Value,
	DoubleQuotedValue,
	SingleQuotedValue,
}

impl Default for TagCodeIn {
	fn default() -> Self {
		Self::Wait
	}
}

pub type RefNode = Rc<RefCell<Node>>;
#[derive(Default, Clone)]
struct RenderStatus {
	inner_type: Option<RenderStatuInnerType>,
	is_in_pre: bool,
}
#[derive(Clone)]
enum RenderStatuInnerType {
	Html,
	Text,
}
/**
 *
 */
#[derive(Serialize, Deserialize)]
pub struct Node {
	// if a tag node, add a index to the node
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uuid: Option<String>,

	// the node's depth in the document
	pub depth: usize,

	// the node's type
	pub node_type: NodeType,

	// the node's start position '<'
	pub begin_at: CodePosAt,

	// the node's end position '>'
	pub end_at: CodePosAt,

	// the end tag </xx> of the tag node
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_tag: Option<RefNode>,

	// parent node, use weak reference,prevent reference loop
	#[serde(skip_serializing, skip_deserializing)]
	pub parent: Option<Weak<RefCell<Node>>>,

	// root
	#[serde(skip_serializing, skip_deserializing)]
	pub root: Option<Rc<RefCell<RootNode>>>,

	// the content,for text/comment/style/script nodes
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<char>>,

	// the child nodes
	#[serde(skip_serializing_if = "Option::is_none")]
	pub childs: Option<Vec<RefNode>>,

	// the tag node meta information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<RefCell<TagMeta>>,

	// special information
	#[serde(skip_serializing_if = "Option::is_none")]
	pub special: Option<SpecialTag>,
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Node")
			.field("uuid", &self.uuid)
			.field("node_type", &self.node_type)
			.field("begin_at", &self.begin_at)
			.field("end_at", &self.end_at)
			.field("content", &self.content)
			.field("childs", &self.childs)
			.field("meta", &self.meta)
			.field("special", &self.special)
			.field("end_tag", &self.end_tag)
			.field("parent", &self.parent.is_some())
			.field("root", &self.root.is_some())
			.finish()
	}
}

impl Node {
	// create a new node
	pub fn new(node_type: NodeType, code_at: CodePosAt) -> Self {
		Node {
			node_type,
			begin_at: code_at,
			end_at: code_at,
			end_tag: None,
			parent: None,
			content: None,
			childs: None,
			meta: None,
			uuid: None,
			depth: 0,
			special: None,
			root: None,
		}
	}
	// build node
	fn build_node(&self, options: &RenderOptions, status: &mut RenderStatus) -> String {
		let mut result = String::with_capacity(5);
		let is_in_pre = status.is_in_pre;
		let inner_type = status.inner_type.as_ref();
		let is_inner = inner_type.is_some();
		use NodeType::*;
		match self.node_type {
			Text | SpacesBetweenTag => {
				if !is_in_pre && options.minify_spaces {
					if self.node_type == SpacesBetweenTag {
						// spaces between tag,just remove it
					} else {
						let mut prev_is_space = false;
						// check if need decode
						let mut content: Vec<char> = Vec::with_capacity(5);
						for &c in self.content.as_ref().unwrap().iter() {
							if c.is_ascii_whitespace() {
								if prev_is_space {
									continue;
								}
								prev_is_space = true;
								content.push(' ');
							} else {
								prev_is_space = false;
								content.push(c);
							}
						}
						// when need decode, the content should append to result
						if !content.is_empty() {
							if options.decode_entity {
								let decode_content = get_content_decode(&Some(content));
								result.push_str(&decode_content);
							} else {
								result.push_str(&content.iter().collect::<String>());
							}
						}
					}
				} else {
					// when has content
					if self.content.is_some() {
						let content = if options.decode_entity {
							get_content_decode(&self.content)
						} else {
							get_content(&self.content)
						};
						result.push_str(content.as_str());
					}
				}
			}
			Tag => {
				let meta = self
					.meta
					.as_ref()
					.expect("tag's meta data must have.")
					.borrow();
				let tag_name = meta.get_name(options.lowercase_tagname);
				// check if is in pre, only check if not in pre
				status.is_in_pre = is_in_pre || {
					if options.lowercase_tagname {
						tag_name == "pre"
					} else {
						tag_name.to_lowercase() == "pre"
					}
				};
				if !is_inner {
					let attrs = meta.attrs_to_string(options.remove_attr_quote);
					let tag = format!("<{}{}", tag_name, attrs);
					result.push_str(tag.as_str());
					// add self closing
					if meta.self_closed || (meta.auto_fix && options.always_close_void) {
						result.push_str(" /");
					} else if meta.auto_fix && self.end_tag.is_none() {
						result.push(TAG_END_CHAR);
						result.push_str(format!("</{}", tag_name).as_str());
					}
					result.push(TAG_END_CHAR);
				}
				// content for some special tags, such as style/script
				if self.content.is_some() {
					result.push_str(get_content(&self.content).as_str());
				}
			}
			TagEnd => {
				let mut content = get_content(&self.content);
				if options.remove_endtag_space {
					content = content.trim_end().to_string();
				}
				if options.lowercase_tagname {
					content = content.to_lowercase();
					if is_in_pre && content == "pre" {
						status.is_in_pre = false;
					}
				} else if is_in_pre && content.to_lowercase() == "pre" {
					status.is_in_pre = false;
				}
				if !is_inner {
					content = format!("</{}>", content);
					result.push_str(content.as_str());
				}
			}
			HTMLDOCTYPE => {
				let meta = self
					.meta
					.as_ref()
					.expect("tag's meta data must have.")
					.borrow();
				let content = format!(
					"<!{}{}>",
					meta.name,
					meta.attrs_to_string(options.remove_attr_quote)
				);
				result.push_str(content.as_str());
			}
			Comment => {
				if !options.remove_comment {
					// comment
					let comment = format!("<!--{}-->", get_content(&self.content));
					result.push_str(comment.as_str());
				}
			}
			XMLCDATA => {
				// cdata
				let content = format!("<![CDATA[{}]]>", get_content(&self.content));
				result.push_str(content.as_str());
			}
			_ => {}
		}
		result
	}
	// build node tree
	fn build_tree(&self, options: &RenderOptions, status: &mut RenderStatus) -> String {
		let mut result = String::with_capacity(ALLOC_CHAR_CAPACITY);
		let content = self.build_node(options, status);
		result.push_str(content.as_str());
		if let Some(childs) = &self.childs {
			for child in childs {
				let mut sub_status = status.clone();
				if let Some(RenderStatuInnerType::Html) = sub_status.inner_type {
					// get inner html
					sub_status.inner_type = None;
				}
				let child = child.borrow();
				let content = child.build_tree(options, &mut sub_status);
				result.push_str(content.as_str());
			}
		}
		if let Some(end_tag) = &self.end_tag {
			let content = end_tag.borrow().build_node(options, status);
			result.push_str(content.as_str());
		}
		result
	}
	// build
	pub fn build(&self, options: &RenderOptions, inner_text: bool) -> String {
		let inner_type = if inner_text {
			// wrong render options when call inner_text
			if options.inner_html {
				panic!("The 'inner_html' render option can't set true when 'inner_text' is true");
			}
			Some(RenderStatuInnerType::Text)
		} else if options.inner_html {
			Some(RenderStatuInnerType::Html)
		} else {
			None
		};
		let throw_wrong_node = |node_type: &NodeType| -> ! {
			panic!(
				"`inner_html` should only used for tag node, but found '{:?}'",
				node_type
			);
		};
		let status = &mut RenderStatus {
			inner_type,
			..Default::default()
		};
		// inner_html or inner_text
		if status.inner_type.is_some() {
			if self.node_type == NodeType::AbstractRoot {
				// inner html for abstract root
				if let Some(childs) = &self.childs {
					let mut finded = false;
					let mut child_node: Option<Rc<RefCell<Node>>> = None;
					for child in childs {
						if child.borrow().node_type == NodeType::Tag {
							if finded {
								panic!(
								"`inner_html` can't used in abstract root node which has multiple tag node childs."
							);
							}
							child_node = Some(Rc::clone(&child));
							finded = true;
						}
					}
					if let Some(child_node) = child_node {
						return child_node.borrow().build_tree(options, status);
					}
					// no tag child node finded
					throw_wrong_node(&childs[childs.len() - 1].borrow().node_type);
				}
				// abstract without any child
				return String::from("");
			}
			if self.node_type != NodeType::Tag {
				throw_wrong_node(&self.node_type);
			}
		}
		self.build_tree(options, status)
	}
	// check if alone tag
	pub fn is_alone_tag(&self) -> bool {
		if let Some(childs) = &self.childs {
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
			self.node_type == NodeType::Tag
		}
	}
	// append a child
	// is document node
	pub fn is_document(&self) -> (bool, bool) {
		let mut is_document = false;
		let mut syntax_ok = true;
		use NodeType::*;
		if self.node_type == AbstractRoot {
			if let Some(childs) = &self.childs {
				let mut find_html = false;
				for child in childs {
					let child_node = child.borrow();
					match child_node.node_type {
						Comment | SpacesBetweenTag => {
							// allowed in document
						}
						Tag => {
							if find_html {
								syntax_ok = false;
							} else {
								// check if is html tag
								if let Some(meta) = &self.meta {
									if meta.borrow().get_name(true) == "html" {
										find_html = true;
										is_document = true;
									} else {
										syntax_ok = false;
									}
								} else {
									syntax_ok = false;
								}
							}
						}
						_ => {
							syntax_ok = false;
						}
					}
				}
			}
		}
		if is_document {
			return (is_document, syntax_ok);
		}
		(false, true)
	}
	// check if two RefNode is same
	pub fn is_same(cur: &RefNode, other: &RefNode) -> bool {
		std::ptr::eq(cur.as_ptr() as *const _, other.as_ptr() as *const _)
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq)]
pub enum SpecialTag {
	Pre,
	MathML,
	Svg,
	Template,
}

impl SpecialTag {
	pub fn is_ok(
		&self,
		code_in: &CodeTypeIn,
		tag_name: &str,
		c: char,
		position: CodePosAt,
	) -> HResult {
		use CodeTypeIn::*;
		use SpecialTag::*;
		match code_in {
			Unkown | UnkownTag | TagEnd | ExclamationBegin | Comment => {
				return Ok(());
			}
			_ => {}
		};
		match self {
			Pre => {
				let message = format!(
					"the tag '{}' can only contains text node, wrong '{:?}' at {:?}",
					tag_name, code_in, position
				);
				if code_in != &TextNode {
					return create_parse_error(ErrorKind::CommonError(message), position);
				}
			}
			Svg | MathML => {
				match code_in {
					Tag | XMLCDATA => {}
					HTMLScript | HTMLStyle if self == &Svg => {}
					TextNode if c.is_ascii_whitespace() => {}
					_ => {
						let message = format!(
							"the tag '{}' can only contains sub tags, find node '{:?}' at {:?}",
							tag_name, code_in, position
						);
						return create_parse_error(ErrorKind::CommonError(message), position);
					}
				};
			}
			Template => {}
		};
		Ok(())
	}
}

pub type HResult = Result<(), Box<dyn Error>>;
type NextHandle = fn(&mut Doc, char) -> HResult;

/*
* no operation, just placeholder for initialize doc
*/
fn noop(_d: &mut Doc, _c: char) -> HResult {
	Ok(())
}

/*
 * code_in: TextNode | Unkown | AbstractRoot
*/
fn parse_wait_and_text(doc: &mut Doc, c: char) -> HResult {
	use CodeTypeIn::*;
	match c {
		// match the tag start '<'
		TAG_BEGIN_CHAR => {
			if doc.code_in == TextNode {
				// finish entity
				if doc.in_entity {
					doc.end_entity();
				}
				let content = doc.clean_chars_return();
				doc.current_node.borrow_mut().content = Some(content);
				doc.check_textnode = if doc.repeat_whitespace {
					Some(Rc::clone(&doc.current_node))
				} else {
					None
				};
				doc.set_tag_end_info();
			}
			doc.mem_position = doc.position;
			doc.set_code_in(UnkownTag);
		}
		_ => {
			// only whitespace allowed
			/* if cur_depth == 1 && !c.is_ascii_whitespace() {
				return create_parse_error(
					ErrorKind::WrongRootTextNode(c.to_string()),
					doc.position,
				);
			} */
			if doc.code_in != TextNode {
				// new text node
				doc.add_new_node(Rc::new(RefCell::new(Node::new(
					NodeType::Text,
					doc.position,
				))));
				doc.code_in = TextNode;
				doc.prev_chars.clear();
				doc.repeat_whitespace = c.is_ascii_whitespace();
			} else {
				doc.repeat_whitespace = doc.repeat_whitespace && c.is_ascii_whitespace();
			}
			doc.prev_chars.push(c);
		}
	}
	Ok(())
}

/**
 * code_in: Tag | HTMLDOCTYPE
 */
fn parse_tag_or_doctype(doc: &mut Doc, c: char) -> HResult {
	use CodeTypeIn::*;
	let mut is_self_closing = false;
	let mut tag_name: String = String::from("");
	// void elements
	let mut is_void_element = false;
	let mut is_node_end = false;
	let mut id_name: Option<String> = None;
	{
		let mut current_node = doc.current_node.borrow_mut();
		match current_node.meta.as_mut() {
			Some(meta) => {
				// meta is initial
				use TagCodeIn::*;
				let mut meta = meta.borrow_mut();
				match meta.tag_in {
					Wait | Key | Value => {
						let tag_in_wait = meta.tag_in == Wait;
						let tag_in_key = meta.tag_in == Key;
						let mut is_end_key_or_value = false;
						// tag in wait, if prev char is '/', the current char must be the end of tag
						if tag_in_wait && doc.prev_char == '/' && c != TAG_END_CHAR {
							return doc.error(ErrorKind::UnexpectedCharacter(c));
						}
						if c.is_ascii_whitespace() {
							// if tag in wait state, ignore whitespaces, otherwise, is an end of key or value
							if !tag_in_wait {
								is_end_key_or_value = true;
							}
						} else if c == TAG_END_CHAR {
							meta.is_end = true;
							is_void_element = is_void_tag(&meta.name.to_lowercase().as_str());
							// self-closing tags
							if tag_in_wait {
								if doc.prev_char == '/' {
									if is_void_element {
										// void element allow self closing
										if doc.parse_options.case_sensitive_tagname
											&& meta.name.to_lowercase() != meta.name
										{
											return doc.error(ErrorKind::WrongCaseSensitive(meta.name.clone()));
										}
									// void element has pop before.
									} else {
										if !doc.parse_options.allow_self_closing {
											// sub element in Svg or MathML allow self-closing
											match doc.check_special() {
												Some(SpecialTag::Svg) | Some(SpecialTag::MathML) => {
													// is in svg or mathml
												}
												_ => {
													return doc.error(ErrorKind::WrongSelfClosing(meta.name.clone()));
												}
											}
										}
										// not void element, but allow self-closing or in <svg/math>, pop from chain nodes
										doc.chain_nodes.pop();
									}
									// set self closing
									is_self_closing = true;
									meta.self_closed = true;
								}
							} else {
								is_end_key_or_value = true;
							}
							// tag end
							is_node_end = true;
							// save tag name
							if doc.code_in == Tag {
								tag_name = meta.name.clone();
							}
						} else {
							match c {
								'"' | '\'' if tag_in_wait => {
									// if not in kv, quoted value should have spaces before.
									if !meta.is_in_kv {
										if !doc.prev_char.is_ascii_whitespace() {
											return doc.error(ErrorKind::NoSpaceBetweenAttr(c));
										}
										// add new value-only attribute
										meta.attrs.push(Default::default());
									} else {
										// meta is now in value of 'key=value'
										meta.is_in_kv = false;
									}
									// reset previous state
									meta.prev_is_key = false;
									meta.tag_in = if c == '"' {
										DoubleQuotedValue
									} else {
										SingleQuotedValue
									};
									doc.mem_position = doc.position;
									doc.prev_chars.clear();
								}
								'/' => {
									if doc.code_in != Tag {
										return doc.error(ErrorKind::WrongTag(String::from("/")));
									}
									if meta.tag_in == Value {
										// value allow string with slash '/'
										doc.prev_chars.push(c);
									} else {
										if !tag_in_wait {
											is_end_key_or_value = true;
										}
										meta.tag_in = Wait;
									}
								}
								'=' => {
									if meta.prev_is_key {
										meta.is_in_kv = true;
									} else {
										return doc.error(ErrorKind::WrongTag(String::from("=")));
									}
									// end the key or value
									meta.tag_in = Wait;
									is_end_key_or_value = true;
								}
								_ => {
									if tag_in_wait {
										doc.prev_chars.clear();
										if meta.is_in_kv {
											meta.tag_in = Value;
											meta.is_in_kv = false;
											meta.prev_is_key = false;
										} else {
											meta.tag_in = Key;
											// move attribute index
											meta.prev_is_key = true;
											meta.attrs.push(Default::default());
										}
										doc.mem_position = doc.position;
									} else {
										// check if key or value is ok
										if tag_in_key {
											if !is_char_available_in_key(&c) {
												return doc.error(ErrorKind::CommonError("wrong key character".into()));
											}
										} else if !is_char_available_in_value(&c) {
											return doc.error(ErrorKind::CommonError("wrong value character".into()));
										}
									}
									doc.prev_chars.push(c);
								}
							}
						}
						if is_end_key_or_value {
							// if end of the key or value
							let cur_attr = meta.attrs.last_mut().expect("the attr must have");
							let value = doc.chars_to_string();
							if tag_in_key {
								let attr_data = doc.make_attr_data(value);
								cur_attr.key = Some(attr_data);
							} else {
								let attr_data = doc.make_attr_data(value);
								cur_attr.value = Some(attr_data);
								// check if id tag
								id_name = cur_attr.check_if_id();
							}
							doc.prev_chars.clear();
							meta.tag_in = Wait;
						}
					}
					DoubleQuotedValue | SingleQuotedValue => {
						let is_in_translate = meta.is_in_translate;
						if is_in_translate {
							meta.is_in_translate = false;
							doc.prev_chars.push(c);
						} else if (meta.tag_in == DoubleQuotedValue && c == '"')
							|| (meta.tag_in == SingleQuotedValue && c == '\'')
						{
							meta.tag_in = Wait;
							let cur_attr = meta.attrs.last_mut().expect("current attr must have");
							cur_attr.quote = Some(c);
							cur_attr.value = Some(doc.make_attr_data(doc.chars_to_string()));
							// check if id tag
							id_name = cur_attr.check_if_id();
							doc.prev_chars.clear();
						} else {
							let is_tran_slash = c == '\\';
							if is_tran_slash {
								meta.is_in_translate = true;
							}
							let cur_attr = meta.attrs.last_mut().expect("current attr must have");
							if !cur_attr.need_quote {
								// need quote characters
								if is_tran_slash || Attr::need_quoted_char(&c) {
									cur_attr.need_quote = true;
								}
							}
							doc.prev_chars.push(c);
						}
					}
				}
			}
			None => {
				let is_whitespace = c.is_ascii_whitespace();
				if is_whitespace || c == TAG_END_CHAR || c == '/' {
					let cur_tag_name: String = doc.chars_to_string();
					if is_whitespace {
						// tag name ended
						if doc.code_in == HTMLDOCTYPE && cur_tag_name.to_ascii_uppercase() != "DOCTYPE" {
							return doc.error(ErrorKind::WrongHtmlDoctype(c));
						}
					} else {
						match doc.code_in {
							HTMLDOCTYPE => {
								// html doctype without any attribute
								return doc.error(ErrorKind::WrongHtmlDoctype(c));
							}
							Tag => {
								tag_name = cur_tag_name.clone();
								// tag end
								is_node_end = c == TAG_END_CHAR;
								// check if void element
								is_void_element = VOID_ELEMENTS.contains(&cur_tag_name.to_lowercase().as_str())
							}
							_ => unreachable!("just detect code in HTMLDOCTYPE and TAG"),
						}
					}
					if !is_identity(&doc.prev_chars) {
						return doc.error(ErrorKind::WrongTagIdentity(cur_tag_name));
					}
					let meta = TagMeta {
						name: cur_tag_name,
						attrs: Vec::with_capacity(5),
						auto_fix: false,
						self_closed: false,
						tag_in: TagCodeIn::Wait,
						prev_is_key: false,
						is_end: false,
						is_in_kv: false,
						is_in_translate: false,
					};
					current_node.meta = Some(RefCell::new(meta));
				} else {
					doc.prev_chars.push(c);
				}
			}
		}
	}
	// add id tag to tags
	if let Some(name) = id_name {
		doc
			.root
			.borrow_mut()
			.id_tags
			.borrow_mut()
			.insert(name, Rc::clone(&doc.current_node));
	}
	if is_node_end {
		doc.set_tag_end_info();
		if doc.code_in == Tag {
			let is_in_svg = doc
				.check_special()
				.map_or(false, |special| special == SpecialTag::Svg);
			match tag_name.to_lowercase().as_str() {
				name @ "script" | name @ "style" | name @ "title" | name @ "textarea" if !is_in_svg => {
					doc.mem_position = doc.position;
					let code_in = match name {
						"script" => HTMLScript,
						"style" => HTMLStyle,
						_ => EscapeableRawText,
					};
					doc.set_code_in(code_in);
					// set detect chars
					let mut next_chars = vec!['<', '/'];
					let tag_chars: Vec<_> = tag_name.chars().collect();
					next_chars.extend(tag_chars);
					doc.detect = Some(next_chars);
				}
				name => {
					if doc.in_special.is_none() && !is_self_closing {
						doc.in_special = if let Some(&special) = SPECIAL_TAG_MAP.get(name) {
							Some((special, Box::leak(tag_name.into_boxed_str())))
						} else {
							None
						}
					}
					doc.set_code_in(Unkown);
				}
			}
			// void elements
			if is_void_element {
				doc.chain_nodes.pop();
			}
			// reset chars
			doc.prev_chars.clear();
		} else {
			doc.set_code_in(Unkown);
		}
	}
	Ok(())
}

/**
 * code_in: TagEnd
 */
fn parse_tagend(doc: &mut Doc, c: char) -> HResult {
	use CodeTypeIn::*;
	// the end tag
	if c == TAG_END_CHAR {
		let end_tag_name = doc.chars_to_string();
		let fix_end_tag_name = end_tag_name.trim_end().to_lowercase();
		let iter = doc.chain_nodes.iter().rev();
		let mut back_num: usize = 0;
		let max_back_num: usize = if doc.parse_options.auto_fix_endtag {
			doc.chain_nodes.len() - 1
		} else {
			0
		};
		let is_allow_fix = max_back_num > 0;
		let mut empty_closed_tags: Vec<RefNode> = vec![];
		let mut real_tag_node: Option<RefNode> = None;
		for node in iter {
			if let Some(meta) = &node.borrow().meta {
				let tag_name = &meta.borrow().name;
				let is_equal = tag_name == &end_tag_name;
				if is_equal || (tag_name.to_lowercase() == fix_end_tag_name) {
					if doc.parse_options.case_sensitive_tagname && !is_equal {
						return doc.error(ErrorKind::WrongCaseSensitive(tag_name.clone()));
					}
					real_tag_node = Some(Rc::clone(node));
					break;
				}
				if is_allow_fix {
					empty_closed_tags.push(Rc::clone(node));
				}
			}
			back_num += 1;
			if back_num >= max_back_num {
				break;
			}
		}
		// find the nearest tag
		if let Some(tag) = &real_tag_node {
			// set end tag for the tag node
			tag.borrow_mut().end_tag = Some(Rc::clone(&doc.current_node));
			let is_only_text_child = match &tag.borrow().childs {
				Some(childs) => childs.len() == 1 && childs[0].borrow().node_type == NodeType::Text,
				None => false,
			};
			if !is_only_text_child {
				doc.set_text_spaces_between();
			}
			// set node end
			doc.set_tag_end_info();
			// set code in
			doc.set_code_in(Unkown);
			// fix the empty tags
			if !empty_closed_tags.is_empty() {
				// reverse the tags, keep the childs order
				empty_closed_tags.reverse();
				doc.fix_unclosed_tag(empty_closed_tags, tag);
			}
			// set end tag more info
			let mut current_node = doc.current_node.borrow_mut();
			current_node.parent = Some(Rc::downgrade(&tag));
			current_node.depth = tag.borrow().depth;
			current_node.content = Some(end_tag_name.chars().collect());
			// end of special tag
			if doc.in_special.is_some() && doc.in_special.unwrap().1.to_lowercase() == fix_end_tag_name {
				doc.in_special = None;
			}
			doc.prev_chars.clear();
			// remove the matched tag from the chain nodes
			doc
				.chain_nodes
				.truncate(doc.chain_nodes.len() - back_num - 1);
		} else {
			if !doc.parse_options.auto_remove_nostart_endtag {
				return create_parse_error(
					ErrorKind::WrongEndTag(end_tag_name),
					doc.current_node.borrow().begin_at,
				);
			}
			// auto remove the unmatched endtag
			doc.prev_chars.clear();
			doc.set_code_in(Unkown);
		}
	} else {
		doc.prev_chars.push(c);
	}
	Ok(())
}

/**
 * code_in: HTMLScript | HTMLStyle | EscapeableRawText
 */
fn parse_special_tag(doc: &mut Doc, c: char) -> HResult {
	use CodeTypeIn::*;
	let end_tag = doc
		.detect
		.as_ref()
		.expect("detect chars must set before set_code_in.");
	let total_len = end_tag.len();
	let mut chars_len = doc.prev_chars.len();
	let mut is_matched = false;
	// parse html script tag and style tag
	match c {
		TAG_BEGIN_CHAR => {
			doc.mem_position = doc.position;
		}
		TAG_END_CHAR
			if (chars_len == total_len && !doc.prev_char.is_ascii_whitespace())
				|| chars_len > total_len =>
		{
			let mut matched_num = 0;
			loop {
				let prev_char = doc.prev_chars[chars_len - 1];
				// ignore end whitespace
				if prev_char.is_ascii_whitespace() {
					if matched_num != 0 {
						break;
					}
				} else {
					let target_char = end_tag[total_len - matched_num - 1];
					if (doc.parse_options.case_sensitive_tagname && prev_char != target_char)
						|| prev_char.to_ascii_lowercase() != target_char.to_ascii_lowercase()
					{
						break;
					}
					matched_num += 1;
				}
				chars_len -= 1;
				if chars_len == 0 || matched_num == total_len {
					break;
				}
			}
			if matched_num == total_len {
				is_matched = true;
				// set code in unkown
				doc.set_code_in(Unkown);
				// set end
				let end_at = doc.mem_position;
				// find the matched
				let end_tag_name = doc.prev_chars.split_off(chars_len).split_off(2);
				// add an end tag
				let mut end = Node::new(NodeType::TagEnd, end_at);
				end.end_at = doc.position.next_col();
				end.content = Some(end_tag_name);
				end.depth = doc.chain_nodes.len() - 1;
				end.parent = Some(Rc::downgrade(&doc.current_node));
				// set tag node's content, end_tag
				let node = Rc::new(RefCell::new(end));
				let mut current_node = doc.current_node.borrow_mut();
				current_node.end_tag = Some(Rc::clone(&node));
				let content = doc.prev_chars.clone();
				current_node.content = Some(content);
				doc.prev_chars.clear();
				// remove current tag
				doc.chain_nodes.pop();
				doc.detect = None;
			}
		}
		_ => {}
	}
	if !is_matched {
		doc.prev_chars.push(c);
	}
	Ok(())
}

/**
 * code_in: Comment|CDATA
 */
fn parse_comment_or_cdata(doc: &mut Doc, c: char) -> HResult {
	use CodeTypeIn::*;
	// comment node
	let end_symbol: char = if doc.code_in == Comment { '-' } else { ']' };
	if c == TAG_END_CHAR && doc.prev_char == end_symbol && doc.prev_chars.len() >= 2 {
		let total_len = doc.prev_chars.len();
		let last_index = total_len - 2;
		let prev_last_char = doc.prev_chars[last_index];
		if prev_last_char == end_symbol {
			let mut content = doc.clean_chars_return();
			content.truncate(last_index);
			doc.current_node.borrow_mut().content = Some(content);
			doc.set_tag_end_info();
			doc.set_code_in(Unkown);
		} else {
			doc.prev_chars.push(c);
		}
	} else {
		doc.prev_chars.push(c);
	}
	Ok(())
}

/**
 * code_in: UnkownTag
 */
fn parse_unkown_tag(doc: &mut Doc, c: char) -> HResult {
	use CodeTypeIn::*;
	// check the tag type
	match c {
		'a'..='z' | 'A'..='Z' => {
			// new tag node, add tag_index
			let mut inner_node = Node::new(NodeType::Tag, doc.mem_position);
			let uuid = make_uuid();
			inner_node.uuid = Some(uuid.clone());
			let node = Rc::new(RefCell::new(inner_node));
			doc
				.root
				.borrow_mut()
				.tags
				.borrow_mut()
				.insert(uuid, Rc::clone(&node));
			doc.add_new_node(node);
			doc.set_text_spaces_between();
			doc.set_code_in(Tag);
			doc.prev_chars.push(c);
		}
		'/' => {
			// tag end
			doc.add_new_node(Rc::new(RefCell::new(Node::new(
				NodeType::TagEnd,
				doc.mem_position,
			))));
			doc.set_code_in(TagEnd);
		}
		'!' => {
			// Comment|DOCTYPE
			doc.set_code_in(ExclamationBegin);
		}
		_ => {
			return create_parse_error(ErrorKind::WrongTag(c.to_string()), doc.mem_position);
		}
	};
	Ok(())
}

/**
 * code_in: ExclamationBegin
 */
fn parse_exclamation_begin(doc: &mut Doc, c: char) -> HResult {
	use CodeTypeIn::*;
	// maybe Comment | DOCTYPE<HTML> | XMLCDATA
	let mut ignore_char = false;
	if let Some(next_chars) = &doc.detect {
		let total_len = doc.prev_chars.len();
		let actual_len = next_chars.len();
		if total_len < actual_len {
			let cur_should_be = next_chars.get(total_len).unwrap();
			if cur_should_be == &c.to_ascii_uppercase() {
				if total_len == actual_len - 1 {
					let begin_at = doc.mem_position;
					match c {
						'-' | '[' => {
							let code_in: CodeTypeIn;
							let node_type: NodeType;
							if c == '-' {
								code_in = Comment;
								// new comment node
								node_type = NodeType::Comment;
							} else {
								code_in = XMLCDATA;
								// new html doctype node
								node_type = NodeType::XMLCDATA;
							}
							doc.set_code_in(code_in);
							// new comment node
							doc.add_new_node(Rc::new(RefCell::new(Node::new(node_type, begin_at))));
							doc.prev_chars.clear();
							ignore_char = true;
							doc.set_text_spaces_between();
						}
						'E' | 'e' => {
							doc.set_code_in(HTMLDOCTYPE);
							// new html doctype node
							doc.add_new_node(Rc::new(RefCell::new(Node::new(
								NodeType::HTMLDOCTYPE,
								begin_at,
							))));
							doc.set_text_spaces_between();
						}
						_ => unreachable!(),
					};
					doc.detect = None;
				}
			} else {
				return create_parse_error(
					ErrorKind::UnrecognizedTag(doc.chars_to_string(), next_chars.iter().collect::<String>()),
					doc.mem_position,
				);
			}
		}
	} else {
		let detect_type: DetectChar;
		match c {
			'-' => {
				detect_type = DetectChar::Comment;
			}
			'D' | 'd' => {
				detect_type = DetectChar::DOCTYPE;
			}
			'[' => {
				// CDATA
				let special_tag_name = doc.in_special.map(|(_, name)| name);
				if let Some(tag_name) = special_tag_name {
					if tag_name
						== doc
							.chain_nodes
							.last()
							.unwrap()
							.borrow()
							.meta
							.as_ref()
							.expect("chain nodes must tag node")
							.borrow()
							.name
							.as_str()
					{
						return create_parse_error(
							ErrorKind::CommonError("<![CDATA tag can in sub node".into()),
							doc.position,
						);
					} else {
						detect_type = DetectChar::XMLCDATA;
					}
				} else {
					return create_parse_error(
						ErrorKind::CommonError("wrong <![CDATA tag can only used in Svg or MathML".into()),
						doc.position,
					);
				}
			}
			_ => {
				return create_parse_error(ErrorKind::WrongTag(doc.chars_to_string()), doc.mem_position);
			}
		};
		doc.detect = Some(DETECT_CHAR_MAP.get(&detect_type).unwrap().to_vec());
	}
	if !ignore_char {
		doc.prev_chars.push(c);
	}
	Ok(())
}

/**
 * Doc
 * the html syntax: https://www.w3.org/TR/2011/WD-html-markup-20110113/syntax.html
*/
pub struct Doc {
	code_in: CodeTypeIn,
	position: CodePosAt,
	mem_position: CodePosAt,
	detect: Option<Vec<char>>,
	prev_chars: Vec<char>,
	prev_char: char,
	chain_nodes: Vec<RefNode>,
	current_node: RefNode,
	in_special: Option<(SpecialTag, &'static str)>,
	repeat_whitespace: bool,
	in_entity: bool,
	entity: Entity,
	check_textnode: Option<RefNode>,
	handle: NextHandle,
	pub total_chars: usize,
	pub parse_options: ParseOptions,
	pub root: Rc<RefCell<RootNode>>,
}

pub type StringNodeMap = HashMap<String, RefNode>;
#[derive(Serialize, Deserialize)]
pub struct RootNode {
	pub node: Option<RefNode>,
	pub id_tags: Rc<RefCell<StringNodeMap>>,
	pub tags: Rc<RefCell<StringNodeMap>>,
}

impl RootNode {
	pub fn get_node(&self) -> RefNode {
		Rc::clone(&self.node.as_ref().unwrap())
	}
	fn get_element(map: &StringNodeMap, query: &str) -> Option<RefNode> {
		map.get(query).map(|node| Rc::clone(node))
	}
	pub fn get_element_by_id(&self, id: &str) -> Option<RefNode> {
		RootNode::get_element(&self.id_tags.borrow(), id)
	}
	pub fn get_element_by_uuid(&self, uuid: &str) -> Option<RefNode> {
		RootNode::get_element(&self.tags.borrow(), uuid)
	}
}

impl Doc {
	// create new parser
	fn new() -> Self {
		let node = Rc::new(RefCell::new(Node::new(
			NodeType::AbstractRoot,
			CodePosAt::begin(),
		)));
		let ref_node = Rc::clone(&node);
		let current_node = Rc::clone(&node);
		let root_node = Rc::clone(&node);
		let root = RootNode {
			node: Some(root_node),
			id_tags: Rc::new(RefCell::new(HashMap::new())),
			tags: Rc::new(RefCell::new(HashMap::new())),
		};
		let mut nodes = Vec::with_capacity(ALLOC_NODES_CAPACITY);
		let mut chain_nodes = Vec::with_capacity(ALLOC_NODES_CAPACITY);
		// set root for root node
		node.borrow_mut().root = Some(Rc::new(RefCell::new(RootNode {
			node: None,
			id_tags: Rc::clone(&root.id_tags),
			tags: Rc::clone(&root.tags),
		})));
		node.borrow_mut().uuid = Some(make_uuid());
		nodes.push(node);
		chain_nodes.push(ref_node);
		let mut doc = Doc {
			code_in: CodeTypeIn::AbstractRoot,
			position: CodePosAt::begin(),
			mem_position: CodePosAt::begin(),
			prev_char: ' ',
			prev_chars: Vec::with_capacity(ALLOC_CHAR_CAPACITY),
			chain_nodes,
			current_node,
			total_chars: 0,
			detect: None,
			in_special: None,
			in_entity: false,
			entity: Entity::new(),
			root: Rc::new(RefCell::new(root)),
			parse_options: Default::default(),
			repeat_whitespace: false,
			check_textnode: None,
			handle: noop,
		};
		doc.init();
		doc
	}

	// init, set handle
	fn init(&mut self) {
		self.handle = parse_wait_and_text;
	}
	// for serde, remove cycle reference
	pub fn prepare_to_json(&mut self) {
		Doc::clear_cycle_ref(&self.get_root_node());
	}

	// clear cycle ref
	fn clear_cycle_ref(node: &RefNode) {
		node.borrow_mut().parent = None;
		if let Some(childs) = &node.borrow().childs {
			for child in childs {
				Doc::clear_cycle_ref(child);
			}
		}
	}

	// parse with string
	pub fn parse(content: &str, options: ParseOptions) -> Result<Self, Box<dyn Error>> {
		let mut doc = Doc::new();
		doc.parse_options = options;
		for c in content.chars() {
			doc.next(c)?;
		}
		doc.eof()?;
		Ok(doc)
	}

	// parse file
	pub fn parse_file<P>(filename: P, options: ParseOptions) -> Result<Self, Box<dyn Error>>
	where
		P: AsRef<Path>,
	{
		let file_path = filename.as_ref();
		let file_path = if file_path.is_absolute() {
			file_path.to_path_buf()
		} else {
			env::current_dir()?.join(filename).canonicalize().unwrap()
		};
		let file = File::open(file_path)?;
		let file = BufReader::new(file);
		let mut doc = Doc::new();
		doc.parse_options = options;
		for line in file.lines() {
			for c in line.unwrap().chars() {
				doc.next(c)?;
			}
			doc.next('\n')?;
		}
		doc.eof()?;
		Ok(doc)
	}
	// gather previous characters
	fn chars_to_string(&self) -> String {
		self.prev_chars.iter().collect::<String>()
	}
	// clean the previous characters and return
	fn clean_chars_return(&mut self) -> Vec<char> {
		let mut content: Vec<char> = Vec::with_capacity(self.prev_chars.len());
		content.append(&mut self.prev_chars);
		content
	}
	// finish entity
	fn end_entity(&mut self) {
		self.prev_chars.extend(self.entity.get_chars());
		self.in_entity = false;
		self.entity = Entity::new();
	}
	// set code_in
	fn set_code_in(&mut self, code_in: CodeTypeIn) {
		self.code_in = code_in;
		use CodeTypeIn::*;
		match code_in {
			TextNode | Unkown | AbstractRoot => {
				self.handle = parse_wait_and_text;
			}
			Tag | HTMLDOCTYPE => {
				self.handle = parse_tag_or_doctype;
			}
			HTMLScript | HTMLStyle | EscapeableRawText => {
				self.handle = parse_special_tag;
			}
			TagEnd => {
				self.handle = parse_tagend;
			}
			Comment | XMLCDATA => {
				self.handle = parse_comment_or_cdata;
			}
			UnkownTag => {
				self.handle = parse_unkown_tag;
			}
			ExclamationBegin => {
				self.handle = parse_exclamation_begin;
			}
		};
	}
	// read one char
	fn next(&mut self, c: char) -> HResult {
		let handle = self.handle;
		let _ = handle(self, c)?;
		/*
		 * do with code position
		 */
		let mut need_move_col = true;
		// \r newline in early macos
		if c == '\r' {
			self.position.set_new_line();
			need_move_col = false;
		} else if c == '\n' {
			// \n in windows, combine \r\n as newline
			if self.prev_char == '\r' {
				// do nothing, because did in \r
			} else {
				// set to nextline
				self.position.set_new_line();
			}
			need_move_col = false;
		}
		// move one col for the code position
		if need_move_col {
			self.position.move_one();
		} else {
			self.position.index += 1;
		}
		// check if special, and character is ok
		if let Some((special, tag_name)) = self.in_special {
			special.is_ok(&self.code_in, tag_name, c, self.position)?;
		}
		// set the previous char
		self.prev_char = c;
		// add total chars
		self.total_chars += 1;
		// check special
		// parse ok
		Ok(())
	}
	// add a new node to the queue
	fn add_new_node(&mut self, node: RefNode) {
		use NodeType::*;
		let node_type = node.borrow().node_type;
		if node_type != TagEnd {
			// set parent node
			let parent_node = self.chain_nodes.last().unwrap();
			node.borrow_mut().parent = Some(Rc::downgrade(parent_node));
			// add the node to parent's child list
			let mut parent_node = parent_node.borrow_mut();
			let child = Rc::clone(&node);
			if let Some(childs) = &mut parent_node.childs {
				childs.push(child);
			} else {
				parent_node.childs = Some(vec![child]);
			}
		}
		// set special
		node.borrow_mut().special = match self.in_special {
			Some((special, _)) => Some(special),
			None => None,
		};
		// set node root
		node.borrow_mut().root = Some(Rc::clone(&self.root));
		// set current node to be new node
		self.current_node = Rc::clone(&node);
		// if is a tag node, add the tag node to chain nodes
		if node_type == Tag {
			self.chain_nodes.push(Rc::clone(&node));
		}
		// add cur node to parent's child nodes
	}
	// set tag end info
	fn set_tag_end_info(&mut self) {
		use NodeType::*;
		let mut current_node = self.current_node.borrow_mut();
		let node_type = current_node.node_type;
		current_node.end_at = if node_type == Text {
			self.position
		} else {
			self.position.next_col()
		};
		// skip set depth for tag end
		if node_type != TagEnd {
			current_node.depth = self.chain_nodes.len() - 1;
		}
	}
	// set spaces between tag
	fn set_text_spaces_between(&mut self) {
		if let Some(text_node) = &mut self.check_textnode {
			text_node.borrow_mut().node_type = NodeType::SpacesBetweenTag;
			self.check_textnode = None;
		}
	}
	// make attr data
	fn make_attr_data(&self, content: String) -> AttrData {
		AttrData {
			content,
			begin_at: self.mem_position,
			end_at: self.position,
		}
	}
	// fix unclosed tag
	fn fix_unclosed_tag(&mut self, mut unclosed: Vec<RefNode>, parent: &RefNode) {
		let cur_depth = parent.borrow().depth + 1;
		for tag_node in unclosed.iter_mut() {
			let mut has_fixed = false;
			let mut tag_node = tag_node.borrow_mut();
			// set it's meta as auto fix
			if let Some(meta) = &tag_node.meta {
				let mut meta = meta.borrow_mut();
				if meta.auto_fix {
					has_fixed = true;
				} else {
					meta.auto_fix = true;
				}
			}
			if !has_fixed {
				// change the parent node
				tag_node.parent = Some(Rc::downgrade(parent));
				tag_node.depth = cur_depth;
			}
			// change all childs's parent and clear
			if let Some(childs) = &tag_node.childs {
				if !childs.is_empty() {
					for child_node in childs.iter() {
						if let Some(childs) = parent.borrow_mut().childs.as_mut() {
							childs.push(Rc::clone(child_node))
						};
						let mut child_node = child_node.borrow_mut();
						child_node.parent = Some(Rc::downgrade(parent));
						child_node.depth = cur_depth;
						if let Some(meta) = &child_node.meta {
							meta.borrow_mut().auto_fix = true;
						}
					}
				}
			}
			// clear childs
			tag_node.childs = None;
		}
	}
	// is in svg or mathml
	fn check_special(&self) -> Option<SpecialTag> {
		self.in_special.map(|(special, _)| special)
	}
	// end of the doc
	fn eof(&mut self) -> HResult {
		let cur_depth = self.chain_nodes.len();
		// check if all tags are closed correctly.
		if cur_depth > 1 {
			if !self.parse_options.auto_fix_endtag {
				let last_node = self.chain_nodes[cur_depth - 1].borrow();
				let begin_at = last_node.begin_at;
				let name = &last_node
					.meta
					.as_ref()
					.expect("tag node's meta must have")
					.borrow()
					.name;
				return create_parse_error(ErrorKind::UnclosedTag(name.to_owned()), begin_at);
			}
			// fix unclosed tags
			let unclosed = self.chain_nodes.split_off(1);
			let root_node = Rc::clone(&self.get_root_node());
			self.fix_unclosed_tag(unclosed, &root_node);
		}
		// check and fix last node info.
		use CodeTypeIn::*;
		if self.code_in == TextNode {
			let mut last_node = self.current_node.borrow_mut();
			last_node.depth = 1;
			last_node.content = Some(self.prev_chars.clone());
			if self.repeat_whitespace {
				last_node.node_type = NodeType::SpacesBetweenTag;
			}
			last_node.end_at = self.position;
		} else if self.code_in != Unkown && self.code_in != AbstractRoot {
			return create_parse_error(
				ErrorKind::UnclosedTag(format!("{:?}", self.code_in)),
				self.current_node.borrow().begin_at,
			);
		}
		// set the root node's end position
		self.get_root_node().borrow_mut().end_at = self.position;
		Ok(())
	}
	// return error with doc.position
	fn error(&self, kind: ErrorKind) -> HResult {
		create_parse_error(kind, self.position)
	}
	pub fn get_root_node(&self) -> RefNode {
		self.root.borrow().get_node()
	}
	// render
	pub fn render(&self, options: &RenderOptions) -> String {
		self.get_root_node().borrow_mut().build(options, false)
	}
	// render text
	pub fn render_text(&self, options: &RenderOptions) -> String {
		self.get_root_node().borrow_mut().build(options, true)
	}
	// render for js
	pub fn render_js_tree(tree: RefNode, options: &RenderOptions) -> String {
		tree.borrow().build(options, false)
	}
}
