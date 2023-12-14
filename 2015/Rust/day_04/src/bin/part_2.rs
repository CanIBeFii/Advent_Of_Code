use md5;

fn part_2(input: &str) -> String {
	let mut counter = 0;

	loop {
		let key: String = input.to_owned() + &counter.to_string();

		let result: String = get_md5_hash(&key).chars().take(6).collect();
		if result == "000000" {
			return counter.to_string()
		}
		counter += 1;
	}
}

fn get_md5_hash(key: &str) -> String {
	let hash = md5::compute(key);
	format!("{:x}", hash)
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
    fn part_2_test_1() {
        let input = "abcdef";
        let result = part_2(input);
		let mut key = input.to_string();

		key.push_str(&result);
		let hash = get_md5_hash(&key);

		let first_five: String = hash.chars().take(5).collect();
     
		assert_eq!(first_five, "00000");
    }
}
