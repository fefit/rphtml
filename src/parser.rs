use crate::config::RenderOptions;
use lazy_static::lazy_static;
use serde::Serialize;
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
const TAG_BEGIN_CHAR: char = '<';
const TAG_END_CHAR: char = '>';

#[derive(Debug)]
pub struct ParseError {
  pub position: CodePosAt,
  pub kind: ErrorKind,
}

impl ParseError {
  pub fn new(kind: ErrorKind, position: CodePosAt) -> Self {
    ParseError { position, kind }
  }
}

// display parse error
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use ErrorKind::*;
    let position = self.position;
    let output = match &self.kind {
      WrongTag(tag) => format!("wrong tag '<{}' at {:?}", tag, position),
      WrongEndTag(tag) => format!("wrong end tag '</{}' at {:?}", tag, position),
      UnmatchedClosedTag(tag) => format!("unmatched tag '</{}>' at {:?}", tag, position),
      UnclosedTag(tag) => format!("unclosed tag '<{}' at {:?}", tag, position),
      WrongHtmlDoctype(c) => format!("wrong html doctype character '{}' at {:?}", c, position),
      NoSpaceBetweenAttr(c) => format!(
        "the tag's attribute '{}' should after a space.{:?}",
        c, position
      ),
      UnrecognizedTag(finded, maybe) => format!(
        "unrecognized tag '{}' at {:?}, do you mean '{}'",
        finded, position, maybe
      ),
      WrongTagIdentity(ident) => format!("wrong tag name '{}' at {:?}", ident, position),
      WrongRootTextNode(text) => format!("wrong text '{}...' in root node at {:?}", text, position),
      ChildInSpecialTag(tag, c) => format!(
        "wrong child tag '<{}' in tag '{}'  at {:?}",
        c, tag, position
      ),
      UnexpectedCharacter(c) => format!("unexpected character '{}' at {:?}", c, position),
      CommonError(msg) => msg.to_string(),
    };
    f.write_str(output.as_str())
  }
}

// impl trait Error
impl Error for ParseError {}

#[derive(Debug)]
pub enum ErrorKind {
  WrongTag(String),
  WrongEndTag(String),
  ChildInSpecialTag(String, char),
  UnmatchedClosedTag(String),
  UnexpectedCharacter(char),
  UnclosedTag(String),
  NoSpaceBetweenAttr(char),
  WrongHtmlDoctype(char),
  UnrecognizedTag(String, String),
  WrongTagIdentity(String),
  WrongRootTextNode(String),
  CommonError(String),
}

#[derive(PartialEq, Eq, Hash)]
pub enum DetectChar {
  Comment,
  DOCTYPE,
  CDATA,
  Script,
  Style,
}

lazy_static! {
  static ref DETECT_CHAR_MAP: HashMap<DetectChar, Vec<char>> = {
    use DetectChar::*;
    let mut map = HashMap::new();
    map.insert(Comment, vec!['-', '-']);
    map.insert(DOCTYPE, vec!['D', 'O', 'C', 'T', 'Y', 'P', 'E']);
    map.insert(CDATA, vec!['[', 'C', 'D', 'A', 'T', 'A', '[']);
    map.insert(Script, vec!['<', '/', 's', 'c', 'r', 'i', 'p', 't']);
    map.insert(Style, vec!['<', '/', 's', 't', 'y', 'l', 'e']);
    map
  };
  static ref VOID_ELEMENTS: Vec<&'static str> = vec![
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr"
  ];
  static ref SPECIAL_TAG_MAP: HashMap<&'static str, SpecialTag> = {
    use SpecialTag::*;
    let mut map = HashMap::new();
    for &tag in (&VOID_ELEMENTS).iter() {
      map.insert(tag, Void);
    }
    map.insert("pre", Pre);
    map.insert("textarea", EscapeableRawText);
    map.insert("title", EscapeableRawText);
    map.insert("svg", Svg);
    map.insert("math", MathML);
    map
  };
  static ref MUST_QUOTE_ATTR_CHARS: Vec<char> = vec!['"', '\'', '`', '=', '<', '>'];
}

#[derive(PartialEq, Debug, Serialize)]
pub enum NodeType {
  Comment,      // comment
  HTMLDOCTYPE,  // html doctype
  Tag,          // the start tag\self-closing tag\autofix empty tag
  TagEnd,       // the end tag node
  Text,         // text node
  AbstractRoot, // abstract root node
}

