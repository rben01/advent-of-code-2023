use std::num::ParseIntError;

pub(crate) type AocResult<T> = Result<T, AocError>;

#[derive(Debug)]
pub(crate) enum AocError {
	FromIntError(ParseIntError),
	StrumParse(strum::ParseError),
	OptionWasNone(&'static str),
	ShapeError(ndarray::ShapeError),
	Other(String),
}

impl From<ParseIntError> for AocError {
	fn from(err: ParseIntError) -> Self {
		Self::FromIntError(err)
	}
}

impl From<strum::ParseError> for AocError {
	fn from(err: strum::ParseError) -> Self {
		Self::StrumParse(err)
	}
}

impl From<ndarray::ShapeError> for AocError {
	fn from(err: ndarray::ShapeError) -> Self {
		Self::ShapeError(err)
	}
}

impl From<String> for AocError {
	fn from(msg: String) -> Self {
		Self::Other(msg)
	}
}

pub trait ToResultDefaultErr<T> {
	/// Converts something to a "default" Result
	/// ## Ok
	/// Good values are preserved, turning into `Ok(x)`
	/// ## Errors
	/// Errors are turned into `Err(default error message)` where the error message is
	/// determined by the trait implementation (by the time you're at the call site, the
	/// error message is already set in stone)
	fn to_result(self) -> AocResult<T>;
}

impl<T> ToResultDefaultErr<T> for Option<T> {
	fn to_result(self) -> AocResult<T> {
		match self {
			Some(t) => Ok(t),
			None => Err(AocError::OptionWasNone(std::any::type_name::<T>())),
		}
	}
}
