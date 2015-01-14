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
fn valid_bytes(byte_vec:Vec<u8>) -> uint {
	let mut count_valid = 0u;
	for byte in byte_vec.iter() {
		if ( (*byte>=32) || (*byte == 10) ) && (*byte != 127) { count_valid += 1; }
	}
	count_valid
}

fn solve_keysize(input:&str, keysize:uint) {
	let bytes_vec = transpose_to_bytes(input, keysize);
	let mut best_bytes_vec:Vec<Vec<u8>> = Vec::new();
	let mut best_keys:Vec<u8> = Vec::new();
	println!("Solving for keysize {}", keysize);
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
	println!("{}", String::from_utf8(flatten(best_bytes_vec)).unwrap());
}


fn main() {
	let data = get_encrypted_string("src/S1C6.txt".as_slice());
	let input_str = String::from_utf8(data.as_slice().from_base64().unwrap()).unwrap();
	let min_dists = get_min_dists(input_str.as_slice(), 40u, 40);
	println!("{}", min_dists);
	for dist in min_dists.iter() {
		solve_keysize(input_str.as_slice(), *dist);
	}
	/*So close! at keysize 29:
	I'm back an> I'xiringik' the bell
A roc1in'5&n the%mike while the fl# gig%s yeli
In ecstasy in t2e bt*k of he
Well that's myzDJ Q,shay futtin' all them Z}s
] ttin'%hard and the girl3es r&in' cwazy
Vanilla's onzthe5$ike, han I'm not lazy. P
I'xilettik' my drug kick inz
It5*ontrois my mouth and I 8egi{i
To jpst let it flow, l?t mliconceuts go
My posse'szto a!e sid` yellin', Go Vani6la R&!

Shooth 'cause that') thpiway I%will be
And if y5u dz''t gise a damn, then
W2y yz< starln' at me
So get 5ff 2*ause L control the stag?
T},re's ko dissin' allowedz
I'xiin my%own phase
The gi(liefisa y qhey love me and t2at |: ok
Dnd I can dance be.ter5=han aky kid n' play

S.age5{ -- Y`a the one ya' wan4a l|:ten tj
It's off my hea> so5%et th` beat play throug2
SziI can%funk it up and ma1e iaisound%good
1-2-3 Yo --zKnov" on sjme wood
For goodzluc~e I line my rhymes atroc3ous5CSuperfalafragilisticexp3aliq&cious%
I'm an effect an> tht= you fan bet
I can tak? a s%y giri and make her wett

\nm lik` Samson -- Samsonzto Q,lilah%
There's no denyi4', L&u can%try to hang
But #ou'y% keep%tryin' to get my )tylpi
Over%and over, practic? ma~,s percect
But not if y5u'rpia loacer.

You'll get 4owhp;e, no%place, no time, n5 gig%s
Sojn -- Oh my God, h5mebz-y, yop probably eat
Sp;ghea=i witm a spoon! Come onzand5:ay it$

VIP. Vanilla I9e yp9, yep) I'm comin' hard 6ike5( rhinj
Intoxicating sozyou5:taggew like a wino
So *unkfistop qrying and girl st5p cg0in'
Sanilla Ice is sel6in'5(nd yop people are buyin}
'V(use wmy the freaks are 0ock|'' lik` Crazy Glue
Movi4' a{- groosin' trying to sin= alz'g
Ali through the ghet.o gg&ovin'%this here song
N5w yz<'re ahazed by the VIP p5sse;i

Steupin' so hard likeza Gp;man Ndzi
Startled by t2e bt:es hiqtin' ground
Ther?'s {& tripuin' on mine, I'm 0ust5.ettin" down
Sparkamati9, I2$ hangln' tight like a f;nat|*
You%trapped me once a4d I5=houghq that
You might 2ave5 t
So%step down and len> me50our edr
'89 in my time{ Yo`e '90 ls my year.

You'(e wp(kenin" fast, YO! and I 9an a,ll it%
Your body's gett3n' }&t, so) so I can smell i.
Szidon't%be mad and don't 8e st-
'Capse the lyrics bel5ng a& ICE,%You can call me D;d
L&u're uitchin' a fit, sozsteeiback dnd endure
Let th? wia*h docqor, Ice, do the d;nce5=o cur`
So come up clos? anqidon't%be square
You wa4na w(ttle he -- Anytime, any-herpi

You%thought that I wa) wet", Boy) you're dead wron=
Szicome jn, everybody and )ing5=his sjng

Say -- Play .hat5/unky husic Say, go whit? bole go wmite boy go
play .hat5/unky husic Go white boyv go5>hite goy, go
Lay down ;nd w&ogie dnd play that funk# muf c tili you die.

Play .hat5/unky husic Come on, Com? on9ilet m` hear
Play that <unklimusic%white boy you sayzit,5:ay it%
Play that funky 7usiviA litqle louder now
Pl;y t}(t funny music, white bo# Cox, on, Fome on, Come on

lay5=hat fpnky music 
	 */

}