#[derive(PartialEq)]
enum CodeTypeIn {
  AbstractRoot,     // abstract root node,the begin node of document
  Unkown,           // wait for detect node
  UnkownTag,        // is a tag begin with '<', but need more diagnosis
  Tag,              // the start tag\self-closing tag\autofix empty tag
  TagEnd,           // the end tag
  ExclamationBegin, // tag begin with '!' maybe Comment|HTMLDOCTYPE
  Comment,          // comment tag
  HTMLDOCTYPE,      // html doctype
  HTMLScript,       // html script
  HTMLStyle,        // html css style
  TextNode,         // text node
}

pub fn is_identity(chars: &Vec<char>) -> bool {
  let mut is_first = true;
  let mut has_ns = false;
  for &c in chars {
    if is_first {
      if !(c.is_ascii_alphabetic() || c == '_') {
        return false;
      }
      is_first = false;
      continue;
    }
    if c.is_ascii_alphanumeric() || c == '-' {
      continue;
    }
    if !has_ns && c == '.' {
      has_ns = true;
      is_first = true;
      continue;
    }
    return false;
  }
  if is_first {
    false
  } else {
    true
  }
}

fn get_content(content: &Option<Vec<char>>) -> String {
  let content = content.as_ref().expect("content must not be empty");
  content.iter().collect()
}

/**
 * the doc's position
*/
#[derive(Copy, Clone, PartialEq, Serialize)]
pub struct CodePosAt {
  pub line_no: u32,
  pub col_no: u32,
}

impl fmt::Debug for CodePosAt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let output = format!("[line:{},col:{}]", self.line_no, self.col_no);
    f.write_str(output.as_str())
  }
}

impl CodePosAt {
  // new
  pub fn new(line_no: u32, col_no: u32) -> Self {
    CodePosAt { line_no, col_no }
  }
  // create a begin position
  pub fn begin() -> Self {
    CodePosAt::new(1, 0)
  }
  // jump to new line
  pub fn set_new_line(&mut self) {
    self.line_no += 1;
    self.col_no = 0;
  }
  // move to next col
  pub fn move_one(&mut self) {
    self.col_no += 1;
  }
  // get the next col position
  pub fn next_col(&self) -> Self {
    CodePosAt {
      line_no: self.line_no,
      col_no: self.col_no + 1,
    }
  }
}

/**
 * Attr
 * attribute data
 * if value is None, it's a boolean attribute
 * if key is None,it's a value with quote
 */
#[derive(Debug, Default, Serialize)]
pub struct Attr {
  pub key: Option<String>,
  pub value: Option<String>,
  pub quote: Option<char>,
  pub need_quote: bool,
}

impl Attr {
  // build attribute code
  pub fn build(&self, remove_quote: bool) -> String {
    let mut ret = String::with_capacity(20);
    let mut has_key = false;
    if let Some(key) = &self.key {
      ret.push_str(key);
      has_key = true;
    }
    if let Some(value) = &self.value {
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
      ret.push_str(value);
      if let Some(quote) = use_quote {
        ret.push(quote);
      }
    }
    ret
  }
}
/**
 * Tag
 * is_end: if the tag end with '>'
 * self_closed: if the tag is self-closing '/>'
 * auto_fix: if the tag either self-closing nor closed with a end tag, may auto fix by the parser
 * name: the tag name
 * attrs: the attribute list
 * attr_index: the current attribute index of the 'attrs'
*/
#[derive(Debug, Serialize)]
pub struct TagMeta {
  pub tag_in: TagCodeIn,
  pub is_end: bool,
  pub self_closed: bool,
  pub auto_fix: bool,
  pub name: String,
  pub attrs: Vec<Attr>,
  pub attr_index: i32,
  pub prev_is_key: bool,
  pub is_in_kv: bool,
  pub is_in_translate: bool,
}

impl TagMeta {
  pub fn get_name(&self, lowercase: bool) -> String {
    if lowercase {
      self.name.to_ascii_lowercase()
    } else {
      self.name.clone()
    }
  }
  pub fn get_attrs(&self, remove_quote: bool) -> String {
    let segs: Vec<String> = self
      .attrs
      .iter()
      .map(|attr| attr.build(remove_quote))
      .collect();
    if segs.len() > 0 {
      format!(" {}", segs.join(" "))
    } else {
      String::from("")
    }
  }
}

