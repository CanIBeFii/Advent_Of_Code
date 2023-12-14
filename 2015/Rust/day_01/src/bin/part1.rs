fn main() {
    let input = include_str!("./input.txt");
	let result = part_1(input.to_string());
	dbg!(result);
}

fn part_1(input : String ) -> i32 {
	input.chars()
		.fold(0, |count, x| {
			if x == '(' {
				count + 1
			} else {
				count - 1
			}
		})
}

#[cfg(test)]
mod tests {
	use super::part_1;

    #[test]
    fn it_works() {
        let input = ")())())";
		let result = part_1(input.to_string());
        assert_eq!(result, 1);
    }
}