use std::io::{self, BufRead, Read};
fn main() {
	let mut data: Vec<Vec<u8>> = Vec::new();
	for line in io::stdin().lock().lines() {
		data.push(line.expect("Could not read input").into_bytes())
	}
	let mut ret = 0;
	while let ct = reduce(&mut data) && ct > 0 {
		ret += ct;
	}
	println!("{}", ret);
}

fn get_counts(data: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let mut counts: Vec<Vec<u8>> = vec![vec![0;data[0].len()]; data.len()];
	for i in 0..data.len() {
		for j in 0..data[0].len() {
			if data[i][j] == b'@' {
				for di in [-1isize, 0, 1] {
					for dj in [-1isize, 0, 1] {
						if di == 0 && dj == 0 {
							continue;
						}
						if let Some(row) = counts.get_mut((i as isize + di) as usize)
							&& let Some(v) = row.get_mut((j as isize + dj) as usize) 
							&& *v < 4
						{
								*v+=1;
						}
					}
				}
			} else { // make impossible if not @
				counts[i][j] = 4;
			}
		}
	}
	counts
}

fn reduce(data: &mut Vec<Vec<u8>>) -> usize {
	let counts = get_counts(&data);
	let mut ret = 0;
	for i in 0..counts.len() {
		for j in 0..counts[0].len() {
			if counts[i][j] < 4 {
				ret += 1;
				data[i][j] = b'.';
			}
		}
	}
	ret
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn get_counts_test() {
		assert_eq!(get_counts(
			&vec![
				vec![b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'@',b'.'],
				vec![b'@',b'@',b'@',b'.',b'@',b'.',b'@',b'.',b'@',b'@'],
				vec![b'@',b'@',b'@',b'@',b'@',b'.',b'@',b'.',b'@',b'@'],
				vec![b'@',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'@',b'.'],
				vec![b'@',b'@',b'.',b'@',b'@',b'@',b'@',b'.',b'@',b'@'],
				vec![b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'.',b'@'],
				vec![b'.',b'@',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'@'],
				vec![b'@',b'.',b'@',b'@',b'@',b'.',b'@',b'@',b'@',b'@'],
				vec![b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'.'],
				vec![b'@',b'.',b'@',b'.',b'@',b'@',b'@',b'.',b'@',b'.'],
			]
		), 
			vec![
				vec![4, 4, 3, 3, 4, 3, 3, 4, 3, 4], 
				vec![3, 4, 4, 4, 4, 4, 4, 4, 4, 4], 
				vec![4, 4, 4, 4, 4, 4, 2, 4, 4, 4], 
				vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4], 
				vec![3, 4, 4, 4, 4, 4, 4, 4, 4, 3], 
				vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4], 
				vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4], 
				vec![2, 4, 4, 4, 4, 4, 4, 4, 4, 4], 
				vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4], 
				vec![1, 4, 3, 4, 4, 4, 4, 4, 2, 4]
			]
		)
	}

	#[test]
	fn reduce_test() {
		let mut input = vec![
			vec![b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'@',b'.'],
			vec![b'@',b'@',b'@',b'.',b'@',b'.',b'@',b'.',b'@',b'@'],
			vec![b'@',b'@',b'@',b'@',b'@',b'.',b'@',b'.',b'@',b'@'],
			vec![b'@',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'@',b'.'],
			vec![b'@',b'@',b'.',b'@',b'@',b'@',b'@',b'.',b'@',b'@'],
			vec![b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'.',b'@'],
			vec![b'.',b'@',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'@'],
			vec![b'@',b'.',b'@',b'@',b'@',b'.',b'@',b'@',b'@',b'@'],
			vec![b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'.'],
			vec![b'@',b'.',b'@',b'.',b'@',b'@',b'@',b'.',b'@',b'.'],
		];

		let ct = reduce(&mut input);
		assert_eq!(ct, 13);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'@',b'.',b'.'],
			vec![b'.',b'@',b'@',b'.',b'@',b'.',b'@',b'.',b'@',b'@'],
			vec![b'@',b'@',b'@',b'@',b'@',b'.',b'.',b'.',b'@',b'@'],
			vec![b'@',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'@',b'.'],
			vec![b'.',b'@',b'.',b'@',b'@',b'@',b'@',b'.',b'@',b'.'],
			vec![b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'.',b'@'],
			vec![b'.',b'@',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'@'],
			vec![b'.',b'.',b'@',b'@',b'@',b'.',b'@',b'@',b'@',b'@'],
			vec![b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 12);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'@',b'@',b'.',b'.',b'.',b'.',b'.',b'@',b'.'],
			vec![b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.',b'@',b'@'],
			vec![b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'@',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'@'],
			vec![b'.',b'.',b'@',b'@',b'@',b'.',b'@',b'@',b'@',b'@'],
			vec![b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 7);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'@',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'@',b'@',b'@',b'.',b'@',b'@',b'@',b'@'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 5);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 2);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 1);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 1);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 1);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 1);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);

		let ct = reduce(&mut input);
		assert_eq!(ct, 0);
		assert_eq!(input, vec![
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'.',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'.',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'@',b'.',b'@',b'.',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'.',b'@',b'@',b'@',b'.'],
			vec![b'.',b'.',b'.',b'@',b'@',b'@',b'@',b'@',b'.',b'.'],
			vec![b'.',b'.',b'.',b'.',b'@',b'@',b'@',b'.',b'.',b'.'],
		]);
	}
}
