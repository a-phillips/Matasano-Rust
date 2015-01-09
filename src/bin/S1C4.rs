extern crate serialize;

use serialize::hex::FromHex;
use std::io::BufferedReader;
use std::io::File;

fn get_data(fp: &str) -> Vec<String> {
	let path = Path::new(fp);
	let mut file = BufferedReader::new(File::open(&path));
	let data_w_newlines:Vec<String> = file.lines().map(|x| x.unwrap()).collect();
	let mut data_vec:Vec<String> = Vec::new();
	for data in data_w_newlines.iter() {
		data_vec.push(data.as_slice()
						.slice_to(data.as_slice().len()-1) //get rid of newlines
						.to_string());
	}
	data_vec
}

fn single_char_xor_decode(encrypted_data_fp: &str, words_fp: &str) {
	//would love to move this from an ugly multiple for loop to separate functions but can't figure
	//out how to work out the ownership of the words vector - it always breaks if I try to pass it
	//to a function.
	let encrypted_vec = get_data(encrypted_data_fp);
	let word_vec = get_data(words_fp);
	for encrypted_hex in encrypted_vec.iter() {
		let bytes = encrypted_hex.from_hex().unwrap();
		for key in range(0u8, 255) {
			let decoded_str = String::from_utf8(bytes.iter().map(|byte| {*byte^key} ).collect());
			let valid_str = match decoded_str {
				Ok(v) => v,
				Err(_) => "ignore".to_string(),
				};
			if valid_str != "ignore" {
				'outer: for word in valid_str.words() {
					for check_word in word_vec.iter() {
						if (word.as_slice() == check_word.as_slice()) && (check_word.as_slice().len() > 1) {
							//added requirement for check_word to be more than 1 letter since the words file I have
							//lists all single etters as words for some reason, which results in lots of garbage strings
							//like "hY6[@PtVw k lC]^" passing since "k" gets parsed as a word.
							//This actually results in only the true answer passing.
							println!("decode: {} \n\
							 		word: {} \n\
							 		check_word: {} \n\
							 		hex: {}",
									valid_str.as_slice()
												.slice_to(valid_str.as_slice().len()-1), //why does valid_str have newlines?
									word,
									check_word,
									encrypted_hex);
							break 'outer;
						}
					}
				}
			}
		}
	}
}

fn main() {
	let words_fp = "src/words.txt".as_slice();
	let encrypted_data_fp = "src/S1C4.txt".as_slice();
	single_char_xor_decode(encrypted_data_fp, words_fp);
	//decode: Now that the party is jumping
	//word: that
	//check_word: that
	//hex: 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
}