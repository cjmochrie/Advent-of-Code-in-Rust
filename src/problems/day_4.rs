
use crypto::digest::Digest;
use crypto::md5::Md5;

use problems::utils;

pub fn day_4() {
	println!("Called day_4!");

	let mut number = 0;
	let input = "bgvyzdsv";

	// Continually md5 hash the input string concatenated with a number
	// Break once the hash has 5 or 6 leading zeros and print the
	// test input that did it
	loop {
		let mut sh = Md5::new();
		let test = input.to_string() + &number.to_string();
		sh.input_str(&test);
		let result: &str = &sh.result_str();

		if &result[0..6] == "000000" {
			println!("The solution is {}", test);
			break;
		}
		number += 1;
	}
}