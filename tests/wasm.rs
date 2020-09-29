use rphtml::wasm::parse;
use wasm_bindgen_test::*;
#[wasm_bindgen_test]
fn test_parse() {
  assert_eq!(parse("<a></b>", Default::default()).is_err(), true);
}
