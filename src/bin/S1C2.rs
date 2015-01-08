//why is converting to binary and xor-ing so difficult?
//turns out, it's not! thanks sean!

extern crate serialize;

use serialize::hex::{FromHex, ToHex};

fn fixed_xor_hex(buff1: &str, buff2: &str) -> String {
	let bytes1 = buff1.from_hex().unwrap();
	let bytes2 = buff2.from_hex().unwrap();
	let bytes_xor:Vec<u8> = bytes1.iter()
							.zip(bytes2.iter())
							.map(| (b1, b2) | {*b1^*b2} )
							.collect();
	bytes_xor.to_hex()
}

fn main() {
	let input = "1c0111001f010100061a024b53535009181c";
	let xor_against = "686974207468652062756c6c277320657965";
	let answer = fixed_xor_hex(input, xor_against);
	println!("{}", answer);
	assert_eq!("746865206b696420646f6e277420706c6179", answer);
}