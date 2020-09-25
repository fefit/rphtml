use rphtml::config::{ParseOptions, RenderOptions};
use rphtml::parser::*;

fn main() {
  let parse_options: ParseOptions = Default::default();
  let mut doc = Doc::new();
  let result = doc.parse(r####"
  <!DOCTYPE html PUBLIC a = b "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"> <script> asgsgkdsj </scRipt >
  "####, parse_options);
  println!("result: {:?}", result);
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
