fn part_2(input: &str) -> u32 {
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
	let repeat_letter = line.as_bytes().windows(3).any(|trio| trio[0] == trio[2]);
	
	if !repeat_letter {
		return false
	}

	let mut index = 0;
	while index + 3 < line.len() {
		let mut index2 = index + 2;
		while index2 + 1 < line.len() {
			if line[index..=index + 1] == line[index2..=index2 + 1] {
				return true
			} else {
				index2 += 1;
			}
		}
		index += 1;
	}
	false
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
    fn is_nice_string() {
        let input = "qjhvhtzxzqqjkmpb";
        let result = is_valid_str(input);

		assert_eq!(result, true);
    }

	#[test]
	fn is_nice_string_2() {
		let input = "xxyxx";
		let result = is_valid_str(input);

		assert_eq!(result, true);
	}

	#[test]
	fn no_repeat_with_single_letter_between() {
		let input = "uurcxstgmygtbstg";
		let result = is_valid_str(input);

		assert_eq!(result, false);
	}

	#[test]
	fn no_twice_pair() {
		let input = "ieodomkazucvgmuy";
		let result = is_valid_str(input);

		assert_eq!(result, false);
	}
}