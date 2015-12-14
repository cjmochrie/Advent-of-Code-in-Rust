use std::cmp;
use regex::Regex;
use problems::utils;

const GRID_SIZE: usize = 1000;

#[derive(Debug)]
enum Operation {
	TurnOn,
	TurnOff,
	Toggle
}

#[derive(Debug)]
struct Instruction {
	op: Operation,
	start_x: i32,
	start_y: i32,
	end_x: i32,
	end_y: i32,
}

pub fn day_6() {
	println!("Called day_6()");

	let s = utils::read_file("inputs/day6.txt");

	let instructions: Vec<Instruction> = parse_instructions(&s);


	let mut grid = [[false; GRID_SIZE]; GRID_SIZE];

	for instruction in &instructions {
		update_grid(&mut grid, instruction);
	}


	println!("There are {} lights on", count_lights(&grid));

	let mut brightness_grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
	for instruction in &instructions {
		update_grid_brightness(&mut brightness_grid, instruction);
	}

	println!("Total brightness is {}", measure_brightness(&brightness_grid));

}

fn count_lights(grid: &[[bool; GRID_SIZE]; GRID_SIZE]) -> i32{
	let mut count = 0;
	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			if grid[x as usize][y as usize] == true { count += 1; }
		}
	}
	count
}

fn measure_brightness(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> i32 {
	let mut brightness: i32 = 0;
	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			brightness += grid[x as usize][y as usize] as i32;
		}
	}
	brightness
}

fn update_grid_brightness(grid: &mut[[u8; GRID_SIZE]; GRID_SIZE], instruction: &Instruction) {
	for x in instruction.start_x..instruction.end_x {
		for y in instruction.start_y..instruction.end_y {
			match instruction.op {
				Operation::TurnOn => grid[x as usize][y as usize] += 1,
				Operation::TurnOff => if grid[x as usize][y as usize] != 0 {
					grid[x as usize][y as usize] -= 1
				},
				Operation::Toggle => grid[x as usize][y as usize] += 2,
			}
		}
	}
}

fn update_grid(grid: &mut[[bool; GRID_SIZE]; GRID_SIZE], instruction: &Instruction) {

	for x in instruction.start_x..instruction.end_x {
		for y in instruction.start_y..instruction.end_y {
			match instruction.op {
				Operation::TurnOn => grid[x as usize][y as usize] = true,
				Operation::TurnOff => grid[x as usize][y as usize] = false,
				Operation::Toggle => grid[x as usize][y as usize] = !grid[x as usize][y as usize],
			}
		}
	}
}

fn parse_instructions(s: &str) -> Vec<Instruction> {

	let raw_instructions: Vec<&str> = s.split("\n").collect();
	let re = Regex::new(r"(?P<op>\w+.\w*) (?P<start_x>\d+),(?P<start_y>\d+) \w+ (?P<end_x>\d+),(?P<end_y>\d+)").unwrap();

	let mut instructions: Vec<Instruction> = Vec::new();

	for raw_inst in &raw_instructions {
		if raw_inst.len() < 1 {
			continue;
		}
		let cap = re.captures(&raw_inst).unwrap();
		let op = match cap.name("op").unwrap() {
			"toggle" => Operation::Toggle,
			"turn on" => Operation::TurnOn,
			"turn off" => Operation::TurnOff,
			_ => panic!(),
		};

		// add one for easier range addressing (exclusive at the end)
		let instruction = Instruction {op: op,
									   start_x: cap.name("start_x").unwrap().parse::<i32>().unwrap(),
									   start_y: cap.name("start_y").unwrap().parse::<i32>().unwrap(),
									   end_x: cap.name("end_x").unwrap().parse::<i32>().unwrap() + 1,
									   end_y: cap.name("end_y").unwrap().parse::<i32>().unwrap() + 1};
		instructions.push(instruction);
	}

	instructions
}

