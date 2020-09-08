use std::cell::RefCell;
use std::collections::LinkedList;
use std::error::Error;
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

pub enum NodeType {
  Comment,      // 注释节点
  HTMLDoctype,  // HTML的Doctype声明
  XMLDeclare,   // XML声明头
  XMLCDATA,     // XMLCDATA数据
  Tag,          // Tag
  Text,         // 文本节点
  AttrKey,      // 属性名
  AttrValue,    // 属性值
  Entity,       // 实体转义字符
  AbstractRoot, // 根节点、用于文档开头等
}
pub enum CodeTypeIn {
  BOF,             // 文本开头
  TagBegin,        // 标签开始
  Comment,         // 注释
  CDATA,           // XML CDATA数据
  DoubleQuoteAttr, // 双引号属性值
  SingleQuoteAttr, // 单引号属性值
  UnQuoteAttr,     // 无引号属性值
  TextNode,        // 文本节点
  EOF,
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
}

/**
 * Tag 标签节点信息
*/
pub struct Tag {
  pub name: String,
  pub namespace: String,
  pub fullname: String,
  pub attrs: Vec<Attr>,
}

/**
 * Node节点
 */
pub struct Node<'a> {
  pub node_type: NodeType,               // 节点类型
  pub start_begin_at: CodePosAt,         // 节点开始标签开始位置
  pub start_end_at: Option<CodePosAt>,   // 节点开始标签结束位置
  pub close_begin_at: Option<CodePosAt>, // 节点结束标签开始位置
  pub close_end_at: CodePosAt,           // 节点结束标签结束位置
  pub child: Vec<&'a Node<'a>>,          // 子节点、针对标签
  pub content: Vec<char>,                // 内容部分、针对text文本节点
}
/**
 * Doc 文档
 * 解析
*/
pub struct Doc<'a> {
  code_in: CodeTypeIn, // 当前解析标签位置
  position: CodePosAt, // 当前解析字符位置
  prev_chars: Vec<char>,
  prev_char: Option<char>,
  chain_nodes: Vec<RefCell<Node<'a>>>,
  pub parser_type: ParserType,
  pub nodes: Vec<Node<'a>>,
}
impl<'a> Doc<'a> {
  // 创建实例
  pub fn new(parser_type: ParserType) -> Self {
    let current_node = Node {
      node_type: NodeType::AbstractRoot,
      parent: LinkedList::new(),
      position: CodePosAt::begin(),
      content: Vec::new(),
    };
    let mut nodes = Vec::with_capacity(10);
    nodes.push(current_node);
    Doc {
      code_in: CodeTypeIn::BOF,
      position: CodePosAt::begin(),
      prev_char: None,
      prev_chars: Vec::with_capacity(30),
      parser_type,
      nodes,
    }
  }
  // 读取一行
  pub fn read_line(&mut self, code: &str) {}
  // 读取一个字符，使用prev_chars来断定分类
  fn next(&mut self, c: char) -> Result<&mut Self, Box<dyn Error>> {
    let Self {
      code_in,
      position,
      prev_chars,
      prev_char,
      nodes,
      parser_type,
      ..
    } = self;
    let isHTML = parser_type == &ParserType::HTML;
    // 引入CodeTypeIn里的所有枚举声明
    use CodeTypeIn::*;
    match code_in {
      BOF | TextNode => match c {
        // 匹配到标签开始标记符
        TAG_BEGIN_CHAR => self.code_in = TagBegin,
        // \r 在某些系统里使用\r充当换行符
        '\r' => {
          position.set_new_line();
        }
        // \n 判断是否前面还有\r，有的话合并\r<windows>
        '\n' => {
          if let Some('\r') = prev_char {
            // 不做处理
          } else {
            position.set_new_line();
          }
        }
        // 其它情况都视作文本节点
        _ => {
          self.code_in = TextNode;
          prev_chars.push(c);
        }
      },
      TagBegin => {
        // 遇到开始标签
        match c {
          // 如果是html定义的ascii空格
          space if space.is_ascii_whitespace() => {}
        }
      }
    }
    Ok(self)
  }
}
