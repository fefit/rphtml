use chtml::parser::*;
fn main() {
	let mut doc = Doc::new(ParserType::HTML);
	/*let html = r#"
	<!DOCTYPE html>
	<html>
		<head>
			<meta charset="utf-8" />
			<meta name="keywords" />
		</head>
		<body class="body" readonly>
			<!---this is a comment--->
			<div class="hello">
				world!
			</div>
		</body>
	</html>
	"#;
	doc.parse(html);*/
	let tag = r#"
			<div class="hello good" readonly></div>
		"#;
	doc.parse(tag);
}
