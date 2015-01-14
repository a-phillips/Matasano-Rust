extern crate serialize;
use serialize::base64::FromBase64;
use std::num::Int;
use std::io::BufferedReader;
use std::io::File;

// 0: import the data
fn get_encrypted_string(fp:&str) -> String {
	let path = Path::new(fp);
	let mut file = BufferedReader::new(File::open(&path));
	let str_w_newlines:Vec<String> = file.lines().map(|x| x.unwrap()).collect();
	let mut str_base64:String = String::new();
	for str in str_w_newlines.iter() {
		str_base64.push_str(str.as_slice()
								.slice_to(str.as_slice().len()-1)); //Do we need newlines?
	}
	str_base64
}

// 2: function to computer Hamming distance
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

// 3&4: calculate normalized hamming distances, return lowest few
fn get_min_dists(input:&str, return_num:uint) -> Vec<uint> {

	//results will be a vector of (norm_dist, keysize) to relate each keysize with its respective norm_dist
	let mut results:Vec<(f64, uint)> = Vec::new();

	//all_sizes is a vector of all norm_dists that are calculated,
	//will be sorted to get the top return_num norm_dist's
	let mut all_sizes:Vec<f64> = Vec::new();
	all_sizes.push(1000f64); //not using 0 (index represents KEYSIZE)
	all_sizes.push(1000f64); //not using 1

	//for each keysize, get the first string of length keysize and then the next string
	//of length keysize, then calculate norm_dist and store the results using results and all_sizes
	for keysize in range(2u, input.len()/2) {
		let str1 = input.slice(0u, keysize);
		let str2 = input.slice(keysize, keysize*2);
		let norm_dist = (hamming_distance(str1, str2) as f64)/(keysize as f64);
		//println!("keysize: {}, dist: {}, norm_dist: {}", keysize, norm_dist*(keysize as f64), norm_dist);
		results.push( (norm_dist, keysize) );
		all_sizes.push(norm_dist);
	}

	//sort all_sizes using the partial_cmp() function of floats
	all_sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());

	//best_sizes will hold the top return_num norm_dist's
	//pair.val0() is the norm_dist, and pair.val1() is the keysize
	let mut best_sizes:Vec<uint> = Vec::new();
	for i in range(0u, return_num) {
		for pair in results.iter() {
			if pair.0 == all_sizes[i] {
				best_sizes.push(pair.1);
				// need to check vector size, since if many numbers have the same norm_dist they
				// will all be added even though we only want a certain number
				if best_sizes.len() == return_num {
					return best_sizes;
				}
			}
		}
	}
	best_sizes
}

// 5&6 : break the input down into bytes and transpose into a vector of vectors
fn transpose_to_bytes(input:&str, keysize:uint) -> Vec<Vec<u8>> {
	let input_bytes = input.to_string().into_bytes();
	let mut bytes_vec:Vec<Vec<u8>> = Vec::new();

	// make vector of length keysize, with vector elements
	for _ in range(0u, keysize) {
		bytes_vec.push(Vec::new());
	}

	//loop though the bytes and place them in the appropriate vector in bytes_vec
	let mut i = 0u;
	for byte in input_bytes.iter() {
		bytes_vec[i].push(*byte);
		if i == (keysize-1) { i = 0; }
		else { i += 1; }
	}
	bytes_vec
}

// new method of checking bytes for validity - vector of bytes must contain only
// numbers 32-126, based on http://web.cs.mun.ca/~michael/c/ascii-table.html
// these are all actual characters, 32 is a space, and 10 is a newline
// ...this doesn't work, need to find a valid check.
fn valid_bytes(byte_vec:Vec<u8>) -> bool {
	//println!("{}", byte_vec);
	for byte in byte_vec.iter() {
		if ( (*byte<32) && (*byte != 10) ) || (*byte == 127) { return false; }
	}
	true
}

fn solve_keysize(input:&str, keysize:uint) {
	let bytes_vec = transpose_to_bytes(input, keysize);
	println!("Solving for keysize {}", keysize);
	for i in range(0u, keysize) {
		for key in range(0u8, 128) {
			let bytes_xor:Vec<u8> = bytes_vec[i].iter().map(|byte| {*byte^key} ).collect();
			if valid_bytes(bytes_xor) {
				println!("keysize: {}, i: {}, key: {}",
						 keysize,
						 i,
						 key);
			}
		}
	}
}

fn main() {
	let data = get_encrypted_string("src/S1C6.txt".as_slice());
	let input_str = String::from_utf8(data.as_slice().from_base64().unwrap()).unwrap();
	let min_dists = get_min_dists(input_str.as_slice(), 3);
	println!("{}", min_dists);
	println!("{}", input_str.to_string().into_bytes().slice_to(50));
	let bytes_vec = transpose_to_bytes(input_str.as_slice(), min_dists[0]);
	println!("{}", bytes_vec[1].slice_to(10));
	solve_keysize(input_str.as_slice(), 5u);
}