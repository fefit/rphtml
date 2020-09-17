use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
pub mod parser;
use parser::{Doc, Node, ParserType};

type StaticNode = Node<'static>;
#[wasm_bindgen]
pub fn parse(content: &str, xml: bool) -> Rc<RefCell<StaticNode>> {
  let parser_type = if xml {
    ParserType::XML
  } else {
    ParserType::HTML
  };
  let mut doc = Doc::new(parser_type);
  let _ = doc.parse(content);
  doc.root
}
