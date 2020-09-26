use rphtml::parser::*;

#[test]
fn test_doctype() {
  let mut doc = Doc::new();
  doc.parse(r##"<!doctype html PUBLIC>"##, Default::default());
}

#[test]
fn test_valid_pre() {
  let mut doc = Doc::new();
  let _ = doc.parse("<pre>text in prev</pre><div></div>", Default::default());
  assert_eq!(doc.nodes.len(), 6);
  assert!(doc.nodes[2].borrow().special.is_some());
  let text_in_pre = if let Some(special) = doc.nodes[2].borrow().special {
    match special {
      SpecialTag::Pre => true,
      _ => false,
    }
  } else {
    false
  };
  let div_not_in_pre = doc.nodes[4].borrow().special.is_none();
  assert!(text_in_pre);
  assert!(div_not_in_pre);
}
