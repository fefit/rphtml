use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ParseOptions {
  pub case_sensitive_tagname: bool, // whether the tagname is case-sensitive, default case-insenstive
  pub allow_self_closing: bool,     // allow self closing that not void elements
  pub allow_fix_unclose: bool,      // allow auto fix unclosed tag
}

#[derive(Default, Deserialize, Serialize)]
pub struct RenderOptions {
  pub minify_spaces: bool,
  pub lowercase_tagname: bool,
  pub remove_endtag_space: bool,
  pub remove_attr_quote: bool,
  pub remove_comment: bool,
  pub always_close_void: bool,
}
