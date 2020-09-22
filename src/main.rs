use rxhtml::config::RenderOptions;
use rxhtml::parser::*;

fn main() {
  let mut doc = Doc::new();
  let _ = doc.parse(
    r###"<div class="hallo js-remove" id="good">   hahahaa   </div><script>aaaa</script>"###,
  );
  doc.into_json();
  println!("doc is {:?}", doc.root);
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
