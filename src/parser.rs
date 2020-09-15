use std::cell::RefCell;
use std::error::Error;
use std::rc::{Rc, Weak};
const TAG_BEGIN_CHAR: char = '<';
const TAG_END_CHAR: char = '>';
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
  HTMLDoctype,  // html doctype
  XMLDeclare,   // xml declare
  XMLCDATA,     // xml cdata
  Tag,          // the start tag\self-closing tag\autofix empty tag
  TagEnd,       // the end tag node
  Text,         // text node
  AbstractRoot, // abstract root node
}

#[derive(PartialEq)]
enum CodeTypeIn {
  AbstractRoot,         // abstract root node,the begin node of document
  Unkown,               // same as abstract root,not in any other node
  UnkownTag(CodePosAt), // is a tag begin with '<', but need more diagnosis
  Tag,                  // the start tag\self-closing tag\autofix empty tag
  TagEnd,               // the end tag
  ExclamationBegin(CodePosAt, Option<Vec<char>>), // tag begin with '!' maybe Comment|XMLCDATA|HTMLDoctype
  Comment,                                        // comment tag
  HTMLDoctype,                                    // html doctype
  XMLCDATA,                                       // xml cdata data
  XMLDeclare,                                     // xml declare
  TextNode,                                       // text node
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CodePosAt {
  pub line_no: u32,
  pub col_no: u32,
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
 * is_closed: if the tag is closed
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
  pub is_closed: bool,
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
  pub tag_index: u32,
  pub node_type: NodeType,                     // the node's type
  pub start_at: CodePosAt,                     // the node's start position '<'
  pub end_at: CodePosAt,                       // the node's end position '>'
  pub end_tag: Option<RefNode<'a>>,            // the end tag </xx> of the tag node
  pub parent: Option<Weak<RefCell<Node<'a>>>>, // parent node, use weak reference,prevent reference loop
  pub content: Option<Vec<char>>,              // the content,for text node/xml cdata/comment
  pub childs: Option<Vec<RefNode<'a>>>,        // the child nodes
  pub meta: Option<RefCell<TagMeta>>,
}

impl<'a> Node<'a> {
  // create a new node
  pub fn new(node_type: NodeType, code_at: CodePosAt) -> Self {
    return Node {
      node_type,
      start_at: code_at,
      end_at: code_at,
      end_tag: None,
      parent: None,
      content: None,
      childs: None,
      meta: None,
      tag_index: 0,
    };
  }
}

/**
 * Doc
*/
pub struct Doc<'a> {
  code_in: CodeTypeIn,
  position: CodePosAt,
  prev_chars: Vec<char>,
  prev_char: char,
  chain_nodes: Vec<RefNode<'a>>,
  current_node: RefNode<'a>,
  tag_index: u32,
  pub parser_type: ParserType,
  pub nodes: Vec<RefNode<'a>>,
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
    let mut nodes = Vec::with_capacity(10);
    let mut chain_nodes = Vec::with_capacity(10);
    nodes.push(node);
    chain_nodes.push(ref_node);
    Doc {
      code_in: CodeTypeIn::AbstractRoot,
      position: CodePosAt::begin(),
      prev_char: ' ',
      prev_chars: Vec::with_capacity(30),
      parser_type,
      nodes,
      chain_nodes,
      current_node,
      tag_index: 0,
    }
  }
  // parse with string
  pub fn parse(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
    for c in content.chars() {
      self.next(c)?;
    }
    for (index, node) in self.nodes.iter().enumerate() {
      println!("index:{}, node: {:?}", index, node);
    }
    Ok(())
  }
  fn chars_to_string(&self) -> String {
    self.prev_chars.iter().collect::<String>()
  }
  // read one char
  fn next(&mut self, c: char) -> Result<(), Box<dyn Error>> {
    let is_html = self.parser_type == ParserType::HTML;
    let is_xml = !is_html;
    // add all CodeTypeIn enum item to namespace
    use CodeTypeIn::*;
    // check if it's a new node
    let mut new_node: Option<RefNode<'a>> = None;
    // if new node, check if tag node
    let mut is_new_tag = false;
    match self.code_in {
      TextNode | Unkown | AbstractRoot => {
        match c {
          // match the tag start '<'
          TAG_BEGIN_CHAR => {
            if self.code_in == TextNode {
              let mut current_node = self.current_node.borrow_mut();
              current_node.content = Some(self.prev_chars.clone());
              current_node.end_at = self.position;
            }
            self.code_in = UnkownTag(self.position);
            self.prev_chars.clear();
          }
          _ => {
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
      Tag | HTMLDoctype | XMLDeclare => {
        let is_whitespace = c.is_ascii_whitespace();
        let is_end = if is_whitespace {
          false
        } else {
          c == TAG_END_CHAR
        };
        let mut current_node = self.current_node.borrow_mut();
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
                      meta.is_closed = true;
                      meta.self_closed = true;
                      // remove from the chain nodes
                      self.chain_nodes.pop();
                    }
                  } else {
                    is_end_key_or_value = true;
                  }
                  self.code_in = Unkown;
                } else {
                  match c {
                    '"' | '\'' if tag_in_wait => {
                      // if not in kv
                      if !meta.is_in_kv {
                        if !self.prev_char.is_ascii_whitespace() {
                          panic!("属性值之间缺少空格");
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
                let is_in_kv = meta.is_in_kv;
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
                    println!("cur_attr,{:?},is_in_kv:{}", cur_attr, is_in_kv);
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
              let tag_name: String = self.prev_chars.iter().collect();
              if is_whitespace {
                if self.code_in == XMLDeclare && tag_name != "xml" {
                  panic!("wrong xml declare:{}", tag_name);
                } else if self.code_in == HTMLDoctype && tag_name != "DOCTYPE" {
                  panic!("wrong html doctype:{}", tag_name);
                }
              } else {
                match self.code_in {
                  XMLDeclare => panic!("wrong xml declare"),
                  HTMLDoctype => panic!("wrong html doctype"),
                  Tag => {
                    self.code_in = Unkown;
                  }
                  _ => unreachable!("enum all"),
                }
              }
              if !is_identity(&self.prev_chars, &self.parser_type) {
                panic!("un correct identity：{}", tag_name);
              }
              let meta = TagMeta {
                name: tag_name,
                attrs: Vec::new(),
                attr_index: -1,
                auto_fix: false,
                self_closed: false,
                is_closed: false,
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
      }
      TagEnd => {
        // the end tag
        if c == TAG_END_CHAR {
          let end_tag_name = self.chars_to_string();
          let mut is_tag_ended = false;
          let mut iter = self.chain_nodes.iter().rev();
          let mut back_num: u32 = 0;
          let max_back_num = 1;
          let is_allow_fix = max_back_num == 1;
          let mut empty_closed_tags: Vec<RefNode<'a>> = vec![];
          let mut real_parent_node: Option<RefNode<'a>> = None;
          while let Some(current_node) = iter.next() {
            if let Some(meta) = &current_node.borrow().meta {
              let tag_name = &meta.borrow().name;
              if tag_name == &end_tag_name {
                is_tag_ended = true;
                real_parent_node = Some(Rc::clone(current_node));
                // todo: set the end tag
                break;
              } else if is_allow_fix {
                empty_closed_tags.push(Rc::clone(current_node));
              }
            }
            back_num += 1;
            if back_num >= max_back_num {
              break;
            }
          }
          if is_tag_ended {
            self.code_in = Unkown;
            // find the nearest tag,
            if empty_closed_tags.len() > 0 {
              if let Some(parent_node) = &real_parent_node {
                for tag_node in empty_closed_tags.iter_mut() {
                  // change the parent node
                  tag_node.borrow_mut().parent = Some(Rc::downgrade(parent_node));
                  // change all childs's parent and clear
                  if let Some(childs) = &tag_node.borrow_mut().childs {
                    if childs.len() > 0 {
                      for child_node in childs.iter() {
                        child_node.borrow_mut().parent = Some(Rc::downgrade(parent_node));
                      }
                    }
                  }
                  tag_node.borrow_mut().childs = None;
                }
              }
            }
          } else {
            panic!("错误的结束标签{}", end_tag_name);
          }
        } else {
          self.prev_chars.push(c);
        }
      }
      Comment | XMLCDATA => {
        // comment node or xml cdata node
        let mut is_end = false;
        let end_symbol = if self.code_in == Comment { ']' } else { '-' };
        if c == '>' && self.prev_char == end_symbol && self.prev_chars.len() >= 2 {
          let total_len = self.prev_chars.len();
          let prev_last_char = self.prev_chars[total_len - 2];
          if prev_last_char == end_symbol {
            is_end = true;
            let mut current_node = self.current_node.borrow_mut();
            current_node.end_at = self.position;
            self.code_in = Unkown;
          }
        }
        if !is_end {
          self.prev_chars.push(c);
        }
      }
      UnkownTag(begin_at) => {
        // check the tag type
        match c {
          'a'..='z' | 'A'..='Z' | '_' => {
            // only xml tag can begin with underscore
            if c == '_' && self.parser_type != ParserType::XML {
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
              panic!("无法识别的html标签")
            }
          }
          '!' => {
            // Comment|DOCTYPE|XMLCDATA
            self.code_in = ExclamationBegin(begin_at, None);
          }
          _ => {
            panic!("wrong tag in {:?}, <{}", begin_at, c);
          }
        };
      }
      ExclamationBegin(begin_at, ref char_queue) => {
        // maybe Comment | CDATA<XML> | DOCTYPE<HTML>
        if let Some(next_chars) = char_queue {
          let total_len = self.prev_chars.len();
          let actual_len = next_chars.len();
          if total_len < actual_len {
            let cur_should_be = next_chars.get(total_len).unwrap();
            if *cur_should_be == c {
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
                    self.code_in = HTMLDoctype;
                    // new html doctype node
                    new_node = Some(Rc::new(RefCell::new(Node::new(
                      NodeType::HTMLDoctype,
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
              self.code_in = ExclamationBegin(begin_at, Some(vec!['-', '-']));
            }
            'D' if self.parser_type == ParserType::HTML => {
              self.code_in =
                ExclamationBegin(begin_at, Some(vec!['D', 'O', 'C', 'T', 'Y', 'P', 'E']));
            }
            '[' if self.parser_type == ParserType::XML => {
              self.code_in =
                ExclamationBegin(begin_at, Some(vec!['[', 'C', 'D', 'A', 'T', 'A', '[']));
            }
            _ => {
              panic!("unrecognized tat <!{}", c);
            }
          };
        }
        self.prev_chars.push(c);
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
      {
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
      let current_node = Rc::clone(&node);
      self.current_node = current_node;
      // if is a tag node, add the tag node to chain nodes
      if is_new_tag {
        self.chain_nodes.push(Rc::clone(&node));
      }
      // add cur node to parent's child nodes
      self.nodes.push(node);
    }
    // set the previous char
    self.prev_char = c;
    Ok(())
  }
}
