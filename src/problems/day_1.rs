use problems::utils;

/// Count parenthesis from input
pub fn day_1() {

	let s = utils::read_file("inputs/day1.txt");

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

