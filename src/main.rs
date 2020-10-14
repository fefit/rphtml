use rphtml::config::{ParseOptions, RenderOptions};
use rphtml::parser::*;
use std::error::Error;
use std::{env, fs};
fn main() -> Result<(), Box<dyn Error>> {
  // let current_dir = env::current_dir()?;
  // let source_dir = current_dir.join("cases").canonicalize()?;
  // for entry in fs::read_dir(source_dir)? {
  //   let entry = entry?;
  //   let filename = entry.path();

  //   let metadata = fs::metadata(&filename)?;

  //   if metadata.is_file() {
  //     let parse_options: ParseOptions = Default::default();
  //     let mut doc = Doc::new();
  //     let result = doc.parse_file(&filename, parse_options);
  //     match result {
  //       Ok(_) => {
  //         println!("compile ok");
  //       }
  //       Err(e) => {
  //         println!("{:?}: {:?}", filename, e);
  //         return Err(e);
  //       }
  //     };
  //   }
  // }
  let code = r#"<div class="read">   hahaha<span>haaa</span>aaaaaa  </div>"#;
  let doc = Doc::parse(
    code,
    ParseOptions {
      ..Default::default()
    },
  )?;
  let render_code = doc.render(&RenderOptions {
    minify_spaces: true,
    remove_comment: true,
    inner_html: true,
    ..Default::default()
  });
  println!("cur result is {:?}, {}", doc.root, doc.total_chars);
  println!("{:?}", code);
  println!("{:?}", render_code);
  println!("{:?}", render_code == code);
  Ok(())
}
