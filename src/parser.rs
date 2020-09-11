use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
const TAG_BEGIN_CHAR: char = '<';
const TAG_END_CHAR: char = '>';
/**
 * ParserType 解析器类型
 * HTML: for html
 * XML: for xml
 */
#[derive(PartialEq)]
pub enum ParserType {
  HTML,
  XML,
}
#[derive(PartialEq)]
pub enum NodeType {
  Comment,      // 注释节点
  HTMLDoctype,  // HTML的Doctype声明
  XMLDeclare,   // XML声明头
  XMLCDATA,     // XMLCDATA数据
  Tag,          // Tag
  TagEnd,       // Tag 结束标签
  Text,         // 文本节点
  AbstractRoot, // 虚拟根节点，用于文档开头等
}

#[derive(PartialEq)]
enum CodeTypeIn {
  AbstractRoot,     // 文本开头
  Unkown,           // 等待解析
  UnkownTag,        // 未知标签
  Tag,              // 标签开始
  TagEnd,           // 标签结束
  ExclamationBegin, // 叹号开头的标签类型、包含下面的注释、HTMLDoctype声明、XMLCDATA
  Comment,          // 注释
  HTMLDoctype,      // HTMLDoctype 声明内
  XMLCDATA,         // XML CDATA数据
  XMLDeclare,       // xml声明
  TextNode,         // 文本节点
}

// 处理带引号数据状态
#[derive(PartialEq)]
struct QuotedCodeInfo {
  in_translate: bool, // 是否处于反斜杠转义状态
  is_single: bool,    // 是否单引号、否则是双引号
}

pub fn is_tag_ok(chars: &Vec<char>, parser_type: &ParserType) -> bool {
  true
}
/**
 * 当前解析行列位置
*/
#[derive(Copy, Clone)]
pub struct CodePosAt {
  pub line_no: u32,
  pub col_no: u32,
}

impl CodePosAt {
  // 构造函数
  pub fn new(line_no: u32, col_no: u32) -> Self {
    CodePosAt { line_no, col_no }
  }
  // 创建一个开始位置
  pub fn begin() -> Self {
    CodePosAt::new(0, 0)
  }
  // 跳转新行
  pub fn set_new_line(&mut self) {
    self.line_no += 1;
    self.col_no = 0;
  }
  // 移动列位
  pub fn move_one(&mut self) {
    self.col_no += 1;
  }
}

/**
 * Attr 标签属性
 */
pub struct Attr {
  pub key: Option<String>,
  pub value: Option<String>,
  pub quote: Option<String>,
}

/**
 * Tag 标签节点信息
*/
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

#[derive(PartialEq)]
pub enum TagCodeIn {
  Wait,
  Key,
  Value,
  DoubleQuotedValue,
  SingleQuotedValue,
}

/**
 * Node节点
 */
pub struct Node<'a> {
  pub node_type: NodeType,                        // 节点类型
  pub start_at: CodePosAt,                        // 节点开始标签开始位置
  pub end_at: CodePosAt,                          // 节点结束标签结束位置
  pub end_tag: Option<Rc<RefCell<Node<'a>>>>,     // 结束标签的索引
  pub parent: Option<Rc<RefCell<Node<'a>>>>,      // 父节点、针对标签
  pub content: Option<Vec<char>>,                 // 内容部分、针对text文本节点
  pub childs: Option<Vec<Rc<RefCell<Node<'a>>>>>, // 子节点
  pub meta: Option<RefCell<TagMeta>>,
}

impl<'a> Node<'a> {
  // 创建一个节点
  pub fn new(node_type: NodeType, code_at: CodePosAt) -> Self {
    use NodeType::*;
    let (content, childs) = match node_type {
      AbstractRoot => (None, None),
      Tag => (
        None,                        // tag不需要内容数据
        Some(Vec::with_capacity(5)), // tag子节点
      ),
      _ => (Some(Vec::with_capacity(10)), None),
    };
    return Node {
      node_type,
      start_at: code_at,
      end_at: code_at,
      end_tag: None,
      parent: None,
      content,
      childs,
      meta: None,
    };
  }
}

