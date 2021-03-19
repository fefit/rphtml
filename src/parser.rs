use crate::config::{ParseOptions, RenderOptions};
use crate::error::{ErrorKind, ParseError};
use crate::position::{CodeAt, CodeRegion};
use crate::types::{GenResult, HResult};
use htmlentity::entity::{decode_chars_to, encode_chars, EncodeType, Entity, EntitySet};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::{Rc, Weak};
use std::{
	cell::{Ref, RefCell},
	env,
	fs::File,
	io::prelude::*,
	io::BufReader,
	path::Path,
};

/*
* constants
*/
const TAG_BEGIN_CHAR: char = '<';
const TAG_END_CHAR: char = '>';
const EMPTY_CHAR: char = ' ';
const END_SLASH_CHAR: char = '/';
const EQUAL_CHAR: char = '=';
const DOUBLE_QUOTE_CHAR: char = '"';
const SINGLE_QUOTE_CHAR: char = '\'';
const EOF_CHAR: char = '\0';
const DASH_CHAR: char = '-';
const LEFT_BRACKET_CHAR: char = '[';
const RIGHT_BRACKET_CHAR: char = ']';
const ALLOC_CHAR_CAPACITY: usize = 50;
const ALLOC_NODES_CAPACITY: usize = 20;
// tagname's characters
const HTML_TAG_NAME: [char; 4] = ['h', 't', 'm', 'l'];
const PRE_TAG_NAME: [char; 3] = ['p', 'r', 'e'];
const SCRIPT_TAG_NAME: [char; 6] = ['s', 'c', 'r', 'i', 'p', 't'];
const STYLE_TAG_NAME: [char; 5] = ['s', 't', 'y', 'l', 'e'];
const TITLE_TAG_NAME: [char; 5] = ['t', 'i', 't', 'l', 'e'];
const TEXTAREA_TAG_NAME: [char; 8] = ['t', 'e', 'x', 't', 'a', 'r', 'e', 'a'];

lazy_static! {
	static ref DETECT_CHAR_MAP: HashMap<DetectChar, Vec<char>> = {
		use DetectChar::*;
		let mut map = HashMap::new();
		map.insert(Comment, vec![DASH_CHAR, DASH_CHAR]);
		map.insert(DOCTYPE, vec!['D', 'O', 'C', 'T', 'Y', 'P', 'E']);
		map.insert(
			XMLCDATA,
			vec![
				LEFT_BRACKET_CHAR,
				'C',
				'D',
				'A',
				'T',
				'A',
				LEFT_BRACKET_CHAR,
			],
		);
		map
	};
	static ref VOID_ELEMENTS: Vec<Vec<char>> = vec![
		vec!['i', 'm', 'g'],
		vec!['i', 'n', 'p', 'u', 't'],
		vec!['m', 'e', 't', 'a'],
		vec!['l', 'i', 'n', 'k'],
		vec!['b', 'r'],
		vec!['h', 'r'],
		vec!['c', 'o', 'l'],
		vec!['b', 'a', 's', 'e'],
		vec!['p', 'a', 'r', 'a', 'm'],
		vec!['s', 'o', 'u', 'r', 'c', 'e'],
		vec!['a', 'r', 'e', 'a'],
		vec!['e', 'm', 'b', 'e', 'd'],
		vec!['t', 'r', 'a', 'c', 'k'],
		vec!['w', 'b', 'r'],
	];
	static ref SPECIAL_TAG_MAP: HashMap<Vec<char>, SpecialTag> = {
		use SpecialTag::*;
		let mut map = HashMap::new();
		map.insert(vec!['s', 'v', 'g'], Svg);
		map.insert(vec!['m', 'a', 't', 'h'], MathML);
		map
	};
	static ref MUST_QUOTE_ATTR_CHARS: Vec<char> = vec![
		DOUBLE_QUOTE_CHAR,
		SINGLE_QUOTE_CHAR,
		TAG_BEGIN_CHAR,
		TAG_END_CHAR,
		EQUAL_CHAR,
		'`',
	];
}

// gather previous characters
fn chars_to_string(content: &[char]) -> String {
	content.iter().collect::<String>()
}

// create parse error
fn create_parse_error(kind: ErrorKind, position: CodeAt, context: &str) -> HResult {
	let err = ParseError::new(
		kind,
		CodeRegion::from_context_index(context, position.index),
	);
	Err(err)
}

fn is_void_tag(name: &[char]) -> bool {
	for cur_name in VOID_ELEMENTS.iter() {
		if is_equal_chars(&cur_name, name, &None) {
			return true;
		}
	}
	false
}

fn is_plain_text_tag(name: &[char], case: &Option<NameCase>) -> bool {
	is_equal_chars(name, &TEXTAREA_TAG_NAME, case) || is_equal_chars(name, &TITLE_TAG_NAME, case)
}

fn is_script_or_style(name: &[char], case: &Option<NameCase>) -> bool {
	is_equal_chars(name, &STYLE_TAG_NAME, case) || is_equal_chars(name, &SCRIPT_TAG_NAME, case)
}

pub fn is_content_tag(name: &[char], case: &Option<NameCase>) -> bool {
	is_script_or_style(name, case) || is_plain_text_tag(name, case)
}

pub fn allow_insert(name: &[char], node_type: NodeType) -> bool {
	if is_void_tag(name) {
		return false;
	}
	let case: Option<NameCase> = None;
	use NodeType::*;
	if let Some(special) = SPECIAL_TAG_MAP.get(name) {
		let code_in = match node_type {
			AbstractRoot => CodeTypeIn::AbstractRoot,
			HTMLDOCTYPE => CodeTypeIn::HTMLDOCTYPE,
			Comment => CodeTypeIn::Comment,
			Text | SpacesBetweenTag => CodeTypeIn::TextNode,
			TagEnd => CodeTypeIn::TagEnd,
			XMLCDATA => CodeTypeIn::XMLCDATA,
			Tag => {
				if is_equal_chars(name, &SCRIPT_TAG_NAME, &case) {
					CodeTypeIn::HTMLScript
				} else if is_equal_chars(name, &STYLE_TAG_NAME, &case) {
					CodeTypeIn::HTMLStyle
				} else {
					CodeTypeIn::Tag
				}
			}
		};
		return special
			.is_ok(&code_in, name, &EMPTY_CHAR, &CodeAt::default(), "")
			.is_ok();
	}
	if is_plain_text_tag(name, &None) {
		return node_type == Text || node_type == SpacesBetweenTag;
	}
	true
}

#[derive(PartialEq, Eq, Hash)]
pub enum DetectChar {
	Comment,
	DOCTYPE,
	XMLCDATA,
}

#[derive(PartialEq, Debug, Clone, Copy)]
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

