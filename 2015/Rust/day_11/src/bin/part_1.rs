use itertools::Itertools;
use std::str;

const PASS_SIZE: usize = 8;

fn part_1(input: &str) -> String {
	let mut pass = Vec::from(input.as_bytes());
	loop {
		let output = String::from_utf8(pass.clone()).unwrap(); // We know the bytes will always be valid chars
		if is_valid_password(&output) {
			return output
		}
		next_pass(&mut pass, PASS_SIZE - 1)
	}
}

fn next_pass(pass: &mut [u8], index: usize) {
	if pass[index] == b'z' {
		pass[index] = b'a';
		next_pass(pass, index - 1)
	} else {
		pass[index] += 1;
	}
}

fn is_valid_password(input: &str) -> bool {
	if input.contains('i') || input.contains('o') || input.contains('l') {
		return false
	}
	input.as_bytes().windows(2)
		.filter(|x| x[0] == x[1]).unique().count() >= 2 &&
		input.as_bytes().windows(3).any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2])
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
    fn fail_contains_forbidden_letters() {
        let input = "hijklmmn";
        let result = is_valid_password(input);

        assert!(!result);
    }

	#[test]
	fn fail_no_3_sequencial_letters() {
		let input = "abbceffg";
		let result = is_valid_password(input);

		assert!(!result);
	}

	#[test]
	fn fail_no_double_double_letters() {
		let input = "abbcegjk";
		let result = is_valid_password(input);

		assert!(!result);
	}

	#[test]
	fn transform_abcdefgh() {
		let input = "abcdefgh";
		let result = part_1(input);

		assert_eq!(result, "abcdffaa");
	}

	#[test]
	fn transform_ghijklmn(){
		let input = "ghijklmn";
		let result = part_1(input);

		assert_eq!(result, "ghjaabcc");
	}
}