#[derive(PartialEq, Debug, Serialize)]
pub enum TagCodeIn {
  Wait,
  Key,
  Value,
  DoubleQuotedValue,
  SingleQuotedValue,
}

type RefNode<'a> = Rc<RefCell<Node<'a>>>;
/**
 * Node
 */
#[derive(Debug, Serialize)]
pub struct Node<'a> {
  pub tag_index: usize,             // if a tag node, add a index to the node
  pub depth: usize,                 // the node's depth in the document
  pub node_type: NodeType,          // the node's type
  pub begin_at: CodePosAt,          // the node's start position '<'
  pub end_at: CodePosAt,            // the node's end position '>'
  pub end_tag: Option<RefNode<'a>>, // the end tag </xx> of the tag node
  pub parent: Option<Weak<RefCell<Node<'a>>>>, // parent node, use weak reference,prevent reference loop
  pub content: Option<Vec<char>>,              // the content,for text/comment/style/script nodes
  pub childs: Option<Vec<RefNode<'a>>>,        // the child nodes
  pub meta: Option<RefCell<TagMeta>>,          // the tag node meta information
  pub special: Option<SpecialTag>,             // special
}

impl<'a> Node<'a> {
  // create a new node
  pub fn new(node_type: NodeType, code_at: CodePosAt) -> Self {
    return Node {
      node_type,
      begin_at: code_at,
      end_at: code_at,
      end_tag: None,
      parent: None,
      content: None,
      childs: None,
      meta: None,
      tag_index: 0,
      depth: 0,
      special: None,
    };
  }
  // build node
  fn build_node(&self, options: &RenderOptions, mut is_in_pre: bool) -> (String, bool) {
    let mut result = String::from("");
    use NodeType::*;
    match self.node_type {
      Text => {
        if !is_in_pre && options.minify_spaces {
          let mut prev_is_space = false;
          for &c in self.content.as_ref().unwrap().iter() {
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
        } else {
          let content = get_content(&self.content);
          result.push_str(content.as_str());
        }
      }
      Tag => {
        let meta = self
          .meta
          .as_ref()
          .expect("tag's meta data must have.")
          .borrow();
        let tagname = meta.get_name(options.lowercase_tagname);
        // check if is in pre, only check if not in pre
        is_in_pre = is_in_pre || {
          if options.lowercase_tagname {
            tagname == "pre"
          } else {
            tagname.to_lowercase() == "pre"
          }
        };
        let attrs = meta.get_attrs(options.remove_attr_quote);
        let tag = format!("<{}{}>", tagname, attrs);
        result.push_str(tag.as_str());
        // content for some special tags, such as style/script
        if let Some(_) = &self.content {
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
            is_in_pre = false;
          }
        } else {
          if is_in_pre && content.to_lowercase() == "pre" {
            is_in_pre = false;
          }
        }
        content = format!("</{}>", content);
        result.push_str(content.as_str());
      }
      HTMLDOCTYPE => {
        let meta = self
          .meta
          .as_ref()
          .expect("tag's meta data must have.")
          .borrow();
        let content = format!("<!DOCTYPE{}>", meta.get_attrs(options.remove_attr_quote));
        result.push_str(content.as_str());
      }
      Comment if !options.remove_comment => {
        // comment
        result.push_str(get_content(&self.content).as_str());
      }
      _ => {
        // otherwise, render nothing
      }
    }
    (result, is_in_pre)
  }
  // build node tree
  fn build_tree(&self, options: &RenderOptions, mut is_in_pre: bool) -> (String, bool) {
    let mut result = String::with_capacity(200);
    use NodeType::*;
    if self.node_type != AbstractRoot {
      let (content, now_in_pre) = self.build_node(options, is_in_pre);
      result.push_str(content.as_str());
      is_in_pre = now_in_pre;
    }
    if let Some(childs) = &self.childs {
      for child in childs {
        let (content, now_in_pre) = child.borrow().build_tree(options, is_in_pre);
        result.push_str(content.as_str());
        is_in_pre = now_in_pre;
      }
    }
    if let Some(end_tag) = &self.end_tag {
      let (content, now_in_pre) = end_tag.borrow().build_node(options, is_in_pre);
      result.push_str(content.as_str());
      is_in_pre = now_in_pre
    }
    (result, is_in_pre)
  }
  // build
  pub fn build(&self, options: &RenderOptions) -> String {
    let (content, _) = self.build_tree(options, false);
    content
  }
}

