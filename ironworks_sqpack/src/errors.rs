use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqPackError {
	#[error("invalid sqpack path \"{0}\"")]
	InvalidPath(String),

	#[error("unknown repository \"{repository}\" in sqpack path \"{path}\"")]
	UnknownRepository { path: String, repository: String },

	#[error("unknown category \"{category}\" in sqpack path \"{path}\"")]
	UnknownCategory { path: String, category: String },
}
