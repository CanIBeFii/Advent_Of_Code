fn main() {
    let input = include_str!("./input.txt");
	let result = part_2(input.to_string());
	dbg!(result);
}

fn part_2(input : String ) -> i32 {
	let mut iterator = input.chars();

	let mut value = iterator.next();
	let mut count = 0;
	let mut position = 0;
	while value != None {
		let c = value.unwrap();
		position += 1;

		if c == '(' {
			count += 1;
		} else {
			count -= 1;
		}

		if count == -1 {
			return position
		}
		value = iterator.next();
	}
	count
}

#[cfg(test)]
mod tests {
	use super::part_2;

    #[test]
    fn it_works() {
        let input = ")())())";
		let result = part_2(input.to_string());
        assert_eq!(result, 1);
    }
}