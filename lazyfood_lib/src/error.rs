#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Represents an empty source. For example, an empty text file being given
	/// as input to `count_words()`.
	#[error("Source contains no data")]
	EmptySource,

	/// Represents a failure to read from input.
	#[error("Read error")]
	ReadError { source: std::io::Error },

	/// Represents all other cases of `std::io::Error`.
	#[error(transparent)]
	IOError(#[from] std::io::Error),
}
