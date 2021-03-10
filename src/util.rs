/**
 * non characters
 * https://infra.spec.whatwg.org/#noncharacter
*/
pub fn is_non_character(ch: &char) -> bool {
	matches!(
		ch,
		'\u{FDD0}'
			..='\u{FDEF}'
				| '\u{FFFE}'
				| '\u{FFFF}'
				| '\u{1FFFE}'
				| '\u{1FFFF}'
				| '\u{2FFFE}'
				| '\u{2FFFF}'
				| '\u{3FFFE}'
				| '\u{3FFFF}'
				| '\u{4FFFE}'
				| '\u{4FFFF}'
				| '\u{5FFFE}'
				| '\u{5FFFF}'
				| '\u{6FFFE}'
				| '\u{6FFFF}'
				| '\u{7FFFE}'
				| '\u{7FFFF}'
				| '\u{8FFFE}'
				| '\u{8FFFF}'
				| '\u{9FFFE}'
				| '\u{9FFFF}'
				| '\u{AFFFE}'
				| '\u{AFFFF}'
				| '\u{BFFFE}'
				| '\u{BFFFF}'
				| '\u{CFFFE}'
				| '\u{CFFFF}'
				| '\u{DFFFE}'
				| '\u{DFFFF}'
				| '\u{EFFFE}'
				| '\u{EFFFF}'
				| '\u{FFFFE}'
				| '\u{FFFFF}'
				| '\u{10FFFE}'
				| '\u{10FFFF}'
	)
}

/**
 *
 * https://www.w3.org/TR/2012/WD-html-markup-20120329/syntax.html#syntax-attributes
 * https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
*/
pub fn is_char_available_in_key(ch: &char) -> bool {
	if ch.is_ascii_alphanumeric() || ['_', '-', '.', ':'].contains(ch) {
		return true;
	}
	if ch.is_ascii_whitespace()
		|| ch.is_ascii_control()
		|| is_non_character(ch)
		|| ch.is_ascii_punctuation()
		|| *ch == '\u{0000}'
	{
		return false;
	}
	true
}

pub fn is_char_available_in_value(ch: &char) -> bool {
	if ch.is_ascii_whitespace() {
		return false;
	}
	!matches!(ch, '\u{0000}' | '"' | '\'' | '<' | '>' | '/' | '=' | '`')
}
