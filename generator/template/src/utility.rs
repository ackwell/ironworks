use crate::error::PopulateError;

pub fn read_array<T>(
	offset: usize,
	count: usize,
	size: usize,
	mut f: impl FnMut(usize) -> Result<T, PopulateError>,
) -> Result<Vec<T>, PopulateError> {
	(0..count)
		.map(|index| f(offset + size * index))
		.collect::<Result<Vec<T>, PopulateError>>()
}
