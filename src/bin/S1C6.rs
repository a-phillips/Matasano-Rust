extern crate serialize;
use serialize::base64::{FromBase64, ToBase64, mod};
use std::num::Int;

fn hamming_distance(str1:&str, str2:&str) -> uint {
	let bytes1 = str1.as_bytes();
	let bytes2 = str2.as_bytes();
	//println!("{}", bytes1);
	//println!("{}", bytes2);
	let bytes_xor:uint = bytes1.iter()
								.zip(bytes2.iter())
								.map(| (b1, b2) | { *b1^*b2 })
								.fold(0u, |acc, x| { acc + x.count_ones() });
								//.collect();
	bytes_xor
}

fn main() {
	/*let input1 = "HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS";
	let input2 = "BgBHVBwNRU0HBAxTEjwMHghJGgkRTxRMIRpHKwAFHUdZEQQJAGQmB1MANxYG";
	let str1 = String::from_utf8(input1.as_slice().from_base64().unwrap()).unwrap();
	let str2 = String::from_utf8(input2.as_slice().from_base64().unwrap()).unwrap();
	*/
	let str1 = "this is a test";
	let str2 = "wokka wokka!!!";
	println!("{}", str1);
	println!("{}", str2);
	let result = hamming_distance(str1.as_slice(),str2.as_slice());
	println!("{}", result);
}