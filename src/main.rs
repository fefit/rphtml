use rphtml::config::{ParseOptions, RenderOptions};
use rphtml::parser::*;
use std::error::Error;
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
	let code = r##"
  <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><script xlink:href="cool-script.js" type="text/ecmascript" /></svg>
  "##;
	// let code = format!("<script>{}</script>", code);
	let doc = Doc::parse(
		code,
		ParseOptions {
			auto_remove_nostart_endtag: true,
			..Default::default()
		},
	)?;
	println!("{:?}", doc.borrow().root);
	// if let Some(childs) = &mut doc.get_root_node().borrow_mut().childs {
	// 	// Node::remove(&childs[0]);
	// }
	// println!("{:?}", result.root.borrow().id_tags);
	println!(
		"result:{:?}",
		doc.render(&RenderOptions {
			// inner_html: true,
			..Default::default()
		})
	);
	Ok(())
}