impl Default for NodeType {
	fn default() -> Self {
		NodeType::AbstractRoot
	}
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

fn get_content_encode(content: &[char]) -> Vec<char> {
	encode_chars(content, EntitySet::Html, EncodeType::Named)
}

// trim chars end whitespaces
fn chars_trim_end(target: &[char]) -> &[char] {
	let mut end_index: usize = target.len();
	for ch in target.iter().rev() {
		if !ch.is_ascii_whitespace() {
			break;
		}
		end_index -= 1;
	}
	&target[..end_index]
}

// check if equal
fn is_equal_chars(target: &[char], cmp: &[char], case: &Option<NameCase>) -> bool {
	if target.len() != cmp.len() {
		return false;
	}
	// check characters
	let is_equal = if let Some(case) = case {
		match case {
			NameCase::Lower => |a: &char, b: &char| -> bool { a == b || &a.to_ascii_lowercase() == b },
			NameCase::Upper => |a: &char, b: &char| -> bool { a == b || &a.to_ascii_uppercase() == b },
		}
	} else {
		|a: &char, b: &char| -> bool { a == b }
	};
	for (index, ch) in target.iter().enumerate() {
		if !is_equal(ch, &cmp[index]) {
			return false;
		}
	}
	true
}

fn is_equal_chars_ignore_case(target: &[char], cmp: &[char]) -> (bool, bool) {
	if target.len() != cmp.len() {
		return (false, false);
	}
	let mut is_total_same = true;
	for (index, ch) in target.iter().enumerate() {
		let cmp_ch = &cmp[index];
		if cmp_ch == ch {
			continue;
		}
		// test if totally same
		is_total_same = false;
		match cmp_ch {
			'a'..='z' => {
				if &cmp_ch.to_ascii_uppercase() != ch {
					return (false, false);
				}
			}
			'A'..='Z' => {
				if &cmp_ch.to_ascii_lowercase() != ch {
					return (false, false);
				}
			}
			_ => {
				// not equal
				return (false, false);
			}
		}
	}
	(true, is_total_same)
}
/**
 * Attr
 * attribute data
 * if value is None, it's a boolean attribute
 * if key is None,it's a value with quote
 */

#[derive(Debug, Default)]
pub struct Attr {
	pub key: Option<AttrData>,
	pub value: Option<AttrData>,
	pub quote: Option<char>,
	pub need_quote: bool,
}

#[derive(Debug, Default)]
pub struct AttrData {
	pub content: Vec<char>,
}

impl Attr {
	// build attribute code
	pub fn build(&self, remove_quote: bool) -> Vec<char> {
		let mut ret = Vec::with_capacity(ALLOC_CHAR_CAPACITY);
		let mut has_key = false;
		if let Some(AttrData { content, .. }) = &self.key {
			ret.extend_from_slice(&content);
			has_key = true;
		}
		if let Some(AttrData { content, .. }) = &self.value {
			if has_key {
				ret.push(EQUAL_CHAR);
			}
			if let Some(quote) = self.quote {
				if self.need_quote || !remove_quote {
					ret.push(quote);
					ret.extend_from_slice(&content);
					ret.push(quote);
					return ret;
				}
			}
			ret.extend_from_slice(&content);
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
			if is_equal_chars(&key.content, &['i', 'd'], &Some(NameCase::Lower)) {
				if let Some(value) = &self.value {
					return Some(chars_to_string(&value.content));
				}
			}
		}
		None
	}
}

pub enum NameCase {
	Upper,
	Lower,
}
/**
 * Tag
 * is_end: if the tag end with '>'
 * self_closed: if the tag is self-closing '/>'
 * auto_fix: if the tag either self-closing nor closed with a end tag, may auto fix by the parser
 * name: the tag name
 * attrs: the attribute list
*/
#[derive(Debug, PartialEq)]
pub enum TagCodeIn {
	Wait,
	Key,
	KeyEnd,
	WaitValue,
	Value,
	ValueEnd,
}

impl Default for TagCodeIn {
	fn default() -> Self {
		TagCodeIn::Wait
	}
}
#[derive(Debug, Default)]
pub struct TagMeta {
	code_in: TagCodeIn,
	pub is_void: bool,
	pub self_closed: bool,
	pub auto_fix: bool,
	pub name: Vec<char>,
	pub attrs: Vec<Attr>,
	pub lc_name_map: HashMap<String, usize>,
}

impl TagMeta {
	pub fn attrs_to_string(&self, remove_quote: bool) -> Vec<char> {
		self
			.attrs
			.iter()
			.map(|attr| {
				let mut attr_content = attr.build(remove_quote);
				attr_content.splice(0..0, vec![EMPTY_CHAR]);
				attr_content
			})
			.flatten()
			.collect()
	}
	// add a key attr
	pub fn add_attr_key(&mut self) {
		self.attrs.push(Attr {
			key: Some(AttrData::default()),
			..Default::default()
		});
	}
	// add a only value attr
	pub fn add_attr_value(&mut self, quote: Option<char>) {
		self.attrs.push(Attr {
			value: Some(AttrData::default()),
			quote,
			..Default::default()
		});
	}
	// set attr key
	pub fn set_attr_key(&mut self, key: Vec<char>) {
		let attr = self
			.attrs
			.last_mut()
			.expect("Attrs must not be empty when call set_attr_key");
		attr.key = Some(AttrData { content: key });
	}
	// set attr value
	pub fn set_attr_value(&mut self, value: Vec<char>, quote: Option<char>) -> &Attr {
		let attr = self
			.attrs
			.last_mut()
			.expect("Attrs must not be empty when call set_attr_key");
		attr.value = Some(AttrData { content: value });
		attr.quote = quote;
		attr
	}
}

pub type RefNode = Rc<RefCell<Node>>;

type RefDoc = Rc<RefCell<Doc>>;

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
#[derive(Default)]
pub struct Node {
	// the node's index of the parent's all childs
	pub index: usize,

	// the node's type
	pub node_type: NodeType,

	// the node's start position '<'
	pub begin_at: CodeAt,

	// the node's end position '>'
	pub end_at: CodeAt,

	// the end tag </xx> of the tag node
	pub end_tag: Option<RefNode>,

	// prev node
	pub prev: Option<Weak<RefCell<Node>>>,

	// parent node, use weak reference,prevent reference loop
	pub parent: Option<Weak<RefCell<Node>>>,

	// root node
	pub root: Option<Weak<RefCell<Node>>>,

	// document
	pub document: Option<RefDoc>,

	// the content,for text/comment/style/script nodes
	pub content: Option<Vec<char>>,

	// the child nodes
	pub childs: Option<Vec<RefNode>>,

	// the tag node meta information
	pub meta: Option<RefCell<TagMeta>>,

	// special information
	pub special: Option<SpecialTag>,
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Node")
			.field("index", &self.index)
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
			.field("document", &self.document.is_some())
			.finish()
	}
}

