use std::str;

// Consider a byte array like [0b1000001, 0b0xxxxxxx, 0b1000001]
// where x denote arbitrary bits. Continue only if the array
// is a valid UTF-8 string. Now introduce one arbitrary bit
// flip in any bit in the middle byte. Does this give a valid UTF-8 string?
// Returns the total number of tests and invalid strings.
fn flips_in_1_byte() -> (u64, u64) {
	let mut total = 0u64;
	let mut invalid = 0u64;

	let mut text = ['A' as u8, 0b0_0000000u8, 'A' as u8];
	for base in 0..128u8 {
		for bit_flip in 0..8 {
			// base defines the x-values in 0xxxxxxx
			// bit_flip defines the index of the bit to flip
			text[1] = base & 0b0111_1111;

			// apply bit flip
			text[1] = text[1] ^ (1 << bit_flip);

			match str::from_utf8(&text) {
				Ok(_) => {},
				Err(_) => invalid += 1,
			};

			total += 1;
		}
	}

	(total, invalid)
}

// Consider a byte array like [0b1000001, 0b110xxxxx, 0b10xxxxxx, 0b1000001]
// where x denote arbitrary bits. Continue only if the array is a valid UTF-8 string.
// Now introduce one arbitrary bit flip in any bit of the middle bytes.
// Does this give a valid UTF-8 string?
// Returns the total number of tests and invalid strings.
fn flips_in_2_bytes() -> (u64, u64) {
	let mut total = 0u64;
	let mut invalid = 0u64;

	let mut text = ['A' as u8, 0b110_00000u8, 0b10_000000, 'A' as u8];
	for base in 0..2048u16 {
		for bit_flip in 0..16 {
			// base defines the x-values in 110xxxxx 10xxxxxx
			// bit_flip defines the index of the bit to flip
			text[1] = 0b1100_0000 | (base & 0b1_1111) as u8;
			text[2] = 0b1000_0000 | (base >> 5) as u8;

			if str::from_utf8(&text).is_err() {
				continue
			}

			// apply bit flip
			if bit_flip < 8 {
				text[1] = text[1] ^ (1 << bit_flip);
			} else {
				text[2] = text[2] ^ (1 << (bit_flip - 8));
			}

			match str::from_utf8(&text) {
				Ok(_) => {},
				Err(_) => invalid += 1,
			};

			total += 1;
		}
	}

	(total, invalid)
}

// Consider a byte array like [0b1000001, 0b1110xxxx, 0b10xxxxxx,
// 0b10xxxxxx, 0b1000001] where x denote arbitrary bits.
// Continue only if the array is a valid UTF-8 string.
// Now introduce one arbitrary bit flip in any bit of the 3 middle bytes.
// Does this give a valid UTF-8 string?
// Returns the total number of tests and invalid strings.
fn flips_in_3_bytes() -> (u64, u64) {
	let mut total = 0u64;
	let mut invalid = 0u64;

	let mut text = ['A' as u8, 0b1110_0000u8, 0b10_000000, 0b10_000000, 'A' as u8];
	for base in 0..=65535u16 {
		for bit_flip in 0..24 {
			// base defines the x-values in 1110xxxx 10xxxxxx 10xxxxxx
			// bit_flip defines the index of the bit to flip
			text[1] = 0b1110_0000 | (base & 0b1111) as u8;
			text[2] = 0b1000_0000 | ((base >> 4) & 0b0011_1111) as u8;
			text[3] = 0b1000_0000 | ((base >> 10) & 0b0011_1111) as u8;

			if str::from_utf8(&text).is_err() {
				continue
			}

			// apply bit flip
			if bit_flip < 8 {
				text[1] = text[1] ^ (1 << bit_flip);
			} else if bit_flip < 16 {
				text[2] = text[2] ^ (1 << (bit_flip - 8));
			} else {
				text[3] = text[3] ^ (1 << (bit_flip - 16));
			}

			match str::from_utf8(&text) {
				Ok(_) => {},
				Err(_) => invalid += 1,
			};

			total += 1;
		}
	}

	(total, invalid)
}

// Consider a byte array like [0b1000001, 0b11110xxx, 0b10xxxxxx,
// 0b10xxxxxx, 0b10xxxxxx, 0b1000001] where x denote arbitrary bits.
// Continue only if the array is a valid UTF-8 string.
// Now introduce one arbitrary bit flip in any bit of the 4 middle bytes.
// Does this give a valid UTF-8 string?
// Returns the total number of tests and invalid strings.
fn flips_in_4_bytes() -> (u64, u64) {
	let mut total = 0u64;
	let mut invalid = 0u64;

	let mut text = ['A' as u8, 0b1111_0000u8, 0b10_000000, 0b10_000000, 0b10_000000, 'A' as u8];
	for base in 0..=2097152u32 {
		for bit_flip in 0..32 {
			// base defines the x-values in 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
			// bit_flip defines the index of the bit to flip
			text[1] = 0b1111_0000 | (base & 0b111) as u8;
			text[2] = 0b1000_0000 | ((base >> 3) & 0b0011_1111) as u8;
			text[3] = 0b1000_0000 | ((base >> 9) & 0b0011_1111) as u8;
			text[4] = 0b1000_0000 | ((base >> 15) & 0b0011_1111) as u8;

			if str::from_utf8(&text).is_err() {
				continue
			}

			// apply bit flip
			if bit_flip < 8 {
				text[1] = text[1] ^ (1 << bit_flip);
			} else if bit_flip < 16 {
				text[2] = text[2] ^ (1 << (bit_flip - 8));
			} else if bit_flip < 24 {
				text[3] = text[3] ^ (1 << (bit_flip - 16));
			} else {
				text[4] = text[4] ^ (1 << (bit_flip - 24));
			}

			match str::from_utf8(&text) {
				Ok(_) => {},
				Err(_) => invalid += 1,
			};

			total += 1;
		}
	}

	(total, invalid)
}

fn main() {
	println!("Byte sequence [A, 0xxx xxxx, A]");
	let (total_1, invalid_1) = flips_in_1_byte();
	println!("total number of tests: {}", total_1);
	println!("total number of invalid byte sequences: {}", invalid_1);

	println!("Byte sequence [A, 110x xxxx, 10xx xxxx, A]");
	let (total_2, invalid_2) = flips_in_2_bytes();
	println!("total number of tests: {}", total_2);
	println!("total number of invalid byte sequences: {}", invalid_2);

	println!("Byte sequence [A, 1110 xxxx, 10xx xxxx, 10xx xxxx, A]");
	let (total_3, invalid_3) = flips_in_3_bytes();
	println!("total number of tests: {}", total_3);
	println!("total number of invalid byte sequences: {}", invalid_3);

	println!("Byte sequence [A, 1111 0xxx, 10xx xxxx, 10xx xxxx, 10xx xxxx, A]");
	let (total_4, invalid_4) = flips_in_4_bytes();
	println!("total number of tests: {}", total_4);
	println!("total number of invalid byte sequences: {}", invalid_4);
}
