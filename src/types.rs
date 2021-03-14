use crate::entity::EncodeType;
use std::error::Error;
// result
pub type GenResult<T> = Result<T, Box<dyn Error>>;
pub type HResult = GenResult<()>;
pub type RenderEncoderOption = Box<dyn Fn(char) -> Option<EncodeType>>;
