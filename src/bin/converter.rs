use crackme::*;

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    println!("{:?}", String::from_utf8(decode(arg.as_bytes())).unwrap());
}
