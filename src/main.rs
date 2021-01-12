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
	let code = r##"<div>affg<div> tags are allowed here</div>def</div>"##;
	// let code = format!("<script>{}</script>", code);
	let doc = Doc::parse(
		code,
		ParseOptions {
			..Default::default()
		},
	)?;
	println!("{:?}", doc.nodes);
	// println!("{:?}", result.root.borrow().id_tags);
	println!(
		"result:{:?}",
		doc.render_text(&RenderOptions {
			// inner_html: true,
			..Default::default()
		})
	);
	Ok(())
}
