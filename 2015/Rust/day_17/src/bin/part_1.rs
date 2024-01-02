use permutator::LargeCombinationIterator;

fn part_1(input: &str) -> u32 {
	let containers: Vec<u32> = input.lines().map(|line| line.parse::<u32>().unwrap()).collect();

	let mut result = 0;
	println!("container size {}", containers.len());
	for size in 1..=containers.len() {
		let combinations = LargeCombinationIterator::new(&containers, size);
		for x in combinations {
			let value: u32 = x.iter().copied().sum();
			println!("{value} is the value");
			if value == 25 {
				result += 1;
			}
		}
	}
	result
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part_1(input);
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
		let result = part_1(input);

		assert_eq!(result, 4);
	}
}
