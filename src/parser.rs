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
  AbstractRoot,                    // 文本开头
  UnkownTag,                       // 未知标签
  Tag,                             // 标签开始
  TagEnd,                          // 标签结束
  ExclamationBegin,                // 叹号开头的标签类型、包含下面的注释、HTMLDoctype声明、XMLCDATA
  Comment,                         // 注释
  HTMLDoctype,                     // HTMLDoctype 声明内
  XMLCDATA,                        // XML CDATA数据
  XMLDeclare,                      // xml声明
  AttrKey,                         // 属性名
  AttrValue,                       // 属性值
  AttrQuotedValue(QuotedCodeInfo), // 属性值<带引号>
  TextNode,                        // 文本节点
}

// 处理带引号数据状态
#[derive(PartialEq)]
struct QuotedCodeInfo {
  in_translate: bool, // 是否处于反斜杠转义状态
  is_single: bool,    // 是否单引号、否则是双引号
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
pub struct TagMeta {
  pub name: String,
  pub namespace: String,
  pub fullname: String,
  pub attrs: Vec<Attr>,
  pub self_closed: bool,
  pub auto_fix: bool,
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
  pub meta: Option<TagMeta>,
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
  prev_char: Option<char>,
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
    let is_html = self.parser_type == ParserType::HTML;
    let is_xml = !is_html;
    let mut need_move_col = true;
    // 引入CodeTypeIn里的所有枚举声明
    use CodeTypeIn::*;
    match self.code_in {
      AbstractRoot | TextNode => {
        let is_in_root = self.code_in == AbstractRoot;
        match c {
          // 匹配到标签开始标记符
          TAG_BEGIN_CHAR => {
            self.code_in = UnkownTag;
          }
          _ => {
            // 添加文本节点
            if is_in_root {
              let node = Rc::new(RefCell::new(Node::new(NodeType::Text, self.position)));
              let current_node = Rc::clone(&node);
              self.current_node = current_node;
              self.nodes.push(node);
            }
            // \r 在某些系统里使用\r充当换行符
            if c == '\r' {
              self.position.set_new_line();
              need_move_col = false;
            } else if c == '\n' {
              // \n 判断是否前面还有\r，有的话合并\r<windows>
              if let Some('\r') = self.prev_char {
                // 不做处理
              } else {
                self.position.set_new_line();
              }
              need_move_col = false;
            } else {
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
        match &self.current_node.borrow().meta {
          Some(meta) => {
            // meta有数据、表示解析到
          }
          None => {}
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
      _ => {}
    }
    // 移动region文档位置
    if need_move_col {
      self.position.move_one();
    }
    // 设置前一个字符串
    self.prev_char = Some(c);
    Ok(())
  }
}
