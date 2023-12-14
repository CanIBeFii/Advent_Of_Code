const ZERO: u8 = 48;

fn part_1(input: &str) -> String {
	let mut output = String::from(input);
	for _n in 1..=50 {
		output = look_and_say(&output);
	}
	output
}

fn look_and_say(input: &str) -> String {
	let mut output = String::new();
	let input = input.as_bytes();
	let mut last_char: u8 = input[0];
	let mut index = 0;
	let mut count: u8 = ZERO; // 48 is 1 in a u8

	while index < input.len() {
		if input[index] != last_char {
			output.push(count as char);
			output.push(last_char as char);
			last_char = input[index];
			count = ZERO; // 48 is 0 in a u8
		}
		count += 1;
		index += 1;
	}
	output.push(count as char);
	output.push(last_char as char);
	output
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part_1(input);
    dbg!(result.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_1() {
        let input = "1";
        let result = look_and_say(input);

        assert_eq!(result, "11");
    }

	#[test]
	fn number_11() {
		let input = "11";
		let result = look_and_say(input);

		assert_eq!(result, "21");
	}

	#[test]
	fn number_21() {
		let input = "21";
		let result = look_and_say(input);

		assert_eq!(result, "1211");
	}

	#[test]
	fn number_1211() {
		let input = "1211";
		let result = look_and_say(input);

		assert_eq!(result, "111221");
	}

	#[test]
	fn number_111221() {
		let input = "111221";
		let result = look_and_say(input);

		assert_eq!(result, "312211");
	}
}
