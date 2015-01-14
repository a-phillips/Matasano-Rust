//Finally have a full, somewhat generalized solution!

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

// 2: function to compute Hamming distance
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
fn get_min_dists(input:&str, max_key:uint, return_num:uint) -> Vec<uint> {

	//results will be a vector of (norm_dist, keysize) to relate each keysize with its respective norm_dist
	let mut results:Vec<(f64, uint)> = Vec::new();

	//all_sizes is a vector of all norm_dists that are calculated,
	//will be sorted to get the top return_num norm_dist's
	let mut all_sizes:Vec<f64> = Vec::new();
	all_sizes.push(1000f64); //not using 0 (index represents KEYSIZE)
	all_sizes.push(1000f64); //not using 1

	//for each keysize, get the first string of length keysize and then the next string
	//of length keysize, then calculate norm_dist and store the results using results and all_sizes
	for keysize in range(2u, max_key+1) {
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

//gotta reverse the transpose
fn flatten(bytes_vec:Vec<Vec<u8>>) -> Vec<u8> {
	let mut vec1d:Vec<u8> = Vec::new();
	for row in range(0u, bytes_vec[0].len()) {
		for col in range(0u, bytes_vec.len()) {
			//vectors may not all be same length - as soon as we would have an index error, we've reached the end
			if row == bytes_vec[col].len() { break; }
			vec1d.push(bytes_vec[col][row]);
		}
	}
	vec1d
}

// new method of checking bytes for validity - count number of bytes in vector that are
// numbers 32-126, based on http://web.cs.mun.ca/~michael/c/ascii-table.html
// these are all actual characters, 32 is a space, and 10 is a newline
// revision: count+2 for letter, count+1 for other valid characters
// revision 2: count+3 for letters, count+2 for a space, count+1 for other valid characters
// revision 2 works!
fn valid_bytes(byte_vec:Vec<u8>) -> uint {
	let mut count_valid = 0u;
	for byte in byte_vec.iter() {
		if ( (*byte>=65)&&(*byte<=90) ) || ( (*byte>=97)&&(*byte<=122) ) { count_valid+=3; }
		else if *byte==32 { count_valid+=2; }
		else if (*byte>=32) && (*byte!=127) { count_valid+=1; }
	}
	count_valid
}

fn solve_keysize(input:&str, keysize:uint) -> (String, String) {
	let bytes_vec = transpose_to_bytes(input, keysize);
	let mut best_bytes_vec:Vec<Vec<u8>> = Vec::new();
	let mut best_keys:Vec<u8> = Vec::new();
	//println!("Solving for keysize {}", keysize);
	for i in range(0u, keysize) {
		let mut max_valid_bytes = 0u;
		let mut best_key = 0u8;
		for key in range(0u8, 128) {
			let bytes_xor:Vec<u8> = bytes_vec[i].iter().map(|byte| {*byte^key} ).collect();
			let valid_bytes_count = valid_bytes(bytes_xor);
			if valid_bytes_count > max_valid_bytes {
				max_valid_bytes = valid_bytes_count;
				best_key = key;
			}
		}
		best_keys.push(best_key);
		best_bytes_vec.push(bytes_vec[i].iter().map(|byte| {*byte^best_key} ).collect());
	}
	let return_key = String::from_utf8(best_keys).unwrap();
	let return_decode = String::from_utf8(flatten(best_bytes_vec)).unwrap();
	(return_key, return_decode)
}

fn solve(input_str:&str) -> (String, String) {
	let min_dists = get_min_dists(input_str.as_slice(), 40u, 40);
	println!("checking order: {}", min_dists)
	let mut answer:(String, String) = ("blah".to_string(), "blah".to_string());
	let mut max_valid_count = 0u;
	for dist in min_dists.iter() {
		let result = solve_keysize(input_str, *dist);
		let result_valid_count = valid_bytes(result.clone().1.into_bytes());
		if result_valid_count > max_valid_count {
			max_valid_count = result_valid_count;
			answer = result;
		}
	}
	answer
}

fn main() {
	let data = get_encrypted_string("src/S1C6.txt".as_slice());
	let input_str = String::from_utf8(data.as_slice().from_base64().unwrap()).unwrap();
	let answer = solve(input_str.as_slice());
	println!("Key: {}", answer.0);
	println!("Decoded:\n{}", answer.1);
}
/*Key: Terminator X: Bring the noise
Decoded:
I'm back and I'm ringin' the bell
A rockin' on the mike while the fly girls yell
In ecstasy in the back of me
Well that's my DJ Deshay cuttin' all them Z's
Hittin' hard and the girlies goin' crazy
Vanilla's on the mike, man I'm not lazy.

I'm lettin' my drug kick in
It controls my mouth and I begin
To just let it flow, let my concepts go
My posse's to the side yellin', Go Vanilla Go!

Smooth 'cause that's the way I will be
And if you don't give a damn, then
Why you starin' at me
So get off 'cause I control the stage
There's no dissin' allowed
I'm in my own phase
The girlies sa y they love me and that is ok
And I can dance better than any kid n' play

Stage 2 -- Yea the one ya' wanna listen to
It's off my head so let the beat play through
So I can funk it up and make it sound good
1-2-3 Yo -- Knock on some wood
For good luck, I like my rhymes atrocious
Supercalafragilisticexpialidocious
I'm an effect and that you can bet
I can take a fly girl and make her wet.

I'm like Samson -- Samson to Delilah
There's no denyin', You can try to hang
But you'll keep tryin' to get my style
Over and over, practice makes perfect
But not if you're a loafer.

You'll get nowhere, no place, no time, no girls
Soon -- Oh my God, homebody, you probably eat
Spaghetti with a spoon! Come on and say it!

VIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino
Intoxicating so you stagger like a wino
So punks stop trying and girl stop cryin'
Vanilla Ice is sellin' and you people are buyin'
'Cause why the freaks are jockin' like Crazy Glue
Movin' and groovin' trying to sing along
All through the ghetto groovin' this here song
Now you're amazed by the VIP posse.

Steppin' so hard like a German Nazi
Startled by the bases hittin' ground
There's no trippin' on mine, I'm just gettin' down
Sparkamatic, I'm hangin' tight like a fanatic
You trapped me once and I thought that
You might have it
So step down and lend me your ear
'89 in my time! You, '90 is my year.

You're weakenin' fast, YO! and I can tell it
Your body's gettin' hot, so, so I can smell it
So don't be mad and don't be sad
'Cause the lyrics belong to ICE, You can call me Dad
You're pitchin' a fit, so step back and endure
Let the witch doctor, Ice, do the dance to cure
So come up close and don't be square
You wanna battle me -- Anytime, anywhere

You thought that I was weak, Boy, you're dead wrong
So come on, everybody and sing this song

Say -- Play that funky music Say, go white boy, go white boy go
play that funky music Go white boy, go white boy, go
Lay down and boogie and play that funky music till you die.

Play that funky music Come on, Come on, let me hear
Play that funky music white boy you say it, say it
Play that funky music A little louder now
Play that funky music, white boy Come on, Come on, Come on
Play that funky music */

