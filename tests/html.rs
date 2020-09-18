use rxhtml::parser::*;

#[test]
#[should_panic]
fn test_doctype() {
  let mut doc = Doc::new(ParserType::HTML);
  doc.parse("<!doctype>");
}
