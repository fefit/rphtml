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

pub enum NodeType {
  Comment,      // 注释节点
  HTMLDoctype,  // HTML的Doctype声明
  XMLDeclare,   // XML声明头
  XMLCDATA,     // XMLCDATA数据
  Tag,          // Tag
  Text,         // 文本节点
  AbstractRoot, // 虚拟根节点，用于文档开头等
}
enum CodeTypeIn {
  AbstractRoot,                    // 文本开头
  Tag(TagCodeInfo),                // 标签开始
  Comment,                         // 注释
  XMLCDATA,                        // XML CDATA数据
  XMLDeclare,                      // xml声明
  AttrKey,                         // 属性名
  AttrValue,                       // 属性值
  AttrQuotedValue(QuotedCodeInfo), // 属性值<带引号>
  TextNode,                        // 文本节点
}

// 处理带引号数据状态
struct QuotedCodeInfo {
  in_translate: bool, // 是否处于反斜杠转义状态
  is_single: bool,    // 是否单引号、否则是双引号
}

struct TagCodeInfo {
  is_tag: bool,
  is_close: bool,
}

// 处理tag信息

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
  pub parent: Option<&'a Node<'a>>,      // 父节点、针对标签
  pub content: Option<Vec<char>>,        // 内容部分、针对text文本节点
  pub childs: Option<Vec<&'a Node<'a>>>, // 子节点
}

impl<'a> Node<'a> {
  // 创建一个节点
  pub fn new(node_type: NodeType, codeAt: CodePosAt) -> Self {
    use NodeType::*;
    let (start_end_at, close_begin_at, close_end_at, content, childs) = match node_type {
      AbstractRoot => (None, None, codeAt, None, None),
      Tag => (
        Some(codeAt),
        Some(codeAt),
        codeAt,
        None,                        // tag不需要内容数据
        Some(Vec::with_capacity(5)), // tag子节点
      ),
      _ => (
        Some(codeAt),
        Some(codeAt),
        codeAt,
        Some(Vec::with_capacity(10)),
        None,
      ),
    };
    return Node {
      node_type,
      start_begin_at: codeAt,
      start_end_at,
      close_begin_at,
      close_end_at,
      parent: None,
      content,
      childs,
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
  prev_char: Option<char>,
  chain_nodes: Vec<Rc<Node<'a>>>,
  current_node: Rc<Node<'a>>,
  pub parser_type: ParserType,
  pub nodes: Vec<Rc<Node<'a>>>,
}
impl<'a> Doc<'a> {
  // 创建实例
  pub fn new(parser_type: ParserType) -> Self {
    let node = Rc::new(Node::new(NodeType::AbstractRoot, CodePosAt::begin()));
    let ref_node = Rc::clone(&node);
    let current_node = Rc::clone(&node);
    let mut nodes = Vec::with_capacity(10);
    let mut chain_nodes = Vec::with_capacity(10);
    nodes.push(node);
    chain_nodes.push(ref_node);
    Doc {
      code_in: CodeTypeIn::AbstractRoot,
      position: CodePosAt::begin(),
      prev_char: None,
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
    let Self {
      code_in,
      position,
      prev_chars,
      prev_char,
      nodes,
      parser_type,
      current_node,
      ..
    } = self;
    let is_html = parser_type == &ParserType::HTML;
    let is_xml = !is_html;
    let mut need_move_col = true;
    // 引入CodeTypeIn里的所有枚举声明
    use CodeTypeIn::*;
    match code_in {
      AbstractRoot | TextNode => match c {
        // 匹配到标签开始标记符
        TAG_BEGIN_CHAR => {
          self.code_in = Tag(TagCodeInfo {
            is_close: false,
            is_tag: false,
          });
        }
        // \r 在某些系统里使用\r充当换行符
        '\r' => {
          position.set_new_line();
          need_move_col = false;
        }
        // \n 判断是否前面还有\r，有的话合并\r<windows>
        '\n' => {
          if let Some('\r') = prev_char {
            // 不做处理
          } else {
            position.set_new_line();
          }
          need_move_col = false;
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
    if (need_move_col) {
      position.move_one();
    }
    Ok(())
  }
}
