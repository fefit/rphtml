use rphtml::parser::*;

#[test]
fn test_doctype() {
  let mut doc = Doc::parse(r##"<!doctype html PUBLIC>"##, Default::default());
}

#[test]
fn test_valid_pre() -> HResult {
  let doc = Doc::parse("<pre>text in prev</pre><div></div>", Default::default())?;
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
  Ok(())
}

#[test]
fn test_title_tag() -> HResult {
  let info = "<title><div> tags are allowed here</div></title>";
  let doc = Doc::parse(info, Default::default())?;
  assert_eq!(doc.render(&Default::default()), info);
  Ok(())
}
