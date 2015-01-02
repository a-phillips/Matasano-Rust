//why is converting to binary and xor-ing so difficult?

extern crate serialize;

use serialize::hex::{FromHex, ToHex};
use std::num::Int;

fn hex_to_bin_vec(hex_str:&str) -> Vec<String> {
	let mut bin_vec = hex_str.from_hex()
							.unwrap()
							.iter()
							.map(|x| format!("{:b}", *x))
							.collect::<Vec<String>>();
	//pad with 0s
	for i in range(0u, bin_vec.len()) {
		let mut bin_8 = String::new();
		for _ in range(0u, 8-bin_vec[i].len()) {
			//println!("{}", j);
			bin_8.push('0');
		}
		bin_8.push_str(bin_vec[i].as_slice());
		bin_vec[i] = bin_8;
	}
	bin_vec
}

fn xor(str1:&str, str2:&str) -> String {
	let mut result = String::new();
	for i in range(0u, str1.len()) {
		if str1.slice(i, i+1) == str2.slice(i, i+1) { result.push('0'); }
		else { result.push('1'); }
	}
	result
}

fn bin_to_u8(bin:&str) -> u8 {
	let mut result = 0u;
	let mut raise_to = bin.len()-1;
	for num in bin.chars() {
		result += (2u.pow(raise_to))*num.to_digit(10).unwrap();
		raise_to -= 1;
	}
	result as u8
}

fn fixed_xor_hex(buff1: &str, buff2: &str) -> String {
	let bin1 = hex_to_bin_vec(buff1);
	let bin2 = hex_to_bin_vec(buff2);
	let mut bin_xor = Vec::new();
	for i in range(0u, bin1.len()) {
		bin_xor.push(bin_to_u8(xor(bin1[i].as_slice(), bin2[i].as_slice()).as_slice()));
	}
	bin_xor.to_hex()
}

fn main() {
	let input = "1c0111001f010100061a024b53535009181c";
	let xor_against = "686974207468652062756c6c277320657965";
	let answer = fixed_xor_hex(input, xor_against);
	println!("{}", answer);
	assert_eq!("746865206b696420646f6e277420706c6179", answer);
}