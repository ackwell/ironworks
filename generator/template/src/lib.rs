// Disable lints that trip on harmless generated code
#![allow(
	clippy::identity_op,
	clippy::needless_question_mark,
	non_camel_case_types,
	unused_variables
)]

mod error;
mod utility;

pub mod sheet;

pub use error::PopulateError;
