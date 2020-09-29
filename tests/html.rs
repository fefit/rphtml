use rphtml::config::{ParseOptions, RenderOptions};
use rphtml::parser::*;
use std::error::Error;

fn parse(code: &str) -> Result<Doc, Box<dyn Error>> {
  let doc = Doc::parse(code, Default::default())?;
  Ok(doc)
}
fn render(doc: &Doc) -> String {
  doc.render(&Default::default())
}

fn get_attr_content<'a>(v: &'a Option<AttrData>) -> Option<&'a str> {
  v.as_ref().map(|AttrData { content, .. }| content.as_str())
}

#[test]
fn test_doctype() -> HResult {
  // normal doctype
  let code = r##"<!doctype html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">"##;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  // html doctype, uppercase
  let code = r##"<!DOCTYPE html>"##;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  // wrong doctype name
  let code = r##"<!DOCTYPES html>"##;
  assert!(parse(code).is_err());
  // wrong doctype end slash
  let code = r##"<!DOCTYPE html/>"##;
  assert!(parse(code).is_err());
  Ok(())
}

#[test]
fn test_pre_tag() -> HResult {
  // test pre special
  let doc = parse("<pre>text in prev</pre><div></div>")?;
  let text_node = doc.nodes[2].borrow();
  let text_in_pre = if let Some(special) = text_node.special {
    match special {
      SpecialTag::Pre => true,
      _ => false,
    }
  } else {
    false
  };
  let div_not_in_pre = doc.nodes[5].borrow().special.is_none();
  assert!(text_in_pre);
  assert!(div_not_in_pre);
  // test pre render
  let code = r##"
    <pre> spaces </pre>
  "##;
  let doc = parse(code)?;
  let options = RenderOptions {
    minify_spaces: true,
    ..Default::default()
  };
  assert_eq!(doc.render(&options), r#"<pre> spaces </pre>"#);
  Ok(())
}

#[test]
fn test_title_tag() -> HResult {
  let code = "<title><div> tags are allowed here</div></title>";
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  Ok(())
}

#[test]
fn test_textarea_tag() -> HResult {
  let code = r#"<textarea name="content"><div> tags are allowed here</div></textarea>"#;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  Ok(())
}

#[test]
fn test_void_elements() {
  // void element allowed
  let code = r#"<br><br/><br >"#;
  assert!(parse(code).is_ok());
  // void element don't allowed
  let code = r#"<br></br>"#;
  assert!(parse(code).is_err());
}

#[test]
fn test_script_tag() -> HResult {
  let code = r#"
  <script type=text/javascript>
    var div=$("<div></div>");
    var script = "</script" + '>';
  </script>
  "#;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  Ok(())
}

#[test]
fn test_style_tag() -> HResult {
  let code = r#"
  <style type="text/css">
    body{
      color: lightblue;
    }
  </style>
  "#;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  Ok(())
}

#[test]
fn test_attrs() -> HResult {
  // a complex attribute
  let code = r##"
    <img src=http://site.com/abc.jpg alt =abc defer data-width= 60 data-name = "abc" data-size='60*60' data-msg="quote is '\"',ok" class="js-img img" />
  "##;
  let doc = parse(code)?;
  doc.root.borrow().childs.as_ref().map(|childs| {
    let _ = childs.iter().map(|c| {
      if let Some(meta) = &c.borrow().meta {
        let attrs = &meta.borrow().attrs;
        // attribute 1
        assert_eq!(get_attr_content(&attrs[0].key), Some("src"));
        assert_eq!(
          get_attr_content(&attrs[0].value),
          Some("http://site.com/abc.jpg")
        );
        // attribute 2
        assert_eq!(get_attr_content(&attrs[1].key), Some("alt"));
        assert_eq!(get_attr_content(&attrs[1].value), Some("abc"));
        // attribute 3
        assert_eq!(get_attr_content(&attrs[2].key), Some("defer"));
        assert!(&attrs[2].value.is_none());
        // attribute 4
        assert_eq!(get_attr_content(&attrs[3].key), Some("data-width"));
        assert_eq!(get_attr_content(&attrs[3].value), Some("60"));
        // attribute 5
        assert_eq!(get_attr_content(&attrs[4].key), Some("data-name"));
        assert_eq!(get_attr_content(&attrs[4].value), Some("abc"));
        assert_eq!(attrs[4].quote, Some('"'));
        assert_eq!(attrs[4].need_quote, false);
        // attribute 6
        assert_eq!(get_attr_content(&attrs[5].key), Some("data-size"));
        assert_eq!(get_attr_content(&attrs[5].value), Some("60*60"));
        assert_eq!(attrs[5].quote, Some('\''));
        // attribute 7
        assert_eq!(get_attr_content(&attrs[6].key), Some("data-msg"));
        assert_eq!(get_attr_content(&attrs[6].value), Some("quote is '\"',ok"));
        // attribute 8
        assert_eq!(get_attr_content(&attrs[7].key), Some("class"));
        assert_eq!(get_attr_content(&attrs[7].value), Some("js-img img"));
        assert_eq!(attrs[4].need_quote, true);
      }
    });
    childs
  });
  // wrong value
  // assert_eq!(parse(r#"<div id"1"></div>"#).is_err(), true);
  // assert_eq!(parse(r#"<div "1"'2'></div>"#).is_err(), true);
  Ok(())
}

#[test]
fn test_tag_close() -> HResult {
  // tag not closed
  let code = "<div>";
  assert_eq!(parse(code).is_err(), true);
  // allow code auto_fix
  let doc = Doc::parse(
    r#"<div id=1><div id=2><div id=3>3</div>"#,
    ParseOptions {
      allow_fix_unclose: true,
      ..Default::default()
    },
  );
  assert_eq!(doc.is_ok(), true);
  assert_eq!(
    render(&doc?),
    "<div id=1></div><div id=2></div><div id=3>3</div>"
  );
  // wrong tag end
  assert_eq!(parse("<div></p>").is_err(), true);
  Ok(())
}

#[test]
fn test_comment() -> HResult {
  let code = r##"
  <!---
  // this is a comment
  // --allowed
  --->
  "##;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  assert_eq!(
    doc.render(&RenderOptions {
      minify_spaces: true,
      remove_comment: true,
      ..Default::default()
    }),
    ""
  );
  Ok(())
}

#[test]
fn test_end_spaces() -> HResult {
  let code = "<div></div >";
  let doc = parse(code)?;
  assert_eq!(
    doc.render(&RenderOptions {
      remove_endtag_space: true,
      ..Default::default()
    }),
    "<div></div>"
  );
  Ok(())
}

#[test]
fn test_svg_tag() -> HResult {
  let code = r#"<svg version="1.1" baseProfile="full" width="300" height="200" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="red" /></svg>"#;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  Ok(())
}

#[test]
fn test_mathml_tag() -> HResult {
  let code = r#"<math><mrow></mrow></math>"#;
  let doc = parse(code)?;
  assert_eq!(render(&doc), code);
  Ok(())
}
