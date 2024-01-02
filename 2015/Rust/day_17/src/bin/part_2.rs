use permutator::LargeCombinationIterator;
use std::collections::HashMap;

fn part_2(input: &str) -> i32 {
	let containers: Vec<u32> = input.lines().map(|line| line.parse::<u32>().unwrap()).collect();

	let mut result = HashMap::new();
	// println!("container size {}", containers.len());
	for size in 1..=containers.len() {
		let combinations = LargeCombinationIterator::new(&containers, size);
		for x in combinations {
			let value: u32 = x.iter().copied().sum();
			// println!("{value} is the value");
			if value == 150 {
				let idk_variable_names = result.entry(size).or_insert(0);
				*idk_variable_names += 1;
			}
		}
	}
	let minimum_containers = *result.keys().min().unwrap();
	*result.get(&minimum_containers).unwrap()
 }

fn main() {
    let input = include_str!("./input.txt");
    let result = part_2(input);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
		let input = "20
15
10
5
5";
		let result = part_2(input);

		assert_eq!(result, 3);
	}
}
