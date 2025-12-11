use std::io::{self, BufRead};

struct Machine {
	light_target: u32,
	light_ct: usize,
	buttons: Vec<u32>,
}

impl Machine {
	fn fewest(&self) -> usize {
		let mut ret = usize::MAX;
		for i in 0..(1 << self.buttons.len()) {
			let mut light_state = 0;
			let mut num_buttons = 0;
			for j in 0..self.buttons.len() {
				if i & (1 << j) != 0 {
					num_buttons += 1;
					light_state ^= self.buttons[j];
				}
			}
			if light_state == self.light_target {
				ret = ret.min(num_buttons);
			}
		}
		ret
	}
}

impl TryFrom<&str> for Machine {
	type Error = &'static str;
	fn try_from(input: &str) -> Result<Self, Self::Error> {
		let lights_end = input.find(']').ok_or("Could not find ']'")?;
		let joltage_start = input.find('{').ok_or("Could not find '{'")?;
		let light_target = (&input[1..lights_end])
			.chars()
			.enumerate()
			.fold(0, |acc, (idx, c)| acc | ((c == '#') as u32) << idx);
		let buttons: Vec<u32> = input[lights_end + 1..joltage_start]
			.trim()
			.split(" ")
			.map(|s| {
				s[1..s.len() - 1].split(',').fold(0, |acc, v| {
					acc | 1 << v.parse::<u32>().expect("Could not parse int")
				})
			})
			.collect();
		Ok(Machine {
			light_target,
			buttons,
			light_ct: lights_end - 1,
		})
	}
}

fn main() {
	let machines: Vec<Machine> = io::stdin()
		.lock()
		.lines()
		.into_iter()
		.map(|line| {
			Machine::try_from(line.expect("Could not read line").as_str())
				.expect("Could not parse line")
		})
		.collect();
	println!(
		"{}",
		machines
			.iter()
			.fold(0, |acc, machine| machine.fewest() + acc)
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn machine_test() {
		let machine =
			Machine::try_from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
		assert_eq!(
			machine.buttons,
			vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011]
		);
		assert_eq!(machine.light_target, 0b0110);
		assert_eq!(machine.light_ct, 4);
		assert_eq!(machine.fewest(), 2);

		let machine =
			Machine::try_from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
				.unwrap();
		assert_eq!(
			machine.buttons,
			vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110]
		);
		assert_eq!(machine.light_target, 0b01000);
		assert_eq!(machine.light_ct, 5);
		assert_eq!(machine.fewest(), 3);

		let machine =
			Machine::try_from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
				.unwrap();
		assert_eq!(
			machine.buttons,
			vec![0b011111, 0b011001, 0b110111, 0b000110]
		);
		assert_eq!(machine.light_target, 0b101110);
		assert_eq!(machine.light_ct, 6);
		assert_eq!(machine.fewest(), 2);
	}
}
