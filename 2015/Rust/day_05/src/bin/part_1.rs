fn part_1(input: &str) -> u32 {
	input.to_string().lines()
		.fold(0, |counter, line| {
			if is_valid_str(line) {
				counter + 1
			} else {
				counter
			}
		})
}

fn is_valid_str(line: &str) -> bool {
	let line = line.to_string();
	if line.contains("ab") || line.contains("cd") || line.contains("pq")
		|| line.contains("xy") {
			return false
		}
	let mut last_char: char = '0';
	let mut double_char = false;
	let mut vowel_count = 0;
	line.chars().for_each(|c| {
		match c {
			'a'|'e'|'i'|'o'|'u' => {vowel_count += 1;}
			_ => {}
		}
		if c == last_char && double_char == false {
			double_char = true;
		}
		last_char = c;
	});
	vowel_count >= 3 && double_char
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
    fn is_nice_string() {
        let input = "ugknbfddgicrmopn";
        let result = is_valid_str(input);

		assert_eq!(result, true);
    }

	#[test]
	fn is_naughty_string_no_double_letter() {
		let input = "jchzalrnumimnmhp";
		let result = is_valid_str(input);

		assert_eq!(result, false);
	}

	#[test]
	fn is_naughty_string_forbidden_str() {
		let input = "haegwjzuvuyypxyu";
		let result = is_valid_str(input);

		assert_eq!(result, false);
	}

	#[test]
	fn is_naughty_string_not_3_vowels() {
		let input = "dvszwmarrgswjxmb";
		let result = is_valid_str(input);

		assert_eq!(result, false);
	}
}
