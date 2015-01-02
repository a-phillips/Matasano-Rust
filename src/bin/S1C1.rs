extern crate serialize;

use serialize::hex::FromHex;
use serialize::base64::{ToBase64, mod};

fn main() {
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let bytes = input.from_hex().unwrap();
	//println!("{}", bytes);
	let output = bytes.to_base64(base64::STANDARD);
	println!("{}", output);
	assert_eq!(output, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}