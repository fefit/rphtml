#![cfg(target_arch = "wasm32")]
use crate::config::{ParseOptions, RenderOptions};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

fn optional_bool(value: Option<bool>) -> bool {
  value.map_or(false, |flag| flag)
}

#[wasm_bindgen(typescript_custom_section)]
const IJS_PARSE_OPTIONS: &'static str = r#"
export interface IJsParseOptions {
  allow_fix_unclose?: boolean;
  allow_self_closing?: boolean;
  case_sensitive_tagname?: boolean;
  decode_entity?: boolean;
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const IJS_RENDER_OPTIONS: &'static str = r#"
export interface IJsRenderOptions {
  always_close_void?: boolean;
  lowercase_tagname?: boolean;
  minify_spaces?: boolean;
  remove_attr_quote?: boolean;
  remove_comment?: boolean;
  remove_endtag_space?: boolean;
  inner_html?: boolean;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJsParseOptions")]
  pub type IJsParseOptions;
  #[wasm_bindgen(typescript_type = "IJsRenderOptions")]
  pub type IJsRenderOptions;
}
#[derive(Default, Deserialize, Serialize)]
pub struct JsParseOptions {
  pub case_sensitive_tagname: Option<bool>,
  pub allow_self_closing: Option<bool>,
  pub allow_fix_unclose: Option<bool>,
  pub decode_entity: Option<bool>,
}

impl From<IJsParseOptions> for JsParseOptions {
  fn from(options: IJsParseOptions) -> Self {
    match JsValue::into_serde(&options) {
      Ok(result) => result,
      Err(_) => Default::default(),
    }
  }
}

impl From<JsParseOptions> for ParseOptions {
  fn from(options: JsParseOptions) -> Self {
    ParseOptions {
      case_sensitive_tagname: optional_bool(options.case_sensitive_tagname),
      allow_self_closing: optional_bool(options.allow_self_closing),
      allow_fix_unclose: optional_bool(options.allow_fix_unclose),
      decode_entity: optional_bool(options.decode_entity),
    }
  }
}
#[derive(Default, Deserialize, Serialize)]
pub struct JsRenderOptions {
  pub minify_spaces: Option<bool>,
  pub lowercase_tagname: Option<bool>,
  pub remove_endtag_space: Option<bool>,
  pub remove_attr_quote: Option<bool>,
  pub remove_comment: Option<bool>,
  pub always_close_void: Option<bool>,
  pub inner_html: Option<bool>,
}

impl From<IJsRenderOptions> for JsRenderOptions {
  fn from(options: IJsRenderOptions) -> Self {
    match JsValue::into_serde(&options) {
      Ok(result) => result,
      Err(_) => Default::default(),
    }
  }
}

impl From<JsRenderOptions> for RenderOptions {
  fn from(options: JsRenderOptions) -> Self {
    RenderOptions {
      minify_spaces: optional_bool(options.minify_spaces),
      lowercase_tagname: optional_bool(options.lowercase_tagname),
      remove_endtag_space: optional_bool(options.remove_endtag_space),
      remove_attr_quote: optional_bool(options.remove_attr_quote),
      remove_comment: optional_bool(options.remove_comment),
      always_close_void: optional_bool(options.always_close_void),
      inner_html: optional_bool(options.inner_html),
    }
  }
}
