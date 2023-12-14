fn part_2(input: &str) -> i32 {
	input.lines()
		.fold(0, |count, line| {
			let literal_string_length = line.len() as i32;
			let encoded_string = encode_string_literal(line);
			let encoded_string_length = encoded_string.len() as i32;

			println!("'{line}' encoded is {encoded_string} len is '{encoded_string_length}' and default size '{literal_string_length}'");

			count - literal_string_length + encoded_string_length
		})
}

fn encode_string_literal(line: &str) -> String {
	let mut index = 0;
	let line = line.as_bytes();
	let mut ouput = String::new();
	ouput.push('\"');
	while index < line.len() {
		if line[index] == b'\\' {
			ouput.push_str("\\\\");
		} else if line[index] == b'\"' {
			ouput.push_str("\\\"");
		} else {
			ouput.push(line[index] as char);
		}
		index += 1;
	}
	ouput.push('\"');
	ouput
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
    fn part_2_works() {
        let input = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";
        let result = part_2(input);

        assert_eq!(result, 19);
    }
}