impl Node {
	// create a new node
	pub fn new(node_type: NodeType, code_at: CodeAt) -> Self {
		Node {
			node_type,
			begin_at: code_at,
			end_at: code_at,
			..Default::default()
		}
	}
	pub fn create_text_node(content: Vec<char>, code_at: Option<CodeAt>) -> Self {
		let mut is_all_spaces = true;
		for ch in &content {
			if !ch.is_ascii_whitespace() {
				is_all_spaces = false;
				break;
			}
		}
		let node_type = if is_all_spaces {
			NodeType::SpacesBetweenTag
		} else {
			NodeType::Text
		};
		let code_at = code_at.unwrap_or_default();
		Node {
			node_type,
			begin_at: code_at,
			end_at: code_at,
			content: Some(content),
			..Default::default()
		}
	}
	// build node
	fn build_node(&self, options: &RenderOptions, status: &mut RenderStatus, result: &mut Vec<char>) {
		let is_in_pre = status.is_in_pre;
		let is_inner = status.inner_type.is_some();
		use NodeType::*;
		match self.node_type {
			Text => {
				// original content
				let content = self
					.content
					.as_ref()
					.expect("Text node's conetnt must not empty");
				// check if need decode
				if !is_in_pre && options.minify_spaces {
					// just keep one space
					let mut prev_is_space = false;
					if options.decode_entity {
						let mut start_index: usize = 0;
						let mut is_in_entity = false;
						for (index, ch) in content.iter().enumerate() {
							if !is_in_entity {
								if ch == &'&' {
									is_in_entity = true;
									start_index = index;
								} else {
									// judge if whitespace
									if ch.is_ascii_whitespace() {
										if prev_is_space {
											continue;
										}
										prev_is_space = true;
										result.push(*ch);
									} else {
										prev_is_space = false;
										result.push(*ch);
									}
								}
							} else if ch == &';' {
								// entity end
								let entity = &content[start_index + 1..index];
								if let Some(decoded) = Entity::decode(entity) {
									result.push(decoded);
								} else {
									result.push('&');
									result.extend_from_slice(entity);
									result.push(';');
								}
								is_in_entity = false;
							}
						}
						// has suffix wrong entity
						if is_in_entity {
							result.extend(&content[start_index..]);
						}
					} else {
						// just add one space when meet repeated whitespaces
						for &c in content {
							if c.is_ascii_whitespace() {
								if prev_is_space {
									continue;
								}
								prev_is_space = true;
								result.push(c);
							} else {
								prev_is_space = false;
								result.push(c);
							}
						}
					}
				} else {
					// decode entity
					if options.decode_entity {
						// when need decode, the content should append to result
						decode_chars_to(&content, result);
					} else {
						result.extend_from_slice(&content);
					}
				}
			}
			Tag => {
				let meta = self
					.meta
					.as_ref()
					.expect("tag's meta data must have.")
					.borrow();
				let tag_name = &meta.name;
				// check if is in pre, only check if not in pre
				status.is_in_pre =
					is_in_pre || is_equal_chars(&tag_name, &PRE_TAG_NAME, &Some(NameCase::Lower));
				if !is_inner {
					// add tag name
					result.push('<');
					if !options.lowercase_tagname {
						result.extend_from_slice(&tag_name);
					} else {
						for ch in tag_name {
							result.push(ch.to_ascii_lowercase());
						}
					}
					// add attrs
					if !meta.attrs.is_empty() {
						let attrs = meta.attrs_to_string(options.remove_attr_quote);
						result.extend_from_slice(&attrs);
					}
					// add self closing
					if meta.self_closed || (meta.auto_fix && options.always_close_void) {
						result.push(EMPTY_CHAR);
						result.push('/');
					}
					// add end char
					result.push(TAG_END_CHAR);
				}
				// content for some special tags, such as style/script
				if let Some(content) = &self.content {
					let need_encode =
						options.encode_content && is_plain_text_tag(&tag_name, &Some(NameCase::Lower));
					if !need_encode {
						result.extend_from_slice(&content);
					} else {
						// content tag's html need encode
						result.extend(get_content_encode(&content));
					}
				}
			}
			TagEnd => {
				let content = self
					.content
					.as_ref()
					.expect("End tag's tag name must not empty");
				let mut content = &content[..];
				if is_in_pre
					&& is_equal_chars(
						chars_trim_end(&content),
						&PRE_TAG_NAME,
						&Some(NameCase::Lower),
					) {
					status.is_in_pre = false;
				}
				if !is_inner {
					result.extend_from_slice(&['<', '/']);
					if options.remove_endtag_space {
						content = chars_trim_end(&content);
					}
					if options.lowercase_tagname {
						let content = content
							.iter()
							.map(|e| e.to_ascii_lowercase())
							.collect::<Vec<char>>();
						result.extend(content);
					} else {
						result.extend_from_slice(&content);
					}
					result.push('>');
				}
			}
			SpacesBetweenTag => {
				if !is_in_pre && options.minify_spaces {
					// do nothing
				} else {
					let content = self
						.content
						.as_ref()
						.expect("Spaces between node must have whitespcaes");
					result.extend_from_slice(&content);
				}
			}
			HTMLDOCTYPE => {
				let meta = self
					.meta
					.as_ref()
					.expect("tag's meta data must have.")
					.borrow();
				result.extend_from_slice(&['<', '!']);
				result.extend_from_slice(&meta.name);
				if !meta.attrs.is_empty() {
					result.extend_from_slice(&meta.attrs_to_string(options.remove_attr_quote));
				}
				result.push('>');
			}
			Comment => {
				if !options.remove_comment {
					// comment
					result.extend_from_slice(&['<', '!', '-', '-']);
					if let Some(content) = &self.content {
						result.extend_from_slice(&content);
					}
					result.extend_from_slice(&['-', '-', '>']);
				}
			}
			XMLCDATA => {
				// cdata
				result.extend_from_slice(&['<', '!', '[', 'C', 'D', 'A', 'T', 'A', '[']);
				if let Some(content) = &self.content {
					result.extend_from_slice(&content);
				}
				result.extend_from_slice(&[']', ']', '>']);
			}
			_ => {}
		}
	}
	// build node tree
	fn build_tree(&self, options: &RenderOptions, status: &mut RenderStatus, result: &mut Vec<char>) {
		self.build_node(options, status, result);
		if let Some(childs) = &self.childs {
			if let Some(RenderStatuInnerType::Html) = status.inner_type {
				// get inner html
				let mut sub_status = status.clone();
				sub_status.inner_type = None;
				for child in childs {
					child.borrow().build_tree(options, &mut sub_status, result);
				}
			} else {
				for child in childs {
					// keep the original inner type
					child.borrow().build_tree(options, status, result);
				}
			}
		}
		if let Some(end_tag) = &self.end_tag {
			end_tag.borrow().build_node(options, status, result);
		}
	}
	// build
	pub fn build(&self, options: &RenderOptions, inner_text: bool) -> Vec<char> {
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
		let mut result: Vec<char> = Vec::with_capacity(50);
		// inner_html or inner_text
		if status.inner_type.is_some() {
			if matches!(self.node_type, NodeType::AbstractRoot) {
				// inner html for abstract root
				if let Some(childs) = &self.childs {
					let mut finded = false;
					let mut child_node: Option<Rc<RefCell<Node>>> = None;
					for child in childs {
						if child.borrow().node_type == NodeType::Tag {
							if finded {
								panic!("`inner_html` can't used in abstract root node which has multiple tag node childs.");
							}
							child_node = Some(Rc::clone(&child));
							finded = true;
						}
					}
					if let Some(child_node) = child_node {
						child_node.borrow().build_tree(options, status, &mut result);
						return result;
					}
					// no tag child node finded
					throw_wrong_node(&childs[childs.len() - 1].borrow().node_type);
				}
				// abstract without any child
				return vec![];
			}
			if self.node_type != NodeType::Tag {
				throw_wrong_node(&self.node_type);
			}
		}
		self.build_tree(options, status, &mut result);
		result
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
						Comment | SpacesBetweenTag | HTMLDOCTYPE => {
							// the above types are allowed in document
						}
						Tag => {
							if find_html {
								syntax_ok = false;
								break;
							} else {
								// check if is html tag
								if let Some(meta) = &child_node.meta {
									if is_equal_chars(
										meta.borrow().name.as_slice(),
										&HTML_TAG_NAME,
										&Some(NameCase::Lower),
									) {
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
			if !is_document {
				syntax_ok = true;
			}
		}
		(is_document, syntax_ok)
	}
	// check if two RefNode is same
	pub fn is_same(cur: &RefNode, other: &RefNode) -> bool {
		std::ptr::eq(cur.as_ptr() as *const _, other.as_ptr() as *const _)
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum SpecialTag {
	MathML,
	Svg,
	Template,
}

impl SpecialTag {
	pub fn is_ok(
		&self,
		code_in: &CodeTypeIn,
		tag_name: &[char],
		c: &char,
		position: &CodeAt,
		context: &str,
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
			Svg | MathML => {
				match code_in {
					Tag | XMLCDATA => {}
					HTMLScript | HTMLStyle if self == &Svg => {}
					TextNode if c.is_ascii_whitespace() => {}
					_ => {
						let message = format!(
							"the tag '{}' can only contains sub tags, find node '{:?}' at {:?}",
							chars_to_string(&tag_name),
							code_in,
							position
						);
						return create_parse_error(ErrorKind::CommonError(message), *position, context);
					}
				};
			}
			Template => {}
		};
		Ok(())
	}
}

type NextHandle = fn(&mut Doc, char, &str) -> HResult;

/*
* No operation, just for initialization
*/
fn noop(_d: &mut Doc, _c: char, _content: &str) -> HResult {
	Ok(())
}

/*
 * code_in:  Unkown | AbstractRoot
*/
fn parse_wait(doc: &mut Doc, c: char, _: &str) -> HResult {
	match c {
		// match the tag start '<'
		TAG_BEGIN_CHAR => {
			doc.mem_position = doc.position;
			doc.set_code_in(CodeTypeIn::UnkownTag);
		}
		_ => {
			if doc.parse_options.auto_fix_unexpected_endtag
				&& matches!(
					doc.current_node.borrow().node_type,
					NodeType::Text | NodeType::SpacesBetweenTag
				) {
				// fix the unexpected endtag
				let mut current_node = doc.current_node.borrow_mut();
				// remove content to prev chars
				if let Some(content) = &mut current_node.content {
					doc.prev_chars.append(content);
				}
				if matches!(current_node.node_type, NodeType::Text) {
					doc.repeat_whitespace = false;
				} else {
					doc.repeat_whitespace = c.is_ascii_whitespace();
				}
			} else {
				// new text node
				doc.add_new_node(Rc::new(RefCell::new(Node::new(
					NodeType::Text,
					doc.position,
				))));
				doc.repeat_whitespace = c.is_ascii_whitespace();
			}
			doc.prev_chars.push(c);
			doc.set_code_in(CodeTypeIn::TextNode);
		}
	}
	Ok(())
}

/**
 * code_in: Text
*/
fn parse_text(doc: &mut Doc, c: char, _: &str) -> HResult {
	use CodeTypeIn::*;
	match c {
		// match the tag start '<'
		TAG_BEGIN_CHAR => {
			let content = doc.clean_chars_to_vec();
			doc.current_node.borrow_mut().content = Some(content);
			doc.check_textnode = if doc.repeat_whitespace {
				Some(Rc::clone(&doc.current_node))
			} else {
				None
			};
			doc.mem_position = doc.position;
			doc.set_tag_end_info();
			doc.set_code_in(UnkownTag);
		}
		_ => {
			// check if repeat whitespace
			if doc.repeat_whitespace {
				doc.repeat_whitespace = c.is_ascii_whitespace();
			}
			doc.prev_chars.push(c);
		}
	}
	Ok(())
}

/**
* code_in: Doctype name
*/
fn parse_doctype_name(doc: &mut Doc, c: char, context: &str) -> HResult {
	if c.is_ascii_whitespace() {
		doc.set_tag_meta();
		doc.set_tag_code_in(TagCodeIn::Wait);
		return Ok(());
	}
	// wrong doctype
	doc.error(ErrorKind::WrongHtmlDoctype(c), context)
}

/**
 * code_in: Tag name
 */
// parse tag name
fn parse_tag_name(doc: &mut Doc, c: char, _: &str) -> HResult {
	// tag name will setted in this process
	// 1. <a>
	// 2. <a/>
	// 3. <a /> <a href="">
	use CodeTypeIn::*;
	match c {
		TAG_END_CHAR => {
			// detect if void tag
			let is_void = doc.set_tag_meta();
			if is_void {
				// tag is end
				doc.chain_nodes.pop();
				doc.set_code_in(Unkown);
			} else {
				// check the tag type
				doc.check_tag_type_do();
			}
		}
		END_SLASH_CHAR => {
			// maybe self closing tag
			// if is doctype, not a good doctype
			doc.set_tag_meta();
			doc.handle = parse_tag_self_closing;
		}
		_ => {
			if c.is_ascii_whitespace() {
				// wait for attribute or end
				doc.set_tag_meta();
				doc.set_tag_code_in(TagCodeIn::Wait);
			} else {
				doc.prev_chars.push(c);
			}
		}
	};
	Ok(())
}

/**
 * Tag self closing
 */
// parse tag self closing
fn parse_tag_self_closing(doc: &mut Doc, c: char, context: &str) -> HResult {
	// The self closing tag will parsed in this proces
	// 1. /> ///>
	// 2. /a="33"  //a="33"
	match c {
		TAG_END_CHAR => {
			if let Some(meta) = &doc.current_node.borrow_mut().meta {
				if !doc.parse_options.allow_self_closing {
					// void elements or self closing element
					// sub element in Svg or MathML allow self-closing
					match doc.check_special() {
						Some(SpecialTag::Svg) | Some(SpecialTag::MathML) => {
							// is in svg or mathml
						}
						_ => {
							// wrong self closing tag
							if !meta.borrow().is_void {
								return doc.error(
									ErrorKind::WrongSelfClosing(chars_to_string(&meta.borrow().name)),
									context,
								);
							}
						}
					}
				}
				meta.borrow_mut().self_closed = true;
			} else {
				panic!("self-closing tag's meta is emtpy");
			}
			doc.chain_nodes.pop();
			doc.set_code_in(CodeTypeIn::Unkown);
		}
		END_SLASH_CHAR => {
			// ignore the end slash tag name character
			// take the end slash equal to empty character
			// e.g. <a//> tag name is 'a'
		}
		_ => {
			// just delegate to wait process
			parse_tag_wait(doc, c, context)?;
		}
	}
	Ok(())
}

/// in tag, wait attribute or end
fn parse_tag_wait(doc: &mut Doc, c: char, _: &str) -> HResult {
	// All tag wait status will parsed in this process
	// 1. spaces between attributes TagCodeIn::Wait
	// 2. (spaces)?= TagCodeIn::KeyEnd
	// 3. new attribute key
	// 4. attribute value
	// 5. double quote single quote value attribute
	// 6. tag end
	// 7. self closing
	match c {
		DOUBLE_QUOTE_CHAR | SINGLE_QUOTE_CHAR => {
			// only attribute value without attribute key
			// add a attribute value
			let is_in_wait_value = doc.is_tag_code_in(&TagCodeIn::WaitValue);
			if !is_in_wait_value {
				// attribute key = "value"
				// no need add attribute to queue, just change the previous attribute's value
				if matches!(doc.code_in, CodeTypeIn::HTMLDOCTYPE) {
					doc.add_tag_attr_value(Some(c));
				} else {
					// take quote value as attribute key too
					doc.add_tag_attr_key();
					doc.prev_chars.push(c);
					doc.set_tag_code_in(TagCodeIn::Key);
					return Ok(());
				}
			}
			doc.mark_char = c;
			doc.set_tag_code_in(TagCodeIn::Value);
		}
		TAG_END_CHAR => {
			if doc
				.current_node
				.borrow()
				.meta
				.as_ref()
				.expect("When parse tag in wait, the tag meta must not empty.")
				.borrow()
				.is_void
			{
				// if void tag, pop from chains, set in unkown
				doc.chain_nodes.pop();
				doc.set_code_in(CodeTypeIn::Unkown);
			} else {
				// need check the tag if special tags
				doc.check_tag_type_do();
			}
		}
		END_SLASH_CHAR => {
			// parse self closing
			doc.handle = parse_tag_self_closing;
		}
		EQUAL_CHAR => {
			// value
			if doc.is_tag_code_in(&TagCodeIn::KeyEnd) {
				// the prev process is parse attr key
				doc.set_tag_code_in(TagCodeIn::WaitValue);
			} else {
				// take as non quoted attribute key value
				doc.prev_chars.push(c);
				// jump to parse value
				doc.mark_char = EMPTY_CHAR;
				doc.set_tag_code_in(TagCodeIn::Value);
			}
		}
		_ => {
			// ignore whitespaces
			if !c.is_ascii_whitespace() {
				// parse tag attribute
				doc.prev_chars.push(c);
				if doc.is_tag_code_in(&TagCodeIn::WaitValue) {
					// set the value end character as empty character
					doc.mark_char = EMPTY_CHAR;
					// parse tag value
					doc.set_tag_code_in(TagCodeIn::Value);
				} else {
					// add attr key
					doc.add_tag_attr_key();
					// parse attr key
					doc.set_tag_code_in(TagCodeIn::Key);
				}
			}
			// ignore whitespaces, just change the detect prev char
		}
	}
	Ok(())
}

// parse tag attribute key
fn parse_tag_attr_key(doc: &mut Doc, c: char, context: &str) -> HResult {
	// attr key will parsed in this process
	match c {
		EQUAL_CHAR => {
			// attribute end
			doc.set_tag_attr_key();
			// set tag wait
			doc.set_tag_code_in(TagCodeIn::WaitValue);
		}
		TAG_END_CHAR => {
			// attribute key end
			doc.set_tag_attr_key();
			doc.set_tag_code_in(TagCodeIn::KeyEnd);
			// just delegate to parse wait
			parse_tag_wait(doc, c, context)?;
		}
		END_SLASH_CHAR => {
			// self closing
			doc.set_tag_attr_key();
			// set tag code in
			doc.set_tag_code_in(TagCodeIn::Wait);
		}
		_ => {
			if c.is_ascii_whitespace() {
				// end of the attribute key
				// attribute end
				doc.set_tag_attr_key();
				// set in wait
				doc.set_tag_code_in(TagCodeIn::KeyEnd);
				// set handle
				doc.handle = parse_tag_wait;
			} else {
				// all charactes except the characters above was allowed
				doc.prev_chars.push(c);
			}
		}
	};
	Ok(())
}

// parse tag attribute double quote value
fn parse_tag_attr_value(doc: &mut Doc, c: char, context: &str) -> HResult {
	#[inline]
	fn make_quote(c: char) -> Option<char> {
		if c == EMPTY_CHAR {
			None
		} else {
			Some(c)
		}
	}
	// logic
	let quote = doc.mark_char;
	if c == quote || (quote == EMPTY_CHAR && c.is_ascii_whitespace()) {
		// reset the detect char
		doc.mark_char = EOF_CHAR;
		doc.set_tag_attr_value(make_quote(quote));
		doc.set_tag_code_in(TagCodeIn::Wait);
	} else {
		// unquoted value
		// check if avaiable character in value
		if c == TAG_END_CHAR {
			// set tag value parse end
			doc.set_tag_attr_value(make_quote(quote));
			doc.set_tag_code_in(TagCodeIn::ValueEnd);
			// delegate to parse wait
			parse_tag_wait(doc, c, context)?;
		} else {
			doc.prev_chars.push(c);
		}
	}
	Ok(())
}

/**
 * code_in: TagEnd
 */
fn parse_tagend(doc: &mut Doc, c: char, context: &str) -> HResult {
	use CodeTypeIn::*;
	// the end tag
	match c {
		TAG_END_CHAR => {
			let end_tag_name = doc.prev_chars.clone();
			let fix_end_tag_name = chars_trim_end(&end_tag_name);
			let mut is_endtag_ok = false;
			// check if the tag matched
			if doc.chain_nodes.len() > 1 {
				if let Some(tag) = doc.chain_nodes.last() {
					let tag = tag.borrow();
					let meta = tag
						.meta
						.as_ref()
						.expect("Tag node must have a meta of tag name")
						.borrow();
					let start_tag_name = &meta.name;
					let (is_equal, is_total_same) =
						is_equal_chars_ignore_case(&start_tag_name, &fix_end_tag_name);

					if is_equal {
						if doc.parse_options.case_sensitive_tagname && !is_total_same {
							return doc.error(
								ErrorKind::WrongCaseSensitive(doc.chars_to_string()),
								context,
							);
						}
						is_endtag_ok = true;
					}
				}
			}
			// meet the right end tag
			if is_endtag_ok {
				// pop from chain nodes
				let last_tag = doc
					.chain_nodes
					.pop()
					.expect("End tag must have matched tag in chain nodes");
				// set end tag
				last_tag.borrow_mut().end_tag = Some(Rc::clone(&doc.current_node));
				// set space between tag
				let is_only_text_child = match &last_tag.borrow().childs {
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
				// end of special tag
				if doc.in_special.is_some()
					&& is_equal_chars_ignore_case(&doc.in_special.as_ref().unwrap().1, fix_end_tag_name).0
				{
					doc.in_special = None;
				}
				// set end tag more info
				let content = doc.clean_chars_to_vec();
				let mut current_node = doc.current_node.borrow_mut();
				current_node.parent = Some(Rc::downgrade(&last_tag));
				current_node.content = Some(content);
			} else {
				// no matched start tag
				if !doc.parse_options.auto_fix_unexpected_endtag {
					// wrong end tag without start tag
					return doc.error(ErrorKind::WrongEndTag(doc.chars_to_string()), context);
				} else {
					// set orig current node
					let mut orig_current_node: Option<RefNode> = None;
					if let Some(prev_node) = &doc.current_node.borrow().prev {
						orig_current_node = Some(prev_node.upgrade().expect(""));
					}
					if let Some(orig_current_node) = orig_current_node {
						doc.current_node = orig_current_node;
					}
					// just ignore the end tag
					doc.prev_chars.clear();
					doc.set_code_in(Unkown);
					return Ok(());
				}
			}
		}
		_ => {
			// just add char to prev chars
			doc.prev_chars.push(c);
		}
	}
	// return
	Ok(())
}

/**
 * code_in: HTMLScript | HTMLStyle | EscapeableRawText
 */
fn parse_special_tag(doc: &mut Doc, c: char, _: &str) -> HResult {
	use CodeTypeIn::*;
	// parse html script tag and style tag
	if c == TAG_END_CHAR {
		let end_tag = doc
			.detect
			.as_ref()
			.expect("detect chars must set before set_code_in.");
		let mut detect_len = end_tag.len();
		let prev_chars = &doc.prev_chars;
		let mut prev_len = doc.prev_chars.len();
		if prev_len >= detect_len {
			// remove suffix whitespaces
			while prev_len > 0 {
				let cur_index = prev_len - 1;
				let prev_char = prev_chars[cur_index];
				if !prev_char.is_ascii_whitespace() {
					break;
				} else {
					prev_len = cur_index;
				}
			}
			// check if need detect again
			if prev_len >= detect_len {
				let case_sensitive = doc.parse_options.case_sensitive_tagname;
				while detect_len > 0 {
					let detect_index = detect_len - 1;
					let detect_char = end_tag[detect_index];
					let prev_index = prev_len - 1;
					let prev_char = prev_chars[prev_index];
					let is_matched = if detect_char == prev_char {
						true
					} else {
						// check if case sensitive
						if case_sensitive {
							false
						} else {
							match detect_char {
								'A'..='Z' => detect_char.to_ascii_lowercase() == prev_char,
								'a'..='z' => detect_char.to_ascii_uppercase() == prev_char,
								_ => false,
							}
						}
					};
					if is_matched {
						// move forward detect
						detect_len = detect_index;
						// move forward prev chars
						prev_len = prev_index;
					} else {
						break;
					}
				}
			}
			// when detect_len equal 0, means all detect characteres are matched
			if detect_len == 0 {
				// set code in unkown
				doc.set_code_in(Unkown);
				// find the matched
				let end_tag_name = doc.prev_chars.split_off(prev_len).split_off(2);
				// add an end tag
				let mut end = Node::new(NodeType::TagEnd, doc.position.next_col());
				end.content = Some(end_tag_name);
				end.parent = Some(Rc::downgrade(&doc.current_node));
				// set tag node's content, end_tag
				let node = Rc::new(RefCell::new(end));
				// here split off is quickly than clean_chars_to_vec
				let content = doc.prev_chars.split_off(0);
				let mut current_node = doc.current_node.borrow_mut();
				current_node.end_tag = Some(Rc::clone(&node));
				current_node.content = Some(content);
				// remove current tag
				doc.chain_nodes.pop();
				doc.detect = None;
				return Ok(());
			}
		}
	}
	doc.prev_chars.push(c);
	Ok(())
}

/**
 * code_in: Comment|CDATA
 */
fn parse_comment_or_cdata(doc: &mut Doc, c: char, _: &str) -> HResult {
	use CodeTypeIn::*;
	// comment node
	if c == TAG_END_CHAR {
		let chars = &doc.prev_chars;
		let total = chars.len();
		if total > 2 {
			let end_symbol = doc.mark_char;
			let prev_char = chars[total - 1];
			if prev_char == end_symbol && chars[total - 2] == end_symbol {
				let mut content = doc.clean_chars_to_vec();
				content.truncate(total - 2);
				doc.current_node.borrow_mut().content = Some(content);
				// reset mark char
				doc.mark_char = EOF_CHAR;
				doc.set_tag_end_info();
				doc.set_code_in(Unkown);
				return Ok(());
			}
		}
	}
	doc.prev_chars.push(c);
	Ok(())
}

/**
 * code_in: UnkownTag
 */
fn parse_unkown_tag(doc: &mut Doc, c: char, context: &str) -> HResult {
	use CodeTypeIn::*;
	// check the tag type
	match c {
		'a'..='z' | 'A'..='Z' => {
			// new tag node
			let inner_node = Node::new(NodeType::Tag, doc.mem_position);
			let node = Rc::new(RefCell::new(inner_node));
			doc.add_new_node(node);
			doc.set_code_in(Tag);
			doc.set_text_spaces_between();
			doc.prev_chars.push(c);
		}
		END_SLASH_CHAR => {
			// if allow auto fix unclosed tag
			if !doc.parse_options.auto_fix_unclosed_tag {
				doc.add_new_node(Rc::new(RefCell::new(Node::new(
					NodeType::TagEnd,
					doc.mem_position,
				))));
			} else {
				let prev_node = Rc::downgrade(&doc.current_node);
				// tag end
				doc.add_new_node(Rc::new(RefCell::new(Node::new(
					NodeType::TagEnd,
					doc.mem_position,
				))));
				doc.current_node.borrow_mut().prev = Some(prev_node);
			}
			// set code in first
			doc.set_code_in(TagEnd);
		}
		'!' => {
			// Comment|DOCTYPE
			doc.set_code_in(ExclamationBegin);
		}
		_ => {
			if !doc.parse_options.auto_fix_unescaped_lt {
				return create_parse_error(
					ErrorKind::WrongTag(c.to_string()),
					doc.mem_position,
					context,
				);
			}
			// fix unescaped left angle bracket
			doc.fix_unescaped_lt(c, context)?;
		}
	};
	Ok(())
}

/**
 * code_in: ExclamationBegin
 */
fn parse_exclamation_begin(doc: &mut Doc, c: char, context: &str) -> HResult {
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
						DASH_CHAR | LEFT_BRACKET_CHAR => {
							let code_in: CodeTypeIn;
							let node_type: NodeType;
							if c == DASH_CHAR {
								code_in = Comment;
								doc.mark_char = DASH_CHAR;
								// new comment node
								node_type = NodeType::Comment;
							} else {
								code_in = XMLCDATA;
								doc.mark_char = RIGHT_BRACKET_CHAR;
								// new cdata
								node_type = NodeType::XMLCDATA;
							}
							doc.set_code_in(code_in);
							// new comment node
							doc.add_new_node(Rc::new(RefCell::new(Node::new(node_type, begin_at))));
							doc.prev_chars.clear();
							doc.set_text_spaces_between();
							// ignore the character
							ignore_char = true;
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
					ErrorKind::UnrecognizedTag(doc.chars_to_string(), chars_to_string(&next_chars)),
					doc.mem_position,
					context,
				);
			}
		}
	} else {
		let detect_type: DetectChar;
		match c {
			DASH_CHAR => {
				detect_type = DetectChar::Comment;
			}
			'D' | 'd' => {
				detect_type = DetectChar::DOCTYPE;
			}
			LEFT_BRACKET_CHAR => {
				// CDATA
				let special_tag_name = doc.in_special.as_ref().map(|(_, name)| name);
				if let Some(tag_name) = special_tag_name {
					if is_equal_chars(
						&tag_name,
						&doc
							.chain_nodes
							.last()
							.unwrap()
							.borrow()
							.meta
							.as_ref()
							.expect("Chain nodes must all be tag nodes")
							.borrow()
							.name,
						&None,
					) {
						return create_parse_error(
							ErrorKind::CommonError("<![CDATA tag can in sub node".into()),
							doc.position,
							context,
						);
					} else {
						detect_type = DetectChar::XMLCDATA;
					}
				} else {
					return create_parse_error(
						ErrorKind::CommonError("wrong <![CDATA tag can only used in Svg or MathML".into()),
						doc.position,
						context,
					);
				}
			}
			_ => {
				return create_parse_error(
					ErrorKind::WrongTag(doc.chars_to_string()),
					doc.mem_position,
					context,
				);
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
	position: CodeAt,
	mem_position: CodeAt,
	detect: Option<Vec<char>>,
	prev_chars: Vec<char>,
	mark_char: char,
	chain_nodes: Vec<RefNode>,
	current_node: RefNode,
	in_special: Option<(SpecialTag, Vec<char>)>,
	repeat_whitespace: bool,
	check_textnode: Option<RefNode>,
	handle: NextHandle,
	pub parse_options: ParseOptions,
	pub root: RefNode,
	pub id_tags: Rc<RefCell<StringNodeMap>>,
	pub onerror: Rc<RefCell<Option<Rc<ErrorHandle>>>>,
}

pub type StringNodeMap = HashMap<String, RefNode>;
pub type ErrorHandle = Box<dyn Fn(Box<dyn Error>)>;

impl Doc {
	// create new parser
	fn new() -> Self {
		let node = Rc::new(RefCell::new(Node::new(
			NodeType::AbstractRoot,
			CodeAt::begin(),
		)));
		let ref_node = Rc::clone(&node);
		let current_node = Rc::clone(&node);
		let root = Rc::clone(&node);
		let mut nodes = Vec::with_capacity(ALLOC_NODES_CAPACITY);
		let mut chain_nodes = Vec::with_capacity(ALLOC_NODES_CAPACITY);
		// set root for root node
		node.borrow_mut().root = Some(Rc::downgrade(&root));
		nodes.push(node);
		chain_nodes.push(ref_node);
		let mut doc = Doc {
			code_in: CodeTypeIn::AbstractRoot,
			position: CodeAt::begin(),
			mem_position: CodeAt::begin(),
			mark_char: EOF_CHAR,
			prev_chars: Vec::with_capacity(ALLOC_CHAR_CAPACITY),
			chain_nodes,
			current_node,
			detect: None,
			in_special: None,
			parse_options: Default::default(),
			repeat_whitespace: false,
			check_textnode: None,
			handle: noop,
			root,
			id_tags: Rc::new(RefCell::new(HashMap::new())),
			onerror: Rc::new(RefCell::new(None)),
		};
		doc.init();
		doc
	}

	// init, set handle
	fn init(&mut self) {
		self.handle = parse_wait;
	}

	// into root
	pub fn into_root(self) -> DocHolder {
		let doc = Rc::new(RefCell::new(self));
		doc.borrow_mut().root.borrow_mut().document = Some(Rc::clone(&doc));
		DocHolder { doc }
	}

	// parse with string
	pub fn parse(content: &str, options: ParseOptions) -> GenResult<DocHolder> {
		let mut doc = Doc::new();
		doc.parse_options = options;
		for c in content.chars() {
			doc.next(&c, content)?;
		}
		doc.eof(content)?;
		Ok(doc.into_root())
	}

	// parse file
	pub fn parse_file<P>(filename: P, options: ParseOptions) -> GenResult<DocHolder>
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
		let mut content = String::with_capacity(500);
		doc.parse_options = options;
		for line in file.lines() {
			let line_content = line.unwrap();
			content.push_str(&line_content);
			for c in line_content.chars() {
				doc.next(&c, &content)?;
			}
			doc.next(&'\n', &content)?;
		}
		doc.eof(&content)?;
		Ok(doc.into_root())
	}

	// chars to string
	fn chars_to_string(&self) -> String {
		chars_to_string(&self.prev_chars)
	}

	// clean the previous characters and return
	fn clean_chars_to_vec(&mut self) -> Vec<char> {
		let mut content: Vec<char> = Vec::with_capacity(self.prev_chars.len());
		content.append(&mut self.prev_chars);
		content
	}

	// set code_in
	fn set_code_in(&mut self, code_in: CodeTypeIn) {
		self.code_in = code_in;
		use CodeTypeIn::*;
		match code_in {
			Unkown => {
				self.handle = parse_wait;
			}
			Tag => {
				self.handle = parse_tag_name;
			}
			TagEnd => {
				self.handle = parse_tagend;
			}
			TextNode => {
				self.handle = parse_text;
			}
			HTMLScript | HTMLStyle | EscapeableRawText => {
				self.handle = parse_special_tag;
			}
			HTMLDOCTYPE => {
				self.handle = parse_doctype_name;
			}
			AbstractRoot => {
				self.handle = parse_wait;
			}
			Comment => {
				self.handle = parse_comment_or_cdata;
			}
			XMLCDATA => {
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

	// read chars one by one
	fn next(&mut self, c: &char, content: &str) -> HResult {
		let handle = self.handle;
		let _ = handle(self, *c, content)?;
		self.position.move_one();
		// check if special, and character is ok
		if let Some((special, tag_name)) = &self.in_special {
			special.is_ok(&self.code_in, tag_name, &c, &self.position, content)?;
		}
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
			// add cur node to parent's child nodes
			if let Some(childs) = &mut parent_node.childs {
				child.borrow_mut().index = childs.len();

				childs.push(child);
			} else {
				child.borrow_mut().index = 0;
				parent_node.childs = Some(vec![child]);
			}
			// set node root
			node.borrow_mut().root = Some(Rc::downgrade(&self.root));
		}
		// set special
		node.borrow_mut().special = match self.in_special {
			Some((special, _)) => Some(special),
			None => None,
		};
		// set current node to be new node
		self.current_node = Rc::clone(&node);
		// if is a tag node, add the tag node to chain nodes
		if node_type == Tag {
			self.chain_nodes.push(Rc::clone(&node));
		}
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
	}
	// set spaces between tag
	fn set_text_spaces_between(&mut self) {
		if let Some(text_node) = &mut self.check_textnode {
			text_node.borrow_mut().node_type = NodeType::SpacesBetweenTag;
			self.check_textnode = None;
		}
	}

	// fix unclosed tag
	fn fix_unclosed_tag(&mut self, unclosed: &[RefNode]) {
		for tag_node in unclosed {
			let mut end_tag: Option<Node> = None;
			if let Some(meta) = &tag_node.borrow_mut().meta {
				// set it's meta as auto fix
				meta.borrow_mut().auto_fix = true;
				// make end tag
				let tag_name = &meta.borrow().name;
				let mut end = Node::new(NodeType::TagEnd, self.position);
				end.content = Some(tag_name.clone());
				end.parent = Some(Rc::downgrade(&tag_node));
				end_tag = Some(end);
			}
			if let Some(end_tag) = end_tag {
				tag_node.borrow_mut().end_tag = Some(Rc::new(RefCell::new(end_tag)));
			}
		}
	}
	// fix unescaped left angle bracket
	fn fix_unescaped_lt(&mut self, c: char, context: &str) -> HResult {
		let mut chars = vec!['&', 'l', 't', ';'];
		let mut parent: Option<RefNode> = None;
		let mut need_parent = false;
		let node_type = self.current_node.borrow().node_type;
		let is_end_text = c == TAG_BEGIN_CHAR;
		if !is_end_text {
			chars.push(c);
		}
		match node_type {
			NodeType::Text | NodeType::SpacesBetweenTag => {
				// text node or spaces between
				if let Some(content) = &mut self.current_node.borrow_mut().content {
					let mut prev_chars: Vec<char> = Vec::with_capacity(content.len() + chars.len());
					prev_chars.append(content);
					prev_chars.append(&mut chars);
					self.prev_chars = prev_chars;
				}
				// change node type if spaces between
				if matches!(node_type, NodeType::SpacesBetweenTag) {
					self.current_node.borrow_mut().node_type = NodeType::Text;
				}
			}
			NodeType::Tag => {
				// tag, check if auto fix or self closing
				let is_closed = self
					.current_node
					.borrow()
					.meta
					.as_ref()
					.map_or(false, |meta| {
						let meta = &meta.borrow();
						meta.self_closed || meta.auto_fix
					});
				if !is_closed {
					parent = Some(Rc::clone(&self.current_node));
				} else {
					need_parent = true;
				}
			}
			_ => {
				need_parent = true;
			}
		}
		if need_parent {
			if !self.chain_nodes.is_empty() {
				let index = self.chain_nodes.len() - 1;
				parent = Some(Rc::clone(&self.chain_nodes[index]));
			} else {
				parent = Some(Rc::clone(&self.root));
			}
		}
		if let Some(parent) = &parent {
			let text_node = Node::new(NodeType::Text, self.mem_position);
			let current_node = Rc::new(RefCell::new(text_node));
			let mut parent = parent.borrow_mut();
			let childs = parent.childs.get_or_insert(Vec::new());
			current_node.borrow_mut().index = childs.len();
			childs.push(Rc::clone(&current_node));
			// set current node as text node
			self.current_node = current_node;
			// set prev chars
			self.prev_chars = chars;
		}
		self.repeat_whitespace = false;
		self.set_code_in(CodeTypeIn::TextNode);
		// if is end of text<the tag left character>
		// should move back one character
		if is_end_text {
			// execute the text node handle
			// the position index will keep because it doesn't call 'next' but `handle`
			let handle = self.handle;
			handle(self, c, context)?;
		}
		Ok(())
	}
	// is in svg or mathml
	fn check_special(&self) -> Option<&SpecialTag> {
		self.in_special.as_ref().map(|(special, _)| special)
	}
	// end of the doc
	fn eof(&mut self, context: &str) -> HResult {
		let cur_depth = self.chain_nodes.len();
		// check if all tags are closed correctly.
		if cur_depth > 1 {
			if !self.parse_options.auto_fix_unclosed_tag {
				let last_node = self.chain_nodes[cur_depth - 1].borrow();
				let begin_at = last_node.begin_at;
				let tag_name = chars_to_string(
					&last_node
						.meta
						.as_ref()
						.expect("tag node's meta must have")
						.borrow()
						.name,
				);
				return create_parse_error(ErrorKind::UnclosedTag(tag_name), begin_at, context);
			}
			// fix unclosed tags
			let unclosed = self.chain_nodes.split_off(1);

			self.fix_unclosed_tag(&unclosed);
		}
		// check and fix last node info.
		use CodeTypeIn::*;
		match self.code_in {
			TextNode => {
				let mut last_node = self.current_node.borrow_mut();
				last_node.content = Some(self.prev_chars.clone());
				if self.repeat_whitespace {
					last_node.node_type = NodeType::SpacesBetweenTag;
				}
				last_node.end_at = self.position;
			}
			Unkown | AbstractRoot => {
				// end ok
			}
			Tag => {
				if !self.parse_options.auto_fix_unclosed_tag {
					return create_parse_error(
						ErrorKind::UnclosedTag(format!("{:?}", self.code_in)),
						self.current_node.borrow().begin_at,
						context,
					);
				}
			}
			_ => {
				return create_parse_error(
					ErrorKind::UnclosedTag(format!("{:?}", self.code_in)),
					self.current_node.borrow().begin_at,
					context,
				);
			}
		}
		// set the root node's end position
		self.root.borrow_mut().end_at = self.position;
		Ok(())
	}
	// return error with doc.position
	fn error(&self, kind: ErrorKind, context: &str) -> HResult {
		create_parse_error(kind, self.position, context)
	}
	// set tag meta
	fn set_tag_meta(&mut self) -> bool {
		let name = self.clean_chars_to_vec();
		let lc_name = name
			.iter()
			.map(|ch| ch.to_ascii_lowercase())
			.collect::<Vec<char>>();
		let is_void = is_void_tag(&lc_name);
		let meta = TagMeta {
			name,
			attrs: Vec::with_capacity(5),
			lc_name_map: HashMap::with_capacity(5),
			is_void,
			..Default::default()
		};
		self.current_node.borrow_mut().meta = Some(RefCell::new(meta));
		is_void
	}
	// set tag code in
	fn set_tag_code_in(&mut self, code_in: TagCodeIn) {
		use TagCodeIn::*;
		match code_in {
			Wait | WaitValue | ValueEnd => self.handle = parse_tag_wait,
			Key => self.handle = parse_tag_attr_key,
			Value => self.handle = parse_tag_attr_value,
			KeyEnd => {}
		};
		if let Some(meta) = &self.current_node.borrow_mut().meta {
			meta.borrow_mut().code_in = code_in;
		}
	}
	// check tag code in
	fn is_tag_code_in(&self, code_in: &TagCodeIn) -> bool {
		let current_node = self.current_node.borrow();
		let tag_code_in = &current_node
			.meta
			.as_ref()
			.expect("Tag meta must set in parse_tag_name or parse_doctype_name")
			.borrow()
			.code_in;
		tag_code_in == code_in
	}
	// add attr key
	fn add_tag_attr_key(&mut self) {
		if let Some(meta) = &self.current_node.borrow_mut().meta {
			meta.borrow_mut().add_attr_key();
			meta.borrow_mut().code_in = TagCodeIn::Key;
		}
	}
	// set attr key
	fn set_tag_attr_key(&mut self) {
		let key = self.clean_chars_to_vec();
		if let Some(meta) = &self.current_node.borrow_mut().meta {
			let mut meta = meta.borrow_mut();
			let index = meta.attrs.len() - 1;
			let key_name = key
				.iter()
				.map(|ch| ch.to_ascii_lowercase())
				.collect::<String>();
			// end the attribute
			// insert lowercase attribute name to maps
			meta.lc_name_map.entry(key_name).or_insert(index);
			// set cur attr key
			meta.set_attr_key(key);
		}
	}
	// add a tag attribute value
	fn add_tag_attr_value(&mut self, quote: Option<char>) {
		let current_node = self.current_node.borrow_mut();
		let meta = current_node
			.meta
			.as_ref()
			.expect("The tag meta must not be empty");
		meta.borrow_mut().add_attr_value(quote);
		meta.borrow_mut().code_in = TagCodeIn::Value;
	}
	// set attr value
	fn set_tag_attr_value(&mut self, quote: Option<char>) {
		let value = self.clean_chars_to_vec();
		if let Some(meta) = &self.current_node.borrow_mut().meta {
			let mut meta = meta.borrow_mut();
			// set attribute value
			let attr = meta.set_attr_value(value, quote);
			// check the value if is a id value
			// store the id nodes to cache
			if let Some(id_name) = attr.check_if_id() {
				self
					.id_tags
					.borrow_mut()
					.insert(id_name, Rc::clone(&self.current_node));
			}
		}
	}
	// check tag type
	fn check_tag_type_do(&mut self) {
		use CodeTypeIn::*;
		let is_in_svg = self
			.check_special()
			.map_or(false, |special| matches!(special, SpecialTag::Svg));
		let lc_tag_name = self
			.current_node
			.borrow()
			.meta
			.as_ref()
			.expect("")
			.borrow()
			.name
			.iter()
			.map(|ch| ch.to_ascii_lowercase())
			.collect::<Vec<char>>();
		if is_script_or_style(&lc_tag_name, &None)
			|| (!is_in_svg && is_plain_text_tag(&lc_tag_name, &None))
		{
			// svg tags allow script and style tag, but title and textarea will treat as normal tag
			self.mem_position = self.position;
			let code_in = if is_equal_chars(&lc_tag_name, &SCRIPT_TAG_NAME, &None) {
				HTMLScript
			} else if is_equal_chars(&lc_tag_name, &STYLE_TAG_NAME, &None) {
				HTMLStyle
			} else {
				EscapeableRawText
			};
			self.set_code_in(code_in);
			// set detect chars
			let mut next_chars = vec!['<', END_SLASH_CHAR];
			next_chars.extend_from_slice(
				&self
					.current_node
					.borrow()
					.meta
					.as_ref()
					.expect("")
					.borrow()
					.name,
			);
			self.detect = Some(next_chars);
		} else {
			if self.in_special.is_none() {
				// not void elements will check if special
				self.in_special = if let Some(&special) = SPECIAL_TAG_MAP.get(&lc_tag_name) {
					Some((
						special,
						self
							.current_node
							.borrow()
							.meta
							.as_ref()
							.expect("")
							.borrow()
							.name
							.clone(),
					))
				} else {
					None
				}
			}
			self.set_code_in(Unkown);
		}
	}
}

// get an element from string node map
fn get_element(map: &StringNodeMap, query: &str) -> Option<RefNode> {
	map.get(query).map(|node| Rc::clone(node))
}
/// Doc holder
pub struct DocHolder {
	doc: RefDoc,
}

impl DocHolder {
	// render
	pub fn render(&self, options: &RenderOptions) -> String {
		chars_to_string(&self.borrow().root.borrow().build(options, false))
	}

	// render text
	pub fn render_text(&self, options: &RenderOptions) -> String {
		chars_to_string(&self.borrow().root.borrow().build(options, true))
	}

	// borrow
	pub fn borrow(&self) -> Ref<Doc> {
		self.doc.borrow()
	}

	// borrow mut
	pub fn get_root_node(&self) -> RefNode {
		Rc::clone(&self.borrow().root)
	}

	// get element by id
	pub fn get_element_by_id(&self, id: &str) -> Option<RefNode> {
		get_element(&self.borrow().id_tags.borrow(), id)
	}
}

impl From<RefDoc> for DocHolder {
	fn from(doc: RefDoc) -> Self {
		DocHolder { doc }
	}
}
