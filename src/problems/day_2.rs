use std::cmp;
use std::num::ParseIntError;

use problems::utils;

pub fn day_2() {
 	let s = utils::read_file("inputs/day2.txt");
 	let packages: Vec<&str> = s.split("\n").collect();

 	let mut total_area = 0;
 	for package in packages.iter() {
 		match calculate_area(package) {
 			Err(err) => println!("Error parsing: {}", err),
 			Ok(result) => total_area += result,
 		}
 	}

 	println!("Total area required: {}", total_area);

}

fn calculate_area(package: &str) -> Result<i32, ParseIntError> {
	let string_dims: Vec<&str> = package.split("x").collect();

	let length = match string_dims[0].parse::<i32>() {
		Ok(n) => n,
		Err(err) => return Err(err),
	};

	let width = match string_dims[1].parse::<i32>() {
		Ok(n) => n,
		Err(err) => return Err(err),
	};
	let height = match string_dims[2].parse::<i32>() {
		Ok(n) => n,
		Err(err) => return Err(err),
	};

	let side1 = length * width;
	let side2 = length * height;
	let side3 = width * height;

	let slack = cmp::min(cmp::min(side1, side2), side3);
	Ok(2 * (side1 + side2 + side3) + slack)
}
