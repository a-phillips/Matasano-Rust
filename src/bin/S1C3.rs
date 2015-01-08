extern crate serialize;

use serialize::hex::FromHex;
use std::io::BufferedReader;
use std::io::File;

fn get_words(fp: &str) -> Vec<String> {
	let path = Path::new(fp);
	let mut file = BufferedReader::new(File::open(&path));
	println!("Compiling word list...");
	let word_vec:Vec<String> = file.lines().map(|x| x.unwrap()).collect();
	println!("Word list complete!");
	word_vec
}

fn decode(hex_input:&str, words_fp:&str) {
	let bytes = hex_input.from_hex().unwrap();
	let word_vec = get_words(words_fp);
	//brute force through keys
	for key in range(0u8, 255) {
		let decoded_str = String::from_utf8(bytes.iter().map( |byte| {*byte^key} ).collect());
		let valid_str = match decoded_str {
			Ok(v) => v,
			Err(_) => "ignore".to_string(), //Breaks at 128, don't know why
		};
		if valid_str != "ignore" {
			'outer: for word in valid_str.words() { //need to break this loop if word found, lebeled "outer" for break
				for check_word in word_vec.iter() {
					if word.as_slice() == check_word.as_slice().slice_to(check_word.len() - 1) { //check_word has newline at end
						println!("{}", valid_str); //print a result that has some word that exists in word_vec
						break 'outer;
					}
				}
			}
		}
	}
}

fn main() {
	let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	let words_fp = "src/words.txt"; //google's 10,000 words list
	decode(input.as_slice(), words_fp.as_slice());
	//"X" - Cooking MC's like a pound of bacon
}