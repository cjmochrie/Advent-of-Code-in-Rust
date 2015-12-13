use regex::Regex;
use regex::Captures;
use problems::utils;

pub fn day_5() {
	println!("Called day_5!");

	let s = utils::read_file("inputs/day5.txt");
 	let strings: Vec<&str> = s.split("\n").collect();

 	let vowel = Regex::new(r".*(a|e|i|o|u){1}.*(a|e|i|o|u){1}.*(a|e|i|o|u){1}.*").unwrap();
 	let special_pairs = Regex::new(r"(ab|cd|pq|xy)").unwrap();

 	let mut count = 0;
 	for string in strings.iter() {
 		if vowel.is_match(&string) {
 			if contains_duplicates(&string) {
 				if !special_pairs.is_match(&string) {
 					count += 1;
 				}
 			}
 		}

 	}

 	println!("{} safe strings for Part 1!", count);
 	part_2();
}

fn part_2() {

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