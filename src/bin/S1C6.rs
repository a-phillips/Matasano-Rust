extern crate serialize;
use serialize::base64::{FromBase64, ToBase64, mod};
use std::num::Int;

fn get_min_dists(input:&str, return_num:uint) -> Vec<uint> {
	let mut results:Vec<(f64, uint)> = Vec::new();
	let mut all_sizes:Vec<f64> = Vec::new();
	all_sizes.push(1000f64); //not using 0 (index represents KEYSIZE)
	all_sizes.push(1000f64); //not using 1
	for keysize in range(2u, input.len()/2) {
		let str1 = input.slice(0u, keysize);
		let str2 = input.slice(keysize, keysize*2);
		let norm_dist = (hamming_distance(str1, str2) as f64)/(keysize as f64);
		println!("keysize:{}, dist:{}, norm_dist{}", keysize, norm_dist*(keysize as f64), norm_dist);
		results.push( (norm_dist, keysize) );
		all_sizes.push(norm_dist);
	}
	println!("{}", results);
	all_sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());
	let mut best_sizes:Vec<uint> = Vec::new();
	for i in range(0u, return_num) {
		for pair in results.iter() {
			if pair.val0() == all_sizes[i] { best_sizes.push(pair.val1()); }
		}
	}
	best_sizes
}

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
	let input = "HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS";
	//let input2 = "BgBHVBwNRU0HBAxTEjwMHghJGgkRTxRMIRpHKwAFHUdZEQQJAGQmB1MANxYG";
	let input_str = String::from_utf8(input.as_slice().from_base64().unwrap()).unwrap();
	//let str2 = String::from_utf8(input2.as_slice().from_base64().unwrap()).unwrap();
	let answer = get_min_dists(input_str.as_slice(), 3);
	println!("{}", answer);
}