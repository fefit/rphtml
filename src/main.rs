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
  let mut doc = Doc::new();
  let result = doc.parse(
    r##"
  <!--this-->
  <!doctype html PUBLIC "http://www.w3c.com">
  <html>
    <head>
      <title><div> is a special tag</title>
      <meta charset="utf-8">
    </head>
    <body>
      <br><br>
      <h3 id="id" class=aaa>LOGO</h3>
      <textarea></textarea><br />
    </body>
  </html>
  "##,
    Default::default(),
  );
  println!("cur result is {:?}, {}", result?, doc.total_chars);
  println!("render ===> {:?}", doc.render(&Default::default()));
  Ok(())
  // let result = doc.parse(
  //   r###"<a> </a><svg>   <!--this is a comment--> <missing-glyph><path d="M0,0h200v200h-200z"/></missing-glyph><rect x="10" y="10" width="30" height="30" stroke="black" fill="transparent" stroke-width="5"/> </svg>"###,
  //   parse_options
  // );
  // let _ = result.map_err(|e| {
  //   println!("发生错误：{}", e);
  // });
  // for node in &doc.nodes {
  //   let node = node.borrow();
  //   println!(
  //     "node_type:{:?}, position:{:?}: content:{:?}",
  //     node.node_type, node.begin_at, node.content
  //   );
  // }
  // doc.into_json();
  // let err = doc.parse_file("./cases/full.html").expect("has not error");
  // println!("error is {:?}", err);
  // println!("now doc is {:?}", doc.nodes);
  // let options: RenderOptions = RenderOptions {
  //   minify_spaces: true,
  //   ..Default::default()
  // };
  // let output = doc.render(&options);
  // println!("output is {:?}", output);
  /*for node in &doc.nodes {
    let node = node.borrow();
    println!(
      "node_type:{:?}, position:{:?}: content:{:?}",
      node.node_type, node.begin_at, node.content
    );
  }
  let options: RenderOptions = RenderOptions {
    remove_attr_quote: true,
    minify_spaces: true,
    ..Default::default()
  };
  let output = doc.render(&options);
  println!("output is {}", output);
  let tree_output = Doc::render_js_tree(doc.root, &options);
  println!("tree is {}", tree_output);*/
}
