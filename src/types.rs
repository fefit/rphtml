use crate::entity::EncodeType;
use std::error::Error;
// result
pub type BoxDynError = Box<dyn Error + Send + Sync + 'static>;
pub type GenResult<T> = Result<T, BoxDynError>;
pub type HResult = GenResult<()>;
pub type RenderEncoderOption = Box<dyn Fn(char) -> Option<EncodeType>>;
