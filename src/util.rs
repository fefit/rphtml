pub fn is_identity(chars: &Vec<char>) -> bool {
  let mut is_first = true;
  let mut has_ns = false;
  for &c in chars {
    if is_first {
      if !(c.is_ascii_alphanumeric() || c == '_') {
        return false;
      }
      is_first = false;
      continue;
    }
    if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
      continue;
    }
    if !has_ns && (c == '.' || c == ':') {
      has_ns = true;
      is_first = true;
      continue;
    }
    return false;
  }
  if is_first {
    false
  } else {
    true
  }
}

/**
 *
 * https://www.w3.org/TR/2012/WD-html-markup-20120329/syntax.html#syntax-attributes
 * https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
*/
pub fn is_key_available_char(ch: &char) -> bool {
  if ch.is_ascii_whitespace() || ch.is_ascii_control() || is_non_character(ch) {
    return false;
  }
  match ch {
    '\u{0000}' => false,
    '"' | '\'' | '>' | '/' | '=' => false,
    _ => true,
  }
}
/**
 * non characters
 * https://infra.spec.whatwg.org/#noncharacter
*/
pub fn is_non_character(ch: &char) -> bool {
  match ch {
    '\u{FDD0}'..='\u{FDEF}' => true,
    '\u{FFFE}' | '\u{FFFF}' | '\u{1FFFE}' | '\u{1FFFF}' | '\u{2FFFE}' | '\u{2FFFF}'
    | '\u{3FFFE}' | '\u{3FFFF}' | '\u{4FFFE}' | '\u{4FFFF}' | '\u{5FFFE}' | '\u{5FFFF}'
    | '\u{6FFFE}' | '\u{6FFFF}' | '\u{7FFFE}' | '\u{7FFFF}' | '\u{8FFFE}' | '\u{8FFFF}'
    | '\u{9FFFE}' | '\u{9FFFF}' | '\u{AFFFE}' | '\u{AFFFF}' | '\u{BFFFE}' | '\u{BFFFF}'
    | '\u{CFFFE}' | '\u{CFFFF}' | '\u{DFFFE}' | '\u{DFFFF}' | '\u{EFFFE}' | '\u{EFFFF}'
    | '\u{FFFFE}' | '\u{FFFFF}' | '\u{10FFFE}' | '\u{10FFFF}' => true,
    _ => false,
  }
}
pub fn is_attr_key_ok(chars: &Vec<char>) -> bool {
  for ch in chars {
    if is_key_available_char(ch) {
      continue;
    }
    return false;
  }
  return true;
}

pub fn is_value_available_char(ch: &char) -> bool {
  if ch.is_ascii_whitespace() {
    return false;
  }
  match ch {
    '\u{0000}' => false,
    '"' | '\'' | '<' | '>' | '/' | '=' | '`' => false,
    _ => true,
  }
}

pub fn is_attr_value_ok(chars: &Vec<char>) -> bool {
  for ch in chars {
    if is_value_available_char(ch) {
      continue;
    }
    return false;
  }
  return true;
}
