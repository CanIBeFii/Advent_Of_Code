use std::collections::HashMap;
use regex::Regex;

#[derive(Hash, Eq, PartialEq)]
enum SueTypeInfo {
	Children,
	Cats,
	Samoyeds,
	Pomeranians,
	Akitas,
	Vizslas,
	Goldfish,
	Trees,
	Cars,
	Perfumes,
}

fn str_to_sue_type_info(field: &str) -> SueTypeInfo {
	match field {
		"children" => SueTypeInfo::Children,
		"cats" => SueTypeInfo::Cats,
		"samoyeds" => SueTypeInfo::Samoyeds,
		"pomeranians" => SueTypeInfo::Pomeranians,
		"akitas" => SueTypeInfo::Akitas,
		"vizslas" => SueTypeInfo::Vizslas,
		"goldfish" => SueTypeInfo::Goldfish,
		"trees" => SueTypeInfo::Trees,
		"cars" => SueTypeInfo::Cars,
		"perfumes" => SueTypeInfo::Perfumes,
		_ => unreachable!(),

	}
}

fn ticker_tape_info(line: &str) -> (SueTypeInfo, u8) {
	let re = Regex::new(r"(\w+): (\d+)").unwrap();
	let captures = re.captures(line).unwrap();

	let info_type = str_to_sue_type_info(&captures[1]);
	let quantity = captures[2].parse().unwrap();
	(info_type, quantity)
}

fn part_1(input: &str) -> usize {
	let mut infos_quantities = HashMap::new();
	let ticker_tape = include_str!("./ticker_tape.txt");

	ticker_tape
		.lines()
		.for_each(|x| {
			let field = ticker_tape_info(x);
			infos_quantities.insert(field.0, field.1);
		});
	
	let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
	let input = input.lines().enumerate();	
	for (index, line) in input {
		let captures = re.captures(line).unwrap();
		let info: (SueTypeInfo, u8) = (str_to_sue_type_info(&captures[2]), captures[3].parse().unwrap());
		if info.1 != infos_quantities[&info.0] {
			continue;
		}
		let info: (SueTypeInfo, u8) = (str_to_sue_type_info(&captures[4]), captures[5].parse().unwrap());
		if info.1 != infos_quantities[&info.0] {
			continue;
		}
		let info: (SueTypeInfo, u8) = (str_to_sue_type_info(&captures[6]), captures[7].parse().unwrap());
		if info.1 != infos_quantities[&info.0] {
			continue;
		}
		return index + 1
	}
	0
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
		let input = "Sue 1: goldfish: 6, trees: 9, akitas: 0
Sue 2: goldfish: 7, trees: 1, akitas: 0
Sue 3: cars: 10, akitas: 6, perfumes: 7
Sue 4: perfumes: 2, vizslas: 0, cars: 6
Sue 5: goldfish: 1, trees: 3, perfumes: 10
Sue 6: children: 9, vizslas: 7, cars: 9
Sue 7: cars: 6, vizslas: 5, cats: 3
Sue 8: akitas: 10, vizslas: 9, children: 3
Sue 9: vizslas: 0, cats: 7, trees: 3
Sue 10: perfumes: 10, trees: 6, cars: 4";
		let result = part_1(input);

		assert_eq!(result, 9);
	}
}
