use lazy_static::lazy_static;
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

#[derive(PartialEq, Eq, Hash)]
pub enum DetectChar {
  Comment,
  DOCTYPE,
  CDATA,
  Script,
  Style,
}

lazy_static! {
  static ref DETECTCHARMAP: HashMap<DetectChar, Vec<char>> = {
    use DetectChar::*;
    let mut map = HashMap::new();
    map.insert(Comment, vec!['-', '-']);
    map.insert(DOCTYPE, vec!['D', 'O', 'C', 'T', 'Y', 'P', 'E']);
    map.insert(CDATA, vec!['[', 'C', 'D', 'A', 'T', 'A', '[']);
    map.insert(Script, vec!['<', '/', 's', 'c', 'r', 'i', 'p', 't']);
    map.insert(Style, vec!['<', '/', 's', 't', 'y', 'l', 'e']);
    map
  };
}

/**
 * ParserType
 * HTML: for html
 * XML: for xml
 */
#[derive(PartialEq)]
pub enum ParserType {
  HTML,
  XML,
}

#[derive(PartialEq, Debug)]
pub enum NodeType {
  Comment,      // comment
  HTMLDOCTYPE,  // html doctype
  XMLDeclare,   // xml declare
  XMLCDATA,     // xml cdata
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
  ExclamationBegin, // tag begin with '!' maybe Comment|XMLCDATA|HTMLDOCTYPE
  Comment,          // comment tag
  HTMLDOCTYPE,      // html doctype
  HTMLScript,       // html script
  HTMLStyle,        // html css style
  XMLCDATA,         // xml cdata data
  XMLDeclare,       // xml declare
  TextNode,         // text node
}

pub fn is_identity(chars: &Vec<char>, parser_type: &ParserType) -> bool {
  let mut is_first = true;
  match parser_type {
    ParserType::HTML => true,
    ParserType::XML => {
      let mut has_ns = false;
      for &c in chars {
        if is_first {
          if !(c.is_ascii_alphabetic() || c == '_') {
            return false;
          }
          is_first = false;
          continue;
        }
        if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
          continue;
        }
        if !has_ns && c == ':' {
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
  }
}

/**
 * the doc's position
*/
#[derive(Copy, Clone, PartialEq)]
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
#[derive(Debug, Default)]
pub struct Attr {
  pub key: Option<String>,
  pub value: Option<String>,
  pub quote: Option<String>,
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
#[derive(Debug)]
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

#[derive(PartialEq, Debug)]
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
#[derive(Debug)]
pub struct Node<'a> {
  pub tag_index: usize,             // if a tag node, add a index to the node
  pub depth: usize,                 // the node's depth in the document
  pub node_type: NodeType,          // the node's type
  pub begin_at: CodePosAt,          // the node's start position '<'
  pub end_at: CodePosAt,            // the node's end position '>'
  pub end_tag: Option<RefNode<'a>>, // the end tag </xx> of the tag node
  pub parent: Option<Weak<RefCell<Node<'a>>>>, // parent node, use weak reference,prevent reference loop
  pub content: Option<Vec<char>>,              // the content,for text node/xml cdata/comment
  pub childs: Option<Vec<RefNode<'a>>>,        // the child nodes
  pub meta: Option<RefCell<TagMeta>>,          // the tag node meta information
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
    };
  }
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
  pub parser_type: ParserType,
  pub nodes: Vec<RefNode<'a>>,
  pub root: RefNode<'a>,
}

