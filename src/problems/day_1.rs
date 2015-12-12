use problems::utils;

/// Count parenthesis from input
pub fn day_1() {

	let s = utils::read_file("inputs/day1.txt");
	let mut counter = 0;
	let mut count: i32 = 0;
	let mut trigger = true;
	let mut first_basement = 0;

	for c in s.chars() {

		match c {
			'(' => count += 1,
			')' => count -= 1,
			_ => panic!(),
		}
		counter += 1;
		if trigger && count < 0 {
			first_basement = counter;
			trigger = false;
		}
	}
    println!("\nFinal count: {}. Reached the basement on character {}", count, first_basement);
}