#[derive(Debug, Serialize, Hash, Clone, Copy)]
pub enum SpecialTag {
  Pre,
  Void,
  EscapeableRawText,
  MathML,
  Svg,
  Template,
}
/**
 * Doc
*/
pub struct Doc<'a> {
  code_in: CodeTypeIn,
  position: CodePosAt,
  mem_position: CodePosAt,
  mem_code_in: CodeTypeIn,
  detect: Option<DetectChar>,
  prev_chars: Vec<char>,
  prev_char: char,
  chain_nodes: Vec<RefNode<'a>>,
  current_node: RefNode<'a>,
  tag_index: usize,
  in_special: Option<(SpecialTag, &'static str)>,
  total_chars: usize,
  pub nodes: Vec<RefNode<'a>>,
  pub root: RefNode<'a>,
}

impl<'a> Doc<'a> {
  // create new parser
  pub fn new() -> Self {
    let node = Rc::new(RefCell::new(Node::new(
      NodeType::AbstractRoot,
      CodePosAt::begin(),
    )));
    let ref_node = Rc::clone(&node);
    let current_node = Rc::clone(&node);
    let root = Rc::clone(&node);
    let mut nodes = Vec::with_capacity(10);
    let mut chain_nodes = Vec::with_capacity(10);
    nodes.push(node);
    chain_nodes.push(ref_node);
    Doc {
      code_in: CodeTypeIn::AbstractRoot,
      position: CodePosAt::begin(),
      mem_position: CodePosAt::begin(),
      mem_code_in: CodeTypeIn::AbstractRoot,
      prev_char: ' ',
      prev_chars: Vec::with_capacity(200),
      nodes,
      chain_nodes,
      current_node,
      tag_index: 0,
      total_chars: 0,
      detect: None,
      in_special: None,
      root,
    }
  }
  // for serde, remove cycle reference
  pub fn to_json(&mut self) {
    for node in &mut self.nodes {
      let mut node = node.borrow_mut();
      node.parent = None;
    }
  }

