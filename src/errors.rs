//use std::num::ParseIntError;

pub enum MiaError {
    ServiceAccountNotFound(String),
    UnknownError(String),
}

//impl From<ParseIntError> for MiaError {
//	fn from(err: ParseIntError) -> Self {
//		MiaError::UnknownError(err.to_string())
//	}
//}
