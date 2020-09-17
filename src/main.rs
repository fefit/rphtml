use chtml::parser::*;
fn main() {
	let mut doc = Doc::new(ParserType::HTML);
	let html = r###"
	<!DOCTYPE html>
	<html>
		<head>
			<style>
				/** color */
			</style>
		</head>
		<body>
			<!--这是一段注释--->
		</body>
	</html>
	"###;
	// doc.parse(html);
	doc.parse_file("./cases/full.html");
	/*let tag = r#"
			<div class="hello good" readonly></div>
		"#;
	doc.parse(tag);*/
}
