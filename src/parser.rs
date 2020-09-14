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
#[derive(PartialEq, Debug)]
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
  AbstractRoot,                                   // 文本开头
  Unkown,                                         // 等待解析
  UnkownTag(CodePosAt),                           // 未知标签
  Tag,                                            // 标签开始
  TagEnd,                                         // 标签结束
  ExclamationBegin(CodePosAt, Option<Vec<char>>), // 叹号开头的标签类型、包含下面的注释、HTMLDoctype声明、XMLCDATA
  Comment,                                        // 注释
  HTMLDoctype,                                    // HTMLDoctype 声明内
  XMLCDATA,                                       // XML CDATA数据
  XMLDeclare,                                     // xml声明
  TextNode,                                       // 文本节点
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
#[derive(Copy, Clone, Debug, PartialEq)]
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
#[derive(Debug)]
pub struct Attr {
  pub key: Option<String>,
  pub value: Option<String>,
  pub quote: Option<String>,
}

/**
 * Tag 标签节点信息
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
 * Node节点
 */
#[derive(Debug)]
pub struct Node<'a> {
  pub tag_index: u32,
  pub node_type: NodeType,              // 节点类型
  pub start_at: CodePosAt,              // 节点开始标签开始位置
  pub end_at: CodePosAt,                // 节点结束标签结束位置
  pub end_tag: Option<RefNode<'a>>,     // 结束标签的索引
  pub parent: Option<RefNode<'a>>,      // 父节点、针对标签
  pub content: Option<Vec<char>>,       // 内容部分、针对text文本节点
  pub childs: Option<Vec<RefNode<'a>>>, // 子节点
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
      tag_index: 0,
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
  chain_nodes: Vec<RefNode<'a>>,
  current_node: RefNode<'a>, // 指向当前解析的节点
  tag_index: u32,
  pub parser_type: ParserType,
  pub nodes: Vec<RefNode<'a>>,
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
      tag_index: 0,
    }
  }
  // 解析文档内容
  pub fn parse(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
    for c in content.chars() {
      self.next(c)?;
    }
    Ok(())
  }
  // 读取一个字符，使用prev_chars来断定分类
  fn next(&mut self, c: char) -> Result<(), Box<dyn Error>> {
    let is_html = self.parser_type == ParserType::HTML;
    let is_xml = !is_html;
    println!("解析到字母{}", String::from(c));
    // 引入CodeTypeIn里的所有枚举声明
    use CodeTypeIn::*;
    match self.code_in {
      TextNode | Unkown | AbstractRoot => {
        match c {
          // 匹配到标签开始标记符
          TAG_BEGIN_CHAR => {
            let mut current_node = self.current_node.borrow_mut();
            if self.code_in == TextNode {
              current_node.content = Some(self.prev_chars.clone());
              current_node.end_at = self.position;
              println!("当前text_node:{:?}", current_node);
              self.prev_chars.clear();
            }
            self.code_in = UnkownTag(self.position);
          }
          _ => {
            // 添加文本节点
            if self.code_in != TextNode {
              let node = Rc::new(RefCell::new(Node::new(NodeType::Text, self.position)));
              let current_node = Rc::clone(&node);
              self.current_node = current_node;
              self.nodes.push(node);
              self.code_in = TextNode;
              self.prev_chars.clear();
            }
            // 将内容直接加入到文本节点
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
            // meta有数据、表示标签名已经解析完成
            use TagCodeIn::*;
            match meta.tag_in {
              Wait | Key | Value => {
                println!("tag状态{:?}", meta.tag_in);
                let tag_in_wait = meta.tag_in == Wait;
                let mut is_end_key_or_value = false;
                // 等待状态
                if tag_in_wait && (self.prev_char == '?' || self.prev_char == '/') && !is_end {
                  panic!("错误的标签{}", self.prev_char);
                }
                if is_whitespace {
                  // wait状态下忽略空格
                  if !tag_in_wait {
                    is_end_key_or_value = true;
                  }
                } else if is_end {
                  meta.is_end = true;
                  // 自闭合标签
                  if tag_in_wait {
                    if self.prev_char == '/' {
                      meta.is_closed = true;
                      meta.self_closed = true;
                      // 将标签从chain列表里移除
                      self.chain_nodes.pop();
                    }
                  } else {
                    is_end_key_or_value = true;
                  }
                  self.code_in = Unkown;
                } else {
                  // 判断其它类型
                  match c {
                    '"' | '\'' if tag_in_wait => {
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
                      self.prev_chars.clear();
                      meta.tag_in = if c == '"' {
                        DoubleQuotedValue
                      } else {
                        SingleQuotedValue
                      };
                    }
                    '?' | '/' | '=' => {
                      if c == '?' && self.code_in != XMLDeclare {
                        panic!("错误的标签?");
                      } else if c == '/' && self.code_in != Tag {
                        panic!("错误的结束标签/");
                      } else if c == '=' {
                        if meta.prev_is_key {
                          meta.is_in_kv = true;
                        } else {
                          panic!("错误的=");
                        }
                      }
                      if !tag_in_wait {
                        // 结束key和value
                        meta.tag_in = Wait;
                        is_end_key_or_value = true;
                      }
                    }
                    _ => {
                      // 判断是否正确的属性名或者value值
                      if tag_in_wait {
                        self.prev_chars = vec![c];
                        if meta.is_in_kv {
                          meta.tag_in = Value;
                        } else {
                          meta.tag_in = Key;
                          // 属性值往前移动一位
                          meta.attr_index += 1;
                          meta.prev_is_key = true;
                        }
                      } else {
                        // key或者value的情况下直接加入到prev_chars列表里
                        self.prev_chars.push(c);
                      }
                    }
                  }
                }
                if is_end_key_or_value {
                  // key或者value状态
                  let attr_index = meta.attr_index as usize;
                  let is_key = meta.tag_in == Key;
                  if meta.attrs.len() <= attr_index {
                    let mut attr = Attr {
                      key: None,
                      value: None,
                      quote: None,
                    };
                    let empty_string = String::from("");
                    if is_key {
                      attr.key = Some(empty_string);
                    } else {
                      attr.value = Some(empty_string);
                    }
                    println!("增加vec属性");
                    meta.attrs.push(attr);
                  };
                  println!("attr_index:{}", attr_index);
                  let cur_attr = meta.attrs.get_mut(attr_index).unwrap();
                  let target = if is_key {
                    cur_attr.value.as_mut()
                  } else {
                    cur_attr.key.as_mut()
                  };
                  if let Some(v) = target {
                    *v = self.prev_chars.iter().collect::<String>();
                    self.prev_chars.clear();
                  }
                }
              }
              DoubleQuotedValue | SingleQuotedValue => {
                let is_in_translate = meta.is_in_translate;
                let attr_index = meta.attr_index as usize;
                if !is_in_translate {
                  meta.is_in_translate = true;
                  self.prev_chars.push(c);
                } else {
                  if (meta.tag_in == DoubleQuotedValue && c == '"')
                    || (meta.tag_in == SingleQuotedValue && c == '\'')
                  {
                    meta.tag_in = Wait;
                    let cur_attr = meta.attrs.get_mut(attr_index).unwrap();
                    if let Some(v) = cur_attr.value.as_mut() {
                      *v = self.prev_chars.iter().collect::<String>();
                      self.prev_chars.clear();
                    }
                  } else {
                    self.prev_chars.push(c);
                  }
                }
              }
            }
          }
          None => {
            println!("===========>meta 尚未初始化{}，{}", is_whitespace, is_end);
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
              current_node.meta = Some(RefCell::new(meta));
            } else {
              // 将字符加入到prev_chars列表里
              self.prev_chars.push(c);
            }
          }
        }
      }
      TagEnd => {
        // 结束标签处理逻辑
        if c == TAG_END_CHAR {
          let end_tag_name: String = self.prev_chars.iter().collect();
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
                // todo设置结束标签
                break;
              } else if is_allow_fix {
                // 如果没有找到匹配且允许自动修复
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
            // 找到最近的匹配标签，没有匹配的标签视作自闭合标签
            if empty_closed_tags.len() > 0 {
              if let Some(parent_node) = &real_parent_node {
                for tag_node in empty_closed_tags.iter_mut() {
                  // 修改tag_node的父级指向
                  tag_node.borrow_mut().parent = Some(Rc::clone(parent_node));
                  // 修改所有子级的父级指向、并且清空子级
                  if let Some(childs) = &tag_node.borrow_mut().childs {
                    if childs.len() > 0 {
                      for child_node in childs.iter() {
                        child_node.borrow_mut().parent = Some(Rc::clone(parent_node));
                      }
                    }
                  }
                  tag_node.borrow_mut().childs = None;
                  // 修改当前node的meta信息
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
        // Comment注释或者XMLCDATA数据
        let mut is_end = false;
        let is_in_comment = self.code_in == Comment;
        let stand_char = if is_in_comment { ']' } else { '-' };
        if c == '>' && self.prev_char == stand_char && self.prev_chars.len() >= 2 {
          let total_len = self.prev_chars.len();
          let prev_last_char = self.prev_chars[total_len - 2];
          if prev_last_char == stand_char {
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
        // 标签还没有被确定类型
        let mut new_node: Option<RefNode<'a>> = None;
        match c {
          'a'..='z' | 'A'..='Z' => {
            // 标签开头、将标签直接加入
            let mut inner_node = Node::new(NodeType::Tag, begin_at);
            inner_node.tag_index = self.tag_index + 1;
            // 设置父级为当前chain_nodes的最后一个节点
            let parent_node = self.chain_nodes.last().unwrap();
            inner_node.parent = Some(Rc::clone(parent_node));
            let node = Rc::new(RefCell::new(inner_node));
            // 设置
            let ref_node = Rc::clone(&node);
            self.chain_nodes.push(ref_node);
            new_node = Some(node);
            self.code_in = Tag;
            self.tag_index += 1;
            self.prev_chars = vec![c];
          }
          '/' => {
            // 结束标签
            let node = Rc::new(RefCell::new(Node::new(NodeType::TagEnd, begin_at)));
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
            self.code_in = ExclamationBegin(begin_at, None);
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
      ExclamationBegin(begin_at, ref char_queue) => {
        // 解析Comment/CDATA/DOCTYPE
        if let Some(next_chars) = char_queue {
          let total_len = self.prev_chars.len();
          let actual_len = next_chars.len();
          if total_len < actual_len {
            let cur_should_be = next_chars.get(total_len).unwrap();
            if *cur_should_be == c {
              if total_len == actual_len - 1 {
                let mut new_node: Option<RefNode<'a>> = None;
                match c {
                  '-' => {
                    self.code_in = Comment;
                    new_node = Some(Rc::new(RefCell::new(Node::new(
                      NodeType::Comment,
                      begin_at,
                    ))));
                  }
                  'E' => {
                    self.code_in = HTMLDoctype;
                    new_node = Some(Rc::new(RefCell::new(Node::new(
                      NodeType::HTMLDoctype,
                      begin_at,
                    ))));
                  }
                  'A' => {
                    self.code_in = XMLCDATA;
                    new_node = Some(Rc::new(RefCell::new(Node::new(
                      NodeType::XMLCDATA,
                      begin_at,
                    ))));
                  }
                  _ => unreachable!(),
                };
                // 1、将current_node指向当前节点
                // 2、将节点加入nodes
                if let Some(node) = new_node {
                  let current_node = Rc::clone(&node);
                  println!("发现新节点：{:?}", current_node);
                  self.current_node = current_node;
                  self.nodes.push(node);
                }
              }
            } else {
              panic!(
                "错误的标签{:?}，是否意思<!{}",
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
                ExclamationBegin(begin_at, Some(vec!['[', '[', 'C', 'D', 'A', 'T', 'A']));
            }
            _ => {
              panic!("无法识别的标签{}", c);
            }
          };
        }
        self.prev_chars.push(c);
      }
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
// 测试代码
mod test {
  use super::*;
  #[test]
  fn test_doc() {
    let mut doc = Doc::new(ParserType::HTML);
    let html = r#"
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="keywords" />
      </head>
      <body>
        <!---this is a comment--->
        <div class="hello">
          world!
        </div>
      </body>
    </html>
    "#;
    doc.parse(html);
  }
}
