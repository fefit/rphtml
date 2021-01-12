use crate::parser::RenderEncoderOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ParseOptions {
	pub case_sensitive_tagname: bool, // whether the tagname is case-sensitive, default case-insenstive
	pub allow_self_closing: bool,     // allow self closing that not void elements
	pub allow_fix_unclose: bool,      // allow auto fix unclosed tag
	pub decode_entity: bool,          // decode the entity to an unicode character
}

#[derive(Default, Deserialize, Serialize)]
pub struct RenderOptions {
	pub minify_spaces: bool,
	pub lowercase_tagname: bool,
	pub remove_endtag_space: bool,
	pub remove_attr_quote: bool,
	pub remove_comment: bool,
	pub always_close_void: bool,
	pub inner_html: bool,
	#[serde(skip_serializing, skip_deserializing)]
	pub encoder: Option<RenderEncoderOption>,
}
