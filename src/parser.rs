use std::collections::LinkedList;
use std::error::Error;

const TAG_BEGIN_CHAR: char = '<';
const TAG_END_CHAR: char = '>';
/**
 * ParserType 解析器类型
 * HTML: for html
 * XML: for xml
 */
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
  Entity,       //实体转义字符
  AbstractRoot, // 根节点、用于文档开头等
}
pub enum CodeTypeIn {
  BOF,
  TagBegin,
  TagEnd,
  StartTagBegin,
  StartTagEnd,
  EndTagBegin,
  EndTagEnd,
  CommentTagBegin,
  CommentBegin,
  DoubleQuoteAttr,
  SingleQuoteAttr,
  TextNode,
  TextEmpty,
  TextEntity,
  EOF,
}

/**
 * 当前解析行列位置
*/
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
 * Node节点
 */
pub struct Node {
  pub node_type: NodeType,
  pub parent: LinkedList<Node>,
  pub position: CodePosAt,
  pub content: Vec<char>,
}
/**
 * Doc 文档
 * 解析
*/
pub struct Doc {
  code_in: CodeTypeIn,
  position: CodePosAt,
  prev_chars: Vec<char>,
  prev_char: Option<char>,
  nodes: Vec<Node>,
}
impl Doc {
  // 创建实例
  pub fn new() -> Self {
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
      prev_chars: Vec::with_capacity(30),
      nodes,
      prev_char: None,
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
    } = self;
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
      }
    }
    Ok(self)
  }
}
