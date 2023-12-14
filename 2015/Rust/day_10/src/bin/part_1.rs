fn part_1(input: &str) -> i32 {
	
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
    fn number_1() {
        let input = "1";
        let result = part_1(input);

        assert_eq!(result, "11");
    }

	#[test]
	fn number_11() {
		let input = "11";
		let result = part_1(input);

		assert_eq!(result, "21");
	}

	#[test]
	fn number_21() {
		let input = "21";
		let result = part_1(input);

		assert_eq!(result, "1211");
	}

	#[test]
	fn number_1211() {
		let input = "1211";
		let result = part_1(input);

		assert_eq!(result, "111221");
	}

	#[test]
	fn number_111221() {
		let input = "111221";
		let result = part_1(input);

		assert_eq!(result, "312211");
	}
}
