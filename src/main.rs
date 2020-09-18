use rxhtml::parser::*;

fn main() {
  let mut doc = Doc::new(ParserType::HTML);
  let _ = doc.parse(
    r###"
    <!DOCTYPE html>
    <html>
        <head>
            <title>测试页面</title>
        </head>
        <body>
            <div class="header">
                测试header
            </div>
        </body>
    </html>
  "###,
  );
  println!("doc.root:{:?}", doc.root);
}
