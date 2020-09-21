pub struct ParseOptions {}
#[derive(Default)]
pub struct RenderOptions {
  pub minify_spaces: bool,
  pub lowercase_tagname: bool,
  pub remove_endtag_space: bool,
  pub remove_attr_quote: bool,
  pub remove_comment: bool,
}
