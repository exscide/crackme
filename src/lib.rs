use std::io::Write;

pub const KEY: u8 = 0b01101001;

pub fn decode(d: &[u8]) -> Vec<u8> {
	d.iter().map(|a| a ^ KEY).collect::<Vec<u8>>()
}

#[inline(always)]
pub fn decode_print(d: &[u8]) {
	for c in d {
		print!("{}", char::from_u32((*c ^ KEY) as u32).unwrap());
	}

	std::io::stdout().flush().unwrap();
}
