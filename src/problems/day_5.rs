use regex::Regex;
use problems::utils;

pub fn day_5() {
	println!("Called day_5!");

	let s = utils::read_file("inputs/day5.txt");
 	let strings: Vec<&str> = s.split("\n").collect();

 	let vowel = Regex::new(r".*(a|e|i|o|u){1}.*(a|e|i|o|u){1}.*(a|e|i|o|u){1}.*").unwrap();
 	let special_pairs = Regex::new(r"(ab|cd|pq|xy)").unwrap();

 	let mut count = 0;
 	for string in &strings {
 		if vowel.is_match(&string) {
 			if contains_duplicates(&string) {
 				if !special_pairs.is_match(&string) {
 					count += 1;
 				}
 			}
 		}

 	}

 	println!("{} safe strings for Part 1!", count);
 	part_2(strings);
}

fn part_2(strings: Vec<&str>) {
	println!("Called Part 2!");
	let mut count = 0;
	for string in &strings {
		if contains_pairs(&string) {

			if contains_repeat_with_separator(&string) {
				count += 1;
			}
		}
	}
	println!("{} safe strings for Part 2", count);
}

fn contains_duplicates(string: &str) -> bool {
	let mut prev_char = 'Z';
	for char in string.chars() {
		if prev_char == char {
			return true
		}
		prev_char = char;
	}
	false
}

fn contains_pairs(string: &str) -> bool {

	let length = string.len();
	if length < 1 { return false; }

	let chars: Vec<char> = string.chars().collect();

	// Loop through n - 2 positions in the char array
	// Temporarily store 2 chars (a pair)
	// Check remainder of the array for a repeat, if found return true
	for i in 1..(length - 2) {
		let char1 = chars[i - 1];
		let char2 = chars[i];
		for j in (i + 2)..length {
			if chars[j - 1] == char1 {
				if chars[j] == char2 {
					return true
				}
			}
		}
	}
	return false;
}

fn contains_repeat_with_separator(string: &str) -> bool {
	let length = string.len();
	if length < 1 { return false; }

	let chars: Vec<char> = string.chars().collect();

	for i in 0..(length - 2) {
		let char1 = chars[i];
		if char1 == chars[i + 2] {
			return true
		}
	}
	return false;
}