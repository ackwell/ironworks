// Stolen and ported to rust from https://github.com/NotAdam/Lumina/blob/8abfa032fc0f47e79a58f7539dca2b23c96c66c8/src/Lumina/Misc/Crc32.cs

const POLY: u32 = 0xEDB88320;

const TABLE_SIZE: usize = 16 * 256;
type Table = [u32; TABLE_SIZE];
const TABLE: Table = build_table();

const fn build_table() -> Table {
	let mut table: Table = [0; TABLE_SIZE];

	let mut i = 0;

	while i < 256 {
		let mut res = i as u32;

		let mut t = 0;
		while t < 16 {
			let mut k = 0;
			while k < 8 {
				res = if (res & 1) == 1 {
					POLY ^ (res >> 1)
				} else {
					res >> 1
				};

				k += 1;
			}

			table[((t * 256) + i)] = res;

			t += 1;
		}

		i += 1;
	}

	table
}

pub fn crc32(bytes: &[u8]) -> u32 {
	let mut working = u32::MAX;

	let mut start = 0;
	let mut end = bytes.len();

	while end >= 16 {
		let a = TABLE[(3 * 256) + bytes[start + 12] as usize]
			^ TABLE[(2 * 256) + bytes[start + 13] as usize]
			^ TABLE[256 + bytes[start + 14] as usize]
			^ TABLE[bytes[start + 15] as usize];

		let b = TABLE[(7 * 256) + bytes[start + 8] as usize]
			^ TABLE[(6 * 256) + bytes[start + 9] as usize]
			^ TABLE[(5 * 256) + bytes[start + 10] as usize]
			^ TABLE[(4 * 256) + bytes[start + 11] as usize];

		let c = TABLE[(11 * 256) + (bytes[start + 4] as usize)]
			^ TABLE[(10 * 256) + bytes[start + 5] as usize]
			^ TABLE[(9 * 256) + bytes[start + 6] as usize]
			^ TABLE[(8 * 256) + bytes[start + 7] as usize];

		let d = TABLE[(15 * 256) + (working as u8 ^ bytes[start]) as usize]
			^ TABLE[(14 * 256) + ((working >> 8) as u8 ^ bytes[start + 1]) as usize]
			^ TABLE[(13 * 256) + ((working >> 16) as u8 ^ bytes[start + 2]) as usize]
			^ TABLE[(12 * 256) + ((working >> 24) as u8 ^ bytes[start + 3]) as usize];

		working = d ^ c ^ b ^ a;
		start += 16;
		end -= 16;
	}

	for _ in 0..end {
		working = TABLE[(working as u8 ^ bytes[start]) as usize] ^ working >> 8;
		start += 1;
	}

	!(working ^ u32::MAX)
}
