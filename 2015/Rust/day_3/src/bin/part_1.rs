use std::collections::HashMap;

fn part_1(input: &str) -> usize {
	let mut houses : HashMap<(i32, i32), i32> = HashMap::new();
	let mut coords = (0,0);

	houses.insert(coords, 1);
	input.to_string().chars()
		.for_each(|x| {
			match x {
				'<' => {
					coords.0 -= 1;
					let house = houses.entry(coords).or_insert(0);
					*house += 1; 
				}
				'>' => {
					coords.0 += 1;
					let house = houses.entry(coords).or_insert(0);
					*house += 1;
				}
				'^' => {
					coords.1 += 1;
					let house = houses.entry(coords).or_insert(0);
					*house += 1;
				}
				'v' => {
					coords.1 -= 1;
					let house = houses.entry(coords).or_insert(0);
					*house += 1;
				}
				_ => {unreachable!();}
			}
		});
	houses.into_keys().count()
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part_1(input);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::part_1;

    #[test]
    fn part_1_works() {
        let input = "^>v<";
        let result = part_1(input);
        assert_eq!(result, 4);
    }
}