/**
 * Doc 文档
 * 解析
*/
pub struct Doc<'a> {
  code_in: CodeTypeIn, // 当前解析标签位置
  position: CodePosAt, // 当前解析字符位置
  prev_chars: Vec<char>,
  prev_char: char,
  chain_nodes: Vec<Rc<RefCell<Node<'a>>>>,
  current_node: Rc<RefCell<Node<'a>>>, // 指向当前解析的节点
  pub parser_type: ParserType,
  pub nodes: Vec<Rc<RefCell<Node<'a>>>>,
}
impl<'a> Doc<'a> {
  // 创建实例
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
    }
  }

  // 读取一行
  pub fn read_line(&mut self, code: &str) {}
  // 读取一个字符，使用prev_chars来断定分类
  fn next(&mut self, c: char) -> Result<(), Box<dyn Error>> {
    let is_html = self.parser_type == ParserType::HTML;
    let is_xml = !is_html;
    // 引入CodeTypeIn里的所有枚举声明
    use CodeTypeIn::*;
    match self.code_in {
      TextNode | Unkown | AbstractRoot => {
        match c {
          // 匹配到标签开始标记符
          TAG_BEGIN_CHAR => {
            self.code_in = UnkownTag;
          }
          _ => {
            // 添加文本节点
            if self.code_in != TextNode {
              let node = Rc::new(RefCell::new(Node::new(NodeType::Text, self.position)));
              let current_node = Rc::clone(&node);
              self.current_node = current_node;
              self.nodes.push(node);
              self.code_in = TextNode;
            }
            // 将内容直接加入到文本节点
            self.current_node.borrow_mut().content.as_mut().map(|v| {
              v.push(c);
            });
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
        match &self.current_node.borrow_mut().meta {
          Some(meta) => {
            // meta有数据、表示标签名已经解析完成
            let mut meta = meta.borrow_mut();
            use TagCodeIn::*;
            match &meta.tag_in {
              Wait => {
                // 等待状态
                let is_self_close = self.prev_char == '/';
                if (self.prev_char == '?' || is_self_close) && !is_end {
                  panic!("错误的标签{}", self.prev_char);
                }
                if is_whitespace {
                  // 忽略空格
                } else if is_end {
                  meta.is_end = true;
                  // 自闭合标签
                  if is_self_close {
                    meta.is_closed = true;
                    meta.self_closed = true;
                    // 将标签从chain列表里移除
                    self.chain_nodes.pop();
                  }
                  self.code_in = Unkown;
                } else {
                  // 判断其它类型
                  match c {
                    '"' | '\'' => {
                      // 如果不是处在key-value位置
                      if !meta.is_in_kv {
                        if !self.prev_char.is_ascii_whitespace() {
                          panic!("属性值之间缺少空格");
                        }
                      } else {
                        // 属性往前移一位
                        meta.attr_index += 1;
                      }
                      // 重置prev状态
                      meta.prev_is_key = false;
                      // 重置prev_chars
                      self.prev_chars = vec![];
                      meta.tag_in = if c == '"' {
                        DoubleQuotedValue
                      } else {
                        SingleQuotedValue
                      };
                    }
                    '?' => {
                      if self.code_in != XMLDeclare {
                        panic!("错误的标签?");
                      }
                    }
                    '/' => {
                      if self.code_in != Tag {
                        panic!("错误的结束标签/");
                      }
                    }
                    '=' => {
                      if meta.prev_is_key {
                        meta.is_in_kv = true;
                      } else {
                        panic!("错误的=");
                      }
                    }
                    _ => {
                      // 判断是否正确的属性名
                      self.prev_chars = vec![c];
                      meta.tag_in = Key;
                      meta.attr_index += 1;
                    }
                  }
                }
              }
              Key | Value => {}
              DoubleQuotedValue | SingleQuotedValue => {
                let attr_index = meta.attr_index as usize;
                let mut cur_attr = meta.attrs.get_mut(attr_index).unwrap();
                cur_attr.value.unwrap().push_str(c.to_string());
              }
            }
          }
          None => {
            if is_whitespace || is_end {
              // 遇到空格或者结束标签表示标签已经结束
              let tag_name: String = self.prev_chars.iter().collect();
              if is_whitespace {
                if self.code_in == XMLDeclare && tag_name != "xml" {
                  panic!("错误的xml声明:{}", tag_name);
                } else if self.code_in == HTMLDoctype && tag_name != "DOCTYPE" {
                  panic!("错误的DOCTYPE声明:{}", tag_name);
                }
              } else {
                match self.code_in {
                  XMLDeclare => panic!(""),
                  HTMLDoctype => panic!(""),
                  Tag => {
                    self.code_in = Unkown;
                  }
                  _ => unreachable!("已经枚举所有情况"),
                }
              }
              // 检查标签名是否符合规范
              if !is_tag_ok(&self.prev_chars, &self.parser_type) {
                panic!("不符合规范的标签名称：{}", tag_name);
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
              self.current_node.borrow_mut().meta = Some(RefCell::new(meta));
            } else {
              // 将字符加入到prev_chars列表里
              self.prev_chars.push(c);
            }
          }
        }
      }
      TagEnd => {
        // 结束标签处理逻辑
      }
      Comment | XMLCDATA => {}
      UnkownTag => {
        // 标签还没有被确定类型
        let begin_at = CodePosAt {
          line_no: self.position.line_no,
          col_no: self.position.col_no - 1,
        };
        let mut new_node: Option<Rc<RefCell<Node<'a>>>> = None;
        match c {
          'a'..='z' | 'A'..='Z' => {
            // 标签开头、将标签直接加入
            let mut orig_node = Node::new(NodeType::Tag, begin_at);
            // 设置父级为当前chain_nodes的最后一个节点
            orig_node.parent = Some(Rc::clone(self.chain_nodes.last().unwrap()));
            let node = Rc::new(RefCell::new(orig_node));
            // 设置
            let ref_node = Rc::clone(&node);
            self.chain_nodes.push(ref_node);
            new_node = Some(node);
            self.code_in = Tag;
            self.prev_chars.push(c);
          }
          '/' => {
            // 结束标签
            let node = Rc::new(RefCell::new(Node::new(NodeType::XMLDeclare, begin_at)));
            new_node = Some(node);
            self.code_in = TagEnd;
          }
          '?' => {
            if is_xml {
              // 视作xml声明标签
              let node = Rc::new(RefCell::new(Node::new(NodeType::XMLDeclare, begin_at)));
              new_node = Some(node);
              self.code_in = XMLDeclare;
            } else {
              panic!("无法识别的html标签")
            }
          }
          '!' => {
            // Comment|DOCTYPE|XMLCDATA
            self.code_in = ExclamationBegin;
          }
          _ => {
            panic!("错误的标签标记");
          }
        };
        // 1、将current_node指向当前节点
        // 2、将节点加入nodes
        if let Some(node) = new_node {
          let current_node = Rc::clone(&node);
          self.current_node = current_node;
          self.nodes.push(node);
        }
      }
      ExclamationBegin => {}
    }
    // 判断是否需要换行等
    let mut need_move_col = true;
    // \r 在Macos系统早期版本里使用\r充当换行符
    if c == '\r' {
      self.position.set_new_line();
      need_move_col = false;
    } else if c == '\n' {
      // \n 判断是否前面还有\r，有的话合并\r<windows>
      if self.prev_char == '\r' {
        // 不做处理
      } else {
        self.position.set_new_line();
      }
      need_move_col = false;
    }
    // 移动region文档位置
    if need_move_col {
      self.position.move_one();
    }
    // 设置前一个字符串
    self.prev_char = c;
    Ok(())
  }
}
