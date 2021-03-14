use crate::position::{CodeAt, CodeRegion};
use crate::types::HResult;
use std::error::Error;
use std::fmt;
use thiserror::Error;
#[derive(Debug)]
pub struct ParseError {
	pub region: CodeRegion,
	pub kind: ErrorKind,
}

impl ParseError {
	pub fn new(kind: ErrorKind, region: CodeRegion) -> Box<Self> {
		Box::new(ParseError { region, kind })
	}
}

// display parse error
impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let output = format!("{}{}", self.region, self.kind);
		f.write_str(output.as_str())
	}
}

// impl trait Error
impl Error for ParseError {}

#[derive(Error, Debug)]
pub enum ErrorKind {
	#[error("wrong tag <{0}")]
	WrongTag(String),
	#[error("wrong end tag </{0}")]
	WrongEndTag(String),
	#[error("wrong child tag '<{1}' in tag '{}'")]
	ChildInSpecialTag(String, char),
	#[error("unmatched tag '</{0}>'")]
	UnmatchedClosedTag(String),
	#[error("unexpected character '{0}'")]
	UnexpectedCharacter(char),
	#[error("the tag '{0}' is not closed")]
	UnclosedTag(String),
	#[error("the tag's attribute should split by spaces,wrong character '{0}'")]
	NoSpaceBetweenAttr(char),
	#[error("unexpected character '{0}' in html doctype decalaration")]
	WrongHtmlDoctype(char),
	#[error("unrecognized tag '{0}', do you mean '{1}'")]
	UnrecognizedTag(String, String),
	#[error("wrong tag name '{0}'")]
	WrongTagIdentity(String),
	#[error("not allowed text '{0}' in root node")]
	WrongRootTextNode(String),
	#[error("because the config of 'case-sensitive', the tag '{0}' is not matched correctly.")]
	WrongCaseSensitive(String),
	#[error("wrong self-closing tag '{0}',make sure you set the config allow self-closing.")]
	WrongSelfClosing(String),
	#[error("{0}")]
	CommonError(String),
}