  // parse with string
  pub fn parse(&mut self, content: &str) -> Result<RefNode<'a>, Box<dyn Error>> {
    for c in content.chars() {
      self.next(c)?;
    }
    self.eof()?;
    Ok(Rc::clone(&self.root))
  }

  // parse file
  pub fn parse_file<P>(&mut self, filename: P) -> Result<RefNode<'a>, Box<dyn Error>>
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
    for line in file.lines() {
      for c in line.unwrap().chars() {
        self.next(c)?;
      }
    }
    self.eof()?;
    Ok(Rc::clone(&self.root))
  }
  // gather previous characters
  fn chars_to_string(&self) -> String {
    self.prev_chars.iter().collect::<String>()
  }

  // read one char
  fn next(&mut self, c: char) -> Result<(), ParseError> {
    // add all CodeTypeIn enum item to namespace
    use CodeTypeIn::*;
    // check if it's a new node
    let mut new_node: Option<RefNode<'a>> = None;
    // if new node, check if tag node
    let mut is_new_tag = false;
    // check if the node end
    let mut is_node_end = false;
    // check if text node end
    let mut is_text_node_end = false;
    // match all posible nodes
    let cur_depth = self.chain_nodes.len();
    // check if indepent node, a end tag is not indepent
    let mut is_indepent_node = true;
    match self.code_in {
      TextNode | Unkown | AbstractRoot => {
        match c {
          // match the tag start '<'
          TAG_BEGIN_CHAR => {
            if self.code_in == TextNode {
              self.mem_code_in = TextNode;
              self.current_node.borrow_mut().content = Some(self.prev_chars.clone());
              is_node_end = true;
              is_text_node_end = true;
            }
            self.mem_position = self.position;
            self.code_in = UnkownTag;
            self.prev_chars.clear();
          }
          _ => {
            // only whitespace allowed
            if cur_depth == 1 && !c.is_ascii_whitespace() {
              return Err(ParseError::new(
                ErrorKind::WrongRootTextNode(c.to_string()),
                self.position,
              ));
            }
            if self.code_in != TextNode {
              // new text node
              new_node = Some(Rc::new(RefCell::new(Node::new(
                NodeType::Text,
                self.position,
              ))));
              self.code_in = TextNode;
              self.prev_chars.clear();
            }
            self.prev_chars.push(c);
          }
        }
      }
      Tag | HTMLDOCTYPE => {
        let is_whitespace = c.is_ascii_whitespace();
        let is_end = if is_whitespace {
          false
        } else {
          c == TAG_END_CHAR
        };
        let is_in_tag = self.code_in == Tag;
        let mut current_node = self.current_node.borrow_mut();
        let mut tag_name: String = String::from("");
        match current_node.meta.as_mut() {
          Some(meta) => {
            let mut meta = meta.borrow_mut();
            // meta is initial
            use TagCodeIn::*;
            match meta.tag_in {
              Wait | Key | Value => {
                let tag_in_wait = meta.tag_in == Wait;
                let is_key = meta.tag_in == Key;
                let mut is_end_key_or_value = false;
                // tag in wait, if prev char is '?' or '/', the current char must be the end of tag
                if tag_in_wait && (self.prev_char == '?' || self.prev_char == '/') && !is_end {
                  return Err(ParseError::new(
                    ErrorKind::UnexpectedCharacter(c),
                    self.position,
                  ));
                }
                if is_whitespace {
                  // if tag in wait state, ignore whitespaces, otherwise, is an end of key or value
                  if !tag_in_wait {
                    is_end_key_or_value = true;
                  }
                } else if is_end {
                  meta.is_end = true;
                  // self-closing tags
                  if tag_in_wait {
                    if self.prev_char == '/' {
                      meta.self_closed = true;
                      // remove from the chain nodes
                      self.chain_nodes.pop();
                    }
                  } else {
                    is_end_key_or_value = true;
                  }
                  // tag end
                  is_node_end = true;
                  // save tag name
                  if is_in_tag {
                    tag_name = meta.name.clone();
                  }
                } else {
                  match c {
                    '"' | '\'' if tag_in_wait => {
                      // if not in kv
                      if !meta.is_in_kv {
                        if !self.prev_char.is_ascii_whitespace() {
                          return Err(ParseError::new(
                            ErrorKind::NoSpaceBetweenAttr(c),
                            self.position,
                          ));
                        }
                        meta.attr_index += 1;
                      } else {
                        meta.is_in_kv = false;
                      }
                      // reset prev
                      meta.prev_is_key = false;
                      self.prev_chars.clear();
                      meta.tag_in = if c == '"' {
                        DoubleQuotedValue
                      } else {
                        SingleQuotedValue
                      };
                    }
                    '/' | '=' => {
                      if c == '/' && self.code_in != Tag {
                        return Err(ParseError::new(
                          ErrorKind::WrongTag(String::from("/")),
                          self.position,
                        ));
                      }
                      if c == '=' {
                        if meta.prev_is_key {
                          meta.is_in_kv = true;
                        } else {
                          return Err(ParseError::new(
                            ErrorKind::WrongTag(String::from("=")),
                            self.position,
                          ));
                        }
                      }
                      if !tag_in_wait {
                        // end the key or value
                        meta.tag_in = Wait;
                        is_end_key_or_value = true;
                      }
                    }
                    _ => {
                      if tag_in_wait {
                        self.prev_chars.clear();
                        if meta.is_in_kv {
                          meta.tag_in = Value;
                          meta.is_in_kv = false;
                          meta.prev_is_key = false;
                        } else {
                          meta.tag_in = Key;
                          // move attribute index
                          meta.attr_index += 1;
                          meta.prev_is_key = true;
                        }
                      }
                      self.prev_chars.push(c);
                    }
                  }
                }
                if is_end_key_or_value {
                  // if end of the key or value
                  let attr_index = meta.attr_index as usize;
                  if meta.attrs.len() <= attr_index {
                    meta.attrs.push(Default::default());
                  };
                  let cur_attr = meta.attrs.get_mut(attr_index).unwrap();
                  let value = self.chars_to_string();
                  if is_key {
                    cur_attr.key = Some(value);
                  } else {
                    cur_attr.value = Some(value);
                  };
                  self.prev_chars.clear();
                }
              }
              DoubleQuotedValue | SingleQuotedValue => {
                let is_in_translate = meta.is_in_translate;
                let attr_index = meta.attr_index as usize;
                if is_in_translate {
                  meta.is_in_translate = false;
                  self.prev_chars.push(c);
                } else {
                  if (meta.tag_in == DoubleQuotedValue && c == '"')
                    || (meta.tag_in == SingleQuotedValue && c == '\'')
                  {
                    meta.tag_in = Wait;
                    if meta.attrs.len() <= attr_index {
                      meta.attrs.push(Default::default());
                    }
                    let cur_attr = meta.attrs.get_mut(attr_index).unwrap();
                    cur_attr.quote = Some(c);
                    cur_attr.value = Some(self.chars_to_string());
                    self.prev_chars.clear();
                  } else {
                    let mut need_quote = false;
                    if c == '\\' {
                      meta.is_in_translate = true;
                      need_quote = true;
                    }
                    let cur_attr = meta.attrs.get_mut(attr_index).unwrap();
                    if !cur_attr.need_quote
                      && (need_quote
                        || c.is_ascii_whitespace()
                        || MUST_QUOTE_ATTR_CHARS.contains(&c))
                    {
                      cur_attr.need_quote = true;
                    }
                    self.prev_chars.push(c);
                  }
                }
              }
            }
          }
          None => {
            if is_whitespace || is_end {
              let cur_tag_name: String = self.chars_to_string();
              if is_whitespace {
                if self.code_in == HTMLDOCTYPE && cur_tag_name != "DOCTYPE" {
                  return Err(ParseError::new(
                    ErrorKind::WrongHtmlDoctype(c),
                    self.position,
                  ));
                }
              } else {
                if is_in_tag {
                  tag_name = cur_tag_name.clone();
                }
                match self.code_in {
                  HTMLDOCTYPE => {
                    return Err(ParseError::new(
                      ErrorKind::WrongHtmlDoctype(c),
                      self.position,
                    ))
                  }
                  Tag => {
                    // tag end
                    is_node_end = true;
                  }
                  _ => unreachable!("enum all"),
                }
              }
              if !is_identity(&self.prev_chars) {
                return Err(ParseError::new(
                  ErrorKind::WrongTagIdentity(cur_tag_name),
                  self.position,
                ));
              }
              let meta = TagMeta {
                name: cur_tag_name,
                attrs: Vec::new(),
                attr_index: -1,
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
              self.prev_chars.push(c);
            }
          }
        }
        if is_node_end {
          if is_in_tag {
            match tag_name.to_lowercase().as_str() {
              name @ "script" | name @ "style" => {
                self.mem_position = self.position;
                self.code_in = if name == "script" {
                  HTMLScript
                } else {
                  HTMLStyle
                };
              }
              name @ _ => {
                if !self.in_special.is_some() {
                  self.in_special = if let Some(&special) = SPECIAL_TAG_MAP.get(name) {
                    Some((special, Box::leak(tag_name.into_boxed_str())))
                  } else {
                    None
                  }
                }
                self.code_in = Unkown;
              }
            }
            self.prev_chars.clear();
          } else {
            self.code_in = Unkown;
          }
        }
      }
      TagEnd => {
        // the end tag
        if c == TAG_END_CHAR {
          let end_tag_name = self.chars_to_string();
          let fix_end_tag_name = end_tag_name.trim_end().to_lowercase();
          let mut is_tag_ended = false;
          let mut iter = self.chain_nodes.iter().rev();
          let mut back_num: usize = 0;
          let max_back_num: usize = self.chain_nodes.len();
          let is_allow_fix = max_back_num > 0;
          let mut empty_closed_tags: Vec<RefNode<'a>> = vec![];
          let mut real_tag_node: Option<RefNode<'a>> = None;
          while let Some(node) = iter.next() {
            if let Some(meta) = &node.borrow().meta {
              let tag_name = &meta.borrow().name;
              if tag_name == &end_tag_name || (tag_name.to_lowercase() == fix_end_tag_name) {
                is_tag_ended = true;
                real_tag_node = Some(Rc::clone(node));
                // todo: set the end tag
                break;
              }
              if is_allow_fix {
                // html void element: https://www.w3.org/TR/2011/WD-html-markup-20110113/syntax.html
                empty_closed_tags.push(Rc::clone(node));
              }
            }
            back_num += 1;
            // void elements
            if back_num > max_back_num {
              break;
            }
          }
          if is_tag_ended {
            is_node_end = true;
            self.code_in = Unkown;
            // end of special tag
            if self.in_special.is_some() && self.in_special.unwrap().1 == fix_end_tag_name {
              self.in_special = None;
            }
            // find the nearest tag,
            if let Some(tag) = &real_tag_node {
              // set end tag for the tag node
              tag.borrow_mut().end_tag = Some(Rc::clone(&self.current_node));
              let mut current_node = self.current_node.borrow_mut();
              current_node.parent = Some(Rc::downgrade(&tag));
              current_node.depth = tag.borrow().depth;
              current_node.content = Some(end_tag_name.chars().collect());
              // change the empty tags
              if empty_closed_tags.len() > 0 {
                for tag_node in empty_closed_tags.iter_mut() {
                  let mut tag_node = tag_node.borrow_mut();
                  // change the parent node
                  tag_node.parent = Some(Rc::downgrade(tag));
                  // set it's meta as auto fix
                  if let Some(meta) = &tag_node.meta {
                    let mut meta = meta.borrow_mut();
                    meta.auto_fix = true;
                  }
                  // change all childs's parent and clear
                  if let Some(childs) = &tag_node.childs {
                    if childs.len() > 0 {
                      for child_node in childs.iter() {
                        let mut child_node = child_node.borrow_mut();
                        child_node.parent = Some(Rc::downgrade(tag));
                        child_node.depth -= back_num;
                      }
                    }
                  }
                  // clear childs
                  tag_node.childs = None;
                }
              }
            }
            // remove the matched tag from the chain nodes
            self.chain_nodes.truncate(cur_depth - back_num - 1);
          } else {
            return Err(ParseError::new(
              ErrorKind::WrongEndTag(end_tag_name),
              self.current_node.borrow().begin_at,
            ));
          }
        } else {
          self.prev_chars.push(c);
        }
      }
      HTMLScript | HTMLStyle => {
        let detect_type = if self.code_in == HTMLScript {
          DetectChar::Script
        } else {
          DetectChar::Style
        };
        let end_tag = DETECT_CHAR_MAP.get(&detect_type).unwrap();
        let total_len = end_tag.len();
        let mut chars_len = self.prev_chars.len();
        // parse html script tag and style tag
        match c {
          '<' => {
            self.mem_position = self.position;
          }
          '>'
            if (chars_len == total_len && !self.prev_char.is_ascii_whitespace())
              || chars_len > total_len =>
          {
            let mut matched_num = 0;
            loop {
              let prev_char = self.prev_chars[chars_len - 1];
              // ignore end whitespace
              if prev_char.is_ascii_whitespace() {
                if matched_num != 0 {
                  break;
                }
              } else {
                if prev_char != end_tag[total_len - matched_num - 1] {
                  break;
                }
                matched_num += 1;
              }
              chars_len -= 1;
              if chars_len <= 0 || matched_num == total_len {
                break;
              }
            }
            if matched_num == total_len {
              let end_at = self.mem_position;
              // find the matched
              let end_tag_name = self.prev_chars.split_off(chars_len).split_off(2);
              // add an end tag
              let mut end = Node::new(NodeType::TagEnd, end_at);
              end.end_at = self.position.next_col();
              end.content = Some(end_tag_name);
              end.depth = cur_depth;
              end.parent = Some(Rc::downgrade(&self.current_node));
              // set tag node's content, end_tag
              let node = Rc::new(RefCell::new(end));
              let mut current_node = self.current_node.borrow_mut();
              current_node.end_tag = Some(Rc::clone(&node));
              current_node.content = Some(self.prev_chars.clone());
              self.nodes.push(node);
              // remove current tag
              self.chain_nodes.pop();
              self.code_in = Unkown;
              self.detect = None;
            }
          }
          _ => {}
        }
        self.prev_chars.push(c);
      }
      Comment => {
        // comment node
        let mut is_end = false;
        const END_SYMBOL: char = '-';
        if c == '>' && self.prev_char == END_SYMBOL {
          let total_len = self.prev_chars.len();
          if total_len >= 2 {
            let prev_last_char = self.prev_chars[total_len - 2];
            if prev_last_char == END_SYMBOL {
              is_end = true;
              self.current_node.borrow_mut().content = Some(self.prev_chars.clone());
              is_node_end = true;
              self.code_in = Unkown;
            }
          }
        }
        if !is_end {
          self.prev_chars.push(c);
        }
      }
      UnkownTag => {
        let begin_at = self.mem_position;
        // check the tag type
        match c {
          'a'..='z' | 'A'..='Z' => {
            // special tags not allowd child tags
            if self.in_special.is_some() {
              return Err(ParseError::new(
                ErrorKind::ChildInSpecialTag(self.in_special.unwrap().1.to_string(), c),
                begin_at,
              ));
            }
            // new tag node, add tag_index
            let mut inner_node = Node::new(NodeType::Tag, begin_at);
            inner_node.tag_index = self.tag_index + 1;
            new_node = Some(Rc::new(RefCell::new(inner_node)));
            is_new_tag = true;
            self.code_in = Tag;
            self.tag_index += 1;
            self.prev_chars.push(c);
          }
          '/' => {
            // new tag end
            new_node = Some(Rc::new(RefCell::new(Node::new(NodeType::TagEnd, begin_at))));
            is_indepent_node = false;
            self.code_in = TagEnd;
          }
          '!' => {
            // Comment|DOCTYPE
            self.code_in = ExclamationBegin;
            self.mem_position = begin_at;
          }
          _ => {
            return Err(ParseError::new(
              ErrorKind::WrongTag(c.to_string()),
              begin_at,
            ));
          }
        };
      }
      ExclamationBegin => {
        // maybe Comment | DOCTYPE<HTML>
        let begin_at = self.mem_position;
        if let Some(detect) = &self.detect {
          let next_chars = DETECT_CHAR_MAP.get(detect).unwrap();
          let total_len = self.prev_chars.len();
          let actual_len = next_chars.len();
          if total_len < actual_len {
            let cur_should_be = next_chars.get(total_len).unwrap();
            if cur_should_be == &c {
              if total_len == actual_len - 1 {
                match c {
                  '-' => {
                    self.code_in = Comment;
                    // new comment node
                    new_node = Some(Rc::new(RefCell::new(Node::new(
                      NodeType::Comment,
                      begin_at,
                    ))));
                  }
                  'E' => {
                    self.code_in = HTMLDOCTYPE;
                    // new html doctype node
                    new_node = Some(Rc::new(RefCell::new(Node::new(
                      NodeType::HTMLDOCTYPE,
                      begin_at,
                    ))));
                  }
                  _ => unreachable!(),
                };
                self.detect = None;
              }
            } else {
              return Err(ParseError::new(
                ErrorKind::UnrecognizedTag(
                  self.chars_to_string(),
                  next_chars.iter().collect::<String>(),
                ),
                begin_at,
              ));
            }
          }
        } else {
          match c {
            '-' => {
              self.detect = Some(DetectChar::Comment);
            }
            'D' => {
              self.detect = Some(DetectChar::DOCTYPE);
            }
            _ => {
              return Err(ParseError::new(
                ErrorKind::WrongTag(self.chars_to_string()),
                begin_at,
              ));
            }
          };
        }
        self.prev_chars.push(c);
      }
    }
    // set the end tag position
    if is_node_end {
      let mut current_node = self.current_node.borrow_mut();
      current_node.end_at = if is_text_node_end {
        self.position
      } else {
        self.position.next_col()
      };
      if is_indepent_node {
        current_node.depth = cur_depth;
      }
    }
    // do with the position
    {
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
      }
    }
    // has a new node
    if let Some(node) = new_node {
      if is_indepent_node {
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
      // set current node to be new node
      self.current_node = Rc::clone(&node);
      // if is a tag node, add the tag node to chain nodes
      if is_new_tag {
        self.chain_nodes.push(Rc::clone(&node));
      }
      // add cur node to parent's child nodes
      self.nodes.push(node);
    }
    // set the previous char
    self.prev_char = c;
    // add total chars
    self.total_chars += 1;
    // parse ok
    Ok(())
  }
  // end of the doc
  fn eof(&mut self) -> Result<(), ParseError> {
    let cur_depth = self.chain_nodes.len();
    // check if tags are all closed correctly.
    if cur_depth > 1 {
      let last_node = self.chain_nodes[cur_depth - 1].borrow();
      let begin_at = last_node.begin_at;
      let name = &last_node.meta.as_ref().unwrap().borrow().name;
      return Err(ParseError::new(
        ErrorKind::UnclosedTag(name.to_owned()),
        begin_at,
      ));
    }
    // fix last node depth
    let mut last_node = self.nodes.last().unwrap().borrow_mut();
    if last_node.node_type == NodeType::Text {
      last_node.depth = cur_depth;
    }
    Ok(())
  }
  // render
  pub fn render(&self, options: &RenderOptions) -> String {
    let mut result = String::with_capacity(self.total_chars);
    let mut is_in_pre = false;
    for node in &self.nodes[1..] {
      let (content, now_in_pre) = node.borrow().build_node(options, is_in_pre);
      result.push_str(content.as_str());
      is_in_pre = now_in_pre;
    }
    result
  }
  // render tree
  pub fn render_tree(&self, options: &RenderOptions) -> String {
    self.root.borrow().build(options)
  }
}