impl<'a> Doc<'a> {
  // create new parser
  pub fn new(parser_type: ParserType) -> Self {
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
      parser_type,
      nodes,
      chain_nodes,
      current_node,
      tag_index: 0,
      detect: None,
      root,
    }
  }
  // parse with string
  pub fn parse(&mut self, content: &str) {
    for c in content.chars() {
      self.next(c);
    }
    self.eof();
  }

  // parse file
  pub fn parse_file<P>(&mut self, filename: P) -> Result<(), Box<dyn Error>>
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
        self.next(c);
      }
    }
    self.eof();
    Ok(())
  }
  // gather previous charecters
  fn chars_to_string(&self) -> String {
    self.prev_chars.iter().collect::<String>()
  }

  // read one char
  fn next(&mut self, c: char) {
    let is_html = self.parser_type == ParserType::HTML;
    let is_xml = !is_html;
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
              panic!(
                "a text node '{}...' should inside a tag node, at {:?}",
                c, self.position
              );
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
      Tag | HTMLDOCTYPE | XMLDeclare => {
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
                  panic!("wrong label {}{} at {:?}", self.prev_char, c, self.position);
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
                          panic!(
                            "no whitespace in tag '<{}' between attributes at {:?}",
                            meta.name, self.position
                          );
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
                    '?' | '/' | '=' => {
                      if c == '?' && self.code_in != XMLDeclare {
                        panic!("wrong tag ?");
                      } else if c == '/' && self.code_in != Tag {
                        panic!("wrong tag end /");
                      } else if c == '=' {
                        if meta.prev_is_key {
                          meta.is_in_kv = true;
                        } else {
                          panic!("wrong =");
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
                    cur_attr.quote = Some(c.to_string());
                    cur_attr.value = Some(self.chars_to_string());
                    self.prev_chars.clear();
                  } else {
                    if c == '\\' {
                      meta.is_in_translate = true;
                    }
                    self.prev_chars.push(c);
                  }
                }
              }
            }
          }
          None => {
            if is_whitespace || is_end {
              let cur_tag_name: String = self.prev_chars.iter().collect();
              if is_whitespace {
                if self.code_in == XMLDeclare && cur_tag_name != "xml" {
                  panic!("wrong xml declare:{}", cur_tag_name);
                } else if self.code_in == HTMLDOCTYPE && cur_tag_name != "DOCTYPE" {
                  panic!("wrong html doctype:{}", cur_tag_name);
                }
              } else {
                if is_in_tag {
                  tag_name = cur_tag_name.clone();
                }
                match self.code_in {
                  XMLDeclare => panic!("wrong xml declare"),
                  HTMLDOCTYPE => panic!("wrong html doctype"),
                  Tag => {
                    // tag end
                    is_node_end = true;
                  }
                  _ => unreachable!("enum all"),
                }
              }
              if !is_identity(&self.prev_chars, &self.parser_type) {
                panic!("incorrect identity：{}", cur_tag_name);
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
          if is_in_tag && is_html {
            match tag_name.to_lowercase().as_str() {
              name @ "script" | name @ "style" => {
                self.mem_position = self.position;
                self.code_in = if name == "script" {
                  HTMLScript
                } else {
                  HTMLStyle
                };
              }
              _ => self.code_in = Unkown,
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
          let mut is_tag_ended = false;
          let mut iter = self.chain_nodes.iter().rev();
          let mut back_num: usize = 0;
          let max_back_num: usize = match self.parser_type {
            ParserType::HTML => self.chain_nodes.len(),
            ParserType::XML => 0,
          };
          let is_allow_fix = max_back_num > 0;
          let mut empty_closed_tags: Vec<RefNode<'a>> = vec![];
          let mut real_tag_node: Option<RefNode<'a>> = None;
          while let Some(node) = iter.next() {
            if let Some(meta) = &node.borrow().meta {
              let tag_name = &meta.borrow().name;
              if tag_name == &end_tag_name
                || (is_html && tag_name.to_lowercase() == end_tag_name.trim_end().to_lowercase())
              {
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
            // in xml, not allowed a void element tag nor self-closing nor end tag
            if back_num > max_back_num {
              break;
            }
          }
          if is_tag_ended {
            is_node_end = true;
            self.code_in = Unkown;
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
            panic!(
              "wrong end tag '</{}>' at {:?}",
              end_tag_name,
              self.current_node.borrow().begin_at
            );
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
        let end_tag = DETECTCHARMAP.get(&detect_type).unwrap();
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
      Comment | XMLCDATA => {
        // comment node or xml cdata node
        let mut is_end = false;
        let end_symbol = if self.code_in == Comment { '-' } else { ']' };
        if c == '>' && self.prev_char == end_symbol && self.prev_chars.len() >= 2 {
          let total_len = self.prev_chars.len();
          let prev_last_char = self.prev_chars[total_len - 2];
          if prev_last_char == end_symbol {
            is_end = true;
            self.current_node.borrow_mut().content = Some(self.prev_chars.clone());
            is_node_end = true;
            self.code_in = Unkown;
          }
        }
        if !is_end {
          self.prev_chars.push(c);
        }
      }
      UnkownTag => {
        let prev_is_textnode = self.mem_code_in == TextNode;
        let begin_at = self.mem_position;
        if prev_is_textnode && is_xml && c != '/' {
          // xml only allow pure text node
          panic!("wrong tag node in a text node at {:?}", self.position);
        }
        // check the tag type
        match c {
          'a'..='z' | 'A'..='Z' | '_' => {
            // only xml tag can begin with underscore
            if c == '_' && !is_xml {
              panic!("wrong start tag <_ as {:?}", begin_at);
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
          '?' => {
            if is_xml {
              // new xml declare
              new_node = Some(Rc::new(RefCell::new(Node::new(
                NodeType::XMLDeclare,
                begin_at,
              ))));
              self.code_in = XMLDeclare;
            } else {
              panic!("unrecognized tag <? at {:?}", begin_at);
            }
          }
          '!' => {
            // Comment|DOCTYPE|XMLCDATA
            self.code_in = ExclamationBegin;
            self.mem_position = begin_at;
          }
          _ => {
            panic!("wrong tag in {:?}, <{}", begin_at, c);
          }
        };
      }
      ExclamationBegin => {
        // maybe Comment | CDATA<XML> | DOCTYPE<HTML>
        let begin_at = self.mem_position;
        if let Some(detect) = &self.detect {
          let next_chars = DETECTCHARMAP.get(detect).unwrap();
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
                  'A' => {
                    self.code_in = XMLCDATA;
                    // new xml cdata
                    new_node = Some(Rc::new(RefCell::new(Node::new(
                      NodeType::XMLCDATA,
                      begin_at,
                    ))));
                  }
                  _ => unreachable!(),
                };
                self.detect = None;
              }
            } else {
              panic!(
                "wrong label at {:?}，do you mean the label '<!{}'",
                begin_at,
                next_chars.iter().collect::<String>()
              );
            }
          }
        } else {
          match c {
            '-' => {
              self.detect = Some(DetectChar::Comment);
            }
            'D' if is_html => {
              self.detect = Some(DetectChar::DOCTYPE);
            }
            '[' if is_xml => {
              self.detect = Some(DetectChar::CDATA);
            }
            _ => {
              panic!("unrecognized tag '<!{}' at {:?}", c, begin_at);
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
  }
  // end of the doc
  fn eof(&mut self) {
    let cur_depth = self.chain_nodes.len();
    // check if tags are all closed correctly.
    if cur_depth > 1 {
      let last_node = self.chain_nodes[cur_depth - 1].borrow();
      let begin_at = last_node.begin_at;
      let name = &last_node.meta.as_ref().unwrap().borrow().name;
      panic!("unclosed tag '{}' at {:?}", name, begin_at)
    }
    // fix last node depth
    let mut last_node = self.nodes.last().unwrap().borrow_mut();
    if last_node.node_type == NodeType::Text {
      last_node.depth = cur_depth;
    }
  }
}
