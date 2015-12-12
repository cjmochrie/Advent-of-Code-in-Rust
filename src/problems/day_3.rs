use std::collections::BTreeSet;
use std::cmp::Ordering;

use problems::utils;

struct Point {
	x: i32,
	y: i32,
}

impl Ord for Point {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.x > other.x {
			Ordering::Greater
		}
		else if self.x < other.x {
			Ordering::Less
		}
		else if self.y > other.y {
			Ordering::Greater
		}
		else if self.y < other.y {
			Ordering::Less
		} else {
			Ordering::Equal
		}
	}
}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		(self.x, &self.y) == (other.x, &other.y)
	}
}

impl Eq for Point { }

pub fn day_3() {
	let s = utils::read_file("inputs/day3.txt");

	let mut set: BTreeSet<Point> = BTreeSet::new();
	let mut current_x = 0;
	let mut current_y = 0;

	let mut marker = false;

	set.insert(Point{x: current_x, y: current_y});
	for c in s.chars() {
		if marker {
			match c {
				'v' => current_y -= 1,
				'^' => current_y += 1,
				'<' => current_x -= 1,
				'>' => current_x += 1,
				_ => println!("Error parsing input"),
			}
			set.insert(Point{x: current_x, y: current_y});
			marker = false;
		} else {
			marker = true;
		}
	}
	marker = true;
	current_x = 0;
	current_y = 0;
	for c in s.chars() {
		if marker {
			match c {
				'v' => current_y -= 1,
				'^' => current_y += 1,
				'<' => current_x -= 1,
				'>' => current_x += 1,
				_ => println!("Error parsing input"),
			}
			set.insert(Point{x: current_x, y: current_y});
			marker = false;
		} else {
			marker = true;
		}
	}


	println!("Santa and the robot have visited {} locations", set.len());
}