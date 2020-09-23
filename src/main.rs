use rxhtml::config::{ParseOptions, RenderOptions};
use rxhtml::parser::*;

fn main() {
  let parse_options: ParseOptions = Default::default();
  let mut doc = Doc::new();
  let result = doc.parse(
    r###"<meta charset="utf-8"><div class="hallo js-remove" id="good">   hahahaa   </div><script>aaaa</script><pre> agsdgsdlj sgs</pre>"###, 
    parse_options
  );
  println!("result is {:?}", result);
  // doc.into_json();
  // let err = doc.parse_file("./cases/full.html").expect("has not error");
  // println!("error is {:?}", err);
  // println!("now doc is {:?}", doc.nodes);
  let options: RenderOptions = Default::default();
  let output = doc.render(&options);
  println!("output is {:?}", output);
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
