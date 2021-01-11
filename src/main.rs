use rphtml::config::{ParseOptions, RenderOptions};
use rphtml::parser::*;
use std::error::Error;
use std::rc::Rc;
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
  let code = r##"<div ><span>abc</span><span id="haha">def</span></div>"##;
  let result = Doc::parse(
    code,
    ParseOptions {
      decode_entity: true,
      ..Default::default()
    },
  )?;
  println!("{:?}", result.id_tags);
  // println!(
  //   "result:{:?}",
  //   Doc::render_js_tree(
  //     Rc::clone(&result.nodes[1]),
  //     &RenderOptions {
  //       inner_html: true,
  //       ..Default::default()
  //     }
  //   )
  // );
  Ok(())
}
