use std::io::Write;

pub const KEY: u8 = 0b01101001;

pub fn decode(d: &[u8]) -> Vec<u8> {
	d.iter().map(|a| a ^ KEY).collect::<Vec<u8>>()
}

pub fn decode_print(d: &[u8]) {
	print!(
		"{}",
		std::str::from_utf8(&d.iter().map(|a| a ^ KEY).collect::<Vec<u8>>()).unwrap()
	);
	std::io::stdout().flush().unwrap();
}
