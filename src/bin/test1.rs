use std::arch::asm;

fn main() {
	let mut x = 2_000_000u64;
	unsafe {
		asm!(
			"mov r9d, 0x68e989fd",
			"ror r9, 32",
			"mov r10d, 0xd8a80000",
			"add r9, r10",

			"mov {x},r9",
			x = inout(reg) x,
		)
	}
	println!("{:064b}", x);
}
