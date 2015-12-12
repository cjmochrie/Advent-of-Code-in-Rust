use std::cmp;
use std::num::ParseIntError;

use problems::utils;

struct Package {
	length: i32,
	width: i32,
	height: i32,
}

pub fn day_2() {
 	let s = utils::read_file("inputs/day2.txt");
 	let packages: Vec<&str> = s.split("\n").collect();

 	let mut parsed_packages: Vec<Package> = Vec::new();
 	for package in packages.iter() {
 		match parse_package(package) {
 			Err(err) => println!("Error parsing: {}", err),
 			Ok(result) => parsed_packages.push(result),
 		}
 	}
 	let mut total_area = 0;
 	for package in parsed_packages.iter() {
 		total_area += calculate_area(&package);
 	}
 	println!("Total area required: {}", total_area);

 	let mut total_feet = 0;
 	for package in parsed_packages.iter() {
 		total_feet += calculate_feet(&package);
 	}
 	println!("Total feet required: {}", total_feet);
}

fn parse_package(string_package: &str) -> Result<Package, ParseIntError> {
	let string_dims: Vec<&str> = string_package.split("x").collect();

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
	Ok(Package { length: length, width: width, height: height})
}


fn calculate_area(package: &Package) -> i32 {

	let side1 = package.length * package.width;
	let side2 = package.length * package.height;
	let side3 = package.width * package.height;

	let slack = cmp::min(cmp::min(side1, side2), side3);
	2 * (side1 + side2 + side3) + slack
}


fn calculate_feet(package: &Package) -> i32 {
	let mut dims:Vec<i32> = vec![package.length, package.width, package.height];

	let volume: i32 = dims.iter().fold(1, |volume, x| volume * x);

	dims.sort();

	let shortest_dist = (dims[0] + dims[1]) * 2;

	shortest_dist + volume
}