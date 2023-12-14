fn part_1(input: &str) -> i32 {
	input.lines()
		.fold(0, |count, line| {
			let literal_string_length = line.len() as i32;
			let decoded_string_length = decode_from_in_memory(line).len() as i32;
			count + literal_string_length - decoded_string_length
		})
}

fn decode_from_in_memory(line: &str) -> String {
	let mut index = 1;
	let line = line.as_bytes();
	let mut output = String::new();
	while index < line.len() - 1 {
		if line[index] == b'\\' {
			if line[index + 1] == b'\"' {
				output.push('\"');
			} else if line[index + 1] == b'\\' {
				output.push('\\')
			} else if line[index + 1] == b'x' {
				output.push('#');
				index += 2;
			}
			index += 1;
		} else {
			output.push(line[index] as char);
		}
		index += 1;
	}
	output
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
    fn part_1_works() {
        let input = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";
        let result = part_1(input);

        assert_eq!(result, 12);
    }
}
