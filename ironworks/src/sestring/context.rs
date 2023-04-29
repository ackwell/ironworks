#[derive(Debug, Default)]
pub struct Context {
	// Parameters
	integers: Vec<u32>,
}

impl Context {
	pub fn integer(&self, index: u32) -> u32 {
		let raw_index = usize::try_from(index).unwrap() - 1;
		// TODO: I'm falling back to 0 when a param isn't available, but I'm not convinced that's the correct approach - realistically this is modelling a string system where arguments are in sync with the string requirements, and a desync would be a failure of some kind. Right? Maybe I should make the fallback u32::MAX and treat that as a "special" value across the baord.
		self.integers.get(raw_index).cloned().unwrap_or(0)
	}
}
