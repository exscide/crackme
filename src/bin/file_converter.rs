use std::io::{ Read, Write };

use crackme::*;

fn main() {
	let src = std::env::args().nth(1).unwrap();
	let dst = std::env::args().nth(2).unwrap();

	let mut file = std::fs::File::open(src).unwrap();

	let mut buf = Vec::new();
	file.read_to_end(&mut buf).unwrap();

	let mut file = std::fs::File::options()
		.read(false)
		.write(true)
		.create(true)
		.truncate(true)
		.open(dst)
		.unwrap();

	file.write_all(
		&buf.into_iter()
			.map(|b| (b ^ !KEY).reverse_bits())
			.collect::<Vec<u8>>(),
	)
	.unwrap();
}
