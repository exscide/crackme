use crackme::*;

use std::arch::asm;
use std::ffi::CString;

use winapi::shared::minwindef::LPVOID;
use winapi::um::memoryapi::{VirtualFree, VirtualProtect};
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, PAGE_EXECUTE_READ, PAGE_READWRITE};

const TYPE_PW: &[u8] = "\u{19}\u{8}\u{1a}\u{1a}\u{1e}\u{6}\u{1b}\rSI".as_bytes();
const CORRECT: &[u8] = "\n\u{6}\u{1b}\u{1b}\u{c}\n\u{1d}".as_bytes();
const WRONG: &[u8] = "\u{1e}\u{1b}\u{6}\u{7}\u{e}".as_bytes();

const CODE_ENCODED: &[u8] = include_bytes!("../target/ext-enc");

fn main() {
	decode_print(TYPE_PW);

	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();

	let input = CString::new(input.trim()).unwrap();

	let ext = unsafe {
		let mut sys_info = SYSTEM_INFO::default();
		GetSystemInfo(&mut sys_info as *mut SYSTEM_INFO);

		let ext = winapi::um::memoryapi::VirtualAlloc(
			0 as LPVOID,
			sys_info.dwPageSize as usize,
			MEM_COMMIT,
			PAGE_READWRITE,
		);
		assert!(!ext.is_null());

		for i in 0..CODE_ENCODED.len() {
			*(ext as *mut u8).add(i) = CODE_ENCODED[i].reverse_bits() ^ !KEY;
		}

		let mut _dummy = 0u32;
		VirtualProtect(ext, CODE_ENCODED.len(), PAGE_EXECUTE_READ, &mut _dummy);

		ext
	};

	let mut _test = 0u8;

	unsafe {
		asm!(
			// set input ptr
			"mov r8, {input}",

			// set pointer to code
			"mov rdx, {ext}",
			// set "return" address
			"lea rcx, [rip+2f]",
			// jump to code
			"jmp rdx",

			// continue
			"2:",
			"mov {test},r10b",
			ext = in(reg) ext as u64,
			test = out(reg_byte) _test,
			input = in(reg) input.as_ptr() as u64
		);
	}

	if _test == 1 {
		decode_print(CORRECT);
	} else {
		decode_print(WRONG);
	}

	println!("");

	unsafe {
		VirtualFree(ext, 0, MEM_RELEASE);
	}
}
