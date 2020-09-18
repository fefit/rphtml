use rxhtml::{parse, parse_file};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
#[should_panic]
fn test_parse() {
  parse("<a></b>", false);
}
