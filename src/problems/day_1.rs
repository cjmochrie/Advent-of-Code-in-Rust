use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// File opening from http://rustbyexample.com/std_misc/file/open.html

/// Count parenthesis from input
pub fn day_1() {
	println!("here");
	// Create a new path to the desired file
	let path = Path::new("../inputs/day1.txt");
	let display = path.display();

	// Open the path in read only mode, returns 'io::Result<File>'
	let mut file = match File::open(&path) {
		// The 'description' method of 'io::Error' returns a string
		// that describes the error
		Err(why) => panic!("couldn't open {}: {}", display,
			Error::description(&why)),
		Ok(file) => file,
	};

	// Read the file contents into a string, returns `io::Result<usize>`
	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
		Ok(_) => print!("Read ok"),
	}

	let mut count: i32 = 0;
	for c in s.chars() {

		match c {
			'(' => count += 1,
			')' => count -= 1,
			_ => panic!(),
		}
	}
    println!("\nFinal count: {}", count);
}