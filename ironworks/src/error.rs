// TODO: non exhaustive?
// TODO: should we have, like, sqpack error, excel error, etc, and then a big daddy Error that combines them?
#[derive(thiserror::Error, Debug)]
pub enum Error {
	// TODO: how do we want to represent these properties of not found?
	#[error("THING VALUE could not be found.")]
	NotFound,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
