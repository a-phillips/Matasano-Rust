//should probably create a module to store this functionality...

extern crate serialize;

use serialize::hex::FromHex;
use std::num::Int;

fn hex_to_bin_vec(hex_str:&str) -> Vec<String> {
	let mut bin_vec = hex_str.from_hex()
							.unwrap()
							.iter()
							.map(|x| format!("{:b}", *x))
							.collect::<Vec<String>>();
	//pad with 0s
	pad_bin_with_0s(bin_vec)
}

fn pad_bin_with_0s(input_bin_vec:Vec<String>) -> Vec<String> {
	let mut bin_vec = input_bin_vec;
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

fn decode(hex_input:&str) {
	let bin = hex_to_bin_vec(hex_input.as_slice());
	let decoders = pad_bin_with_0s(range(0u64, 256).map(|x| format!("{:b}", x)).collect::<Vec<String>>());
	//println!("{}", bin);
	//println!("{}", decoders);
	//brute force through the decoders
	for key in decoders.iter() {
		let mut bin_xor = Vec::new();
		for i in range(0u, bin.len()) {
			bin_xor.push(bin_to_u8(xor(bin[i].as_slice(), key.as_slice()).as_slice()));
		}
		println!("{}, {}", bin_to_u8(key.as_slice()), String::from_utf8(bin_xor).unwrap());
	}
}

fn main() {
	let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	decode(input.as_slice());
	//"X" - Cooking MC's like a pound of bacon
	//process breaks at 128th iteration, need to investigate...
}