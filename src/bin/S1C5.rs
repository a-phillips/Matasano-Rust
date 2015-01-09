extern crate serialize;
use serialize::hex::ToHex;

fn repeating_key_xor(input:Vec<&str>, key:&str) -> String {
	let key_bytes = key.as_bytes();
	let mut encrypted_hex = String::new();
	let mut i = 0u;
	let max_i = key.len() - 1u;
	for line in input.iter() {
		let line_bytes = line.as_bytes();
		let mut xor_bytes:Vec<u8> = Vec::new();
		for byte in line_bytes.iter() {
			xor_bytes.push(*byte ^ key_bytes[i]);
			//println!("{}, {}, {}", *byte, key_bytes[i], *byte ^ key_bytes[i]);
			if i == max_i { i = 0;} else { i += 1;}
		}
		encrypted_hex.push_str(xor_bytes.to_hex().as_slice());
		encrypted_hex.push_str("\n");
		//println!("{}", encrypted_hex);
	}
	encrypted_hex
}

fn main() {
	//not necessary to put into a vector, but wanted to make sure I would be able to work with strings in a
	//vector for this kind of encryption - feels like it will be useful.
	let input = vec!("Burning 'em, if you ain't quick and nimble\n".as_slice(),
					"I go crazy when I hear a cymbal".as_slice());
	let key = "ICE".as_slice();
	println!("{}", repeating_key_xor(input, key));
	//I get the right hex, but the lines break at a different point from the website. I'm inclined to think that
	//the break in the hex form the website is just for formatting, not to indicate that it represents
	//the second line of text.
}