// TODO: If this is a notable bottleneck, look into optimisations e.g. suffix trees.
// Thanks, wikipedia.
pub fn longest_common_subsequence(a: &str, b: &str) -> String {
	// Initialise table
	let chars_a = a.chars().collect::<Vec<_>>();
	let chars_b = b.chars().collect::<Vec<_>>();

	let mut table = vec![vec![0u32; b.chars().count() + 1]; a.chars().count() + 1];

	// Build LCS table
	for i in 1..=chars_a.len() {
		for j in 1..=chars_b.len() {
			table[i][j] = if chars_a[i - 1] == chars_b[j - 1] {
				table[i - 1][j - 1] + 1
			} else if table[i - 1][j] > table[i][j - 1] {
				table[i - 1][j]
			} else {
				table[i][j - 1]
			}
		}
	}

	// Backtrack through the table to build a string
	let mut chars = Vec::<char>::new();
	let mut i = chars_a.len();
	let mut j = chars_b.len();
	while i > 0 && j > 0 {
		if chars_a[i - 1] == chars_b[j - 1] {
			chars.push(chars_a[i - 1]);
			i -= 1;
			j -= 1;
		} else if table[i - 1][j] > table[i][j - 1] {
			i -= 1;
		} else {
			j -= 1;
		}
	}

	chars.iter().rev().collect()
}

#[cfg(test)]
mod test {
	use super::longest_common_subsequence as lcs;

	#[test]
	fn equal() {
		assert_eq!(lcs("abcde", "abcde"), "abcde".to_string());
	}

	#[test]
	fn partial() {
		assert_eq!(lcs("abcde", "zbcdz"), "bcd".to_string());
		assert_eq!(lcs("zbcdz", "abcde"), "bcd".to_string());
	}

	#[test]
	fn distributed() {
		assert_eq!(lcs("abcde", "zazbzczdzez"), "abcde".to_string());
		assert_eq!(lcs("zazbzczdzez", "abcde"), "abcde".to_string());
	}

	#[test]
	fn none() {
		assert_eq!(lcs("abcde", "vwxyz"), "".to_string());
	}

	#[test]
	fn start() {
		assert_eq!(lcs("abc", "abcde"), "abc".to_string());
		assert_eq!(lcs("abcde", "abc"), "abc".to_string());
	}

	#[test]
	fn centre() {
		assert_eq!(lcs("bcd", "abcde"), "bcd".to_string());
		assert_eq!(lcs("abcde", "bcd"), "bcd".to_string());
	}

	#[test]
	fn end() {
		assert_eq!(lcs("cde", "abcde"), "cde".to_string());
		assert_eq!(lcs("abcde", "cde"), "cde".to_string());
	}
}
