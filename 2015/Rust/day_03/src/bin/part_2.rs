use std::collections::HashMap;

fn update_coords(coords: &mut (i32, i32), key: char) {
	match key {
		'>' => {coords.0 += 1;}
		'<' => {coords.0 -= 1;}
		'^' => {coords.1 += 1;}
		'v' => {coords.1 -= 1;}
		_ => {unreachable!();}
		
	}
}

fn part_2(input: &str) -> usize {
	let mut houses : HashMap<(i32, i32), i32> = HashMap::new();
	let mut coords_santa = (0, 0);
	let mut coords_robot = (0, 0);
	let mut index = 0;

	houses.insert(coords_santa, 1);
	input.to_string().chars()
		.for_each(|x| {
			if index % 2 == 0 {
				update_coords(&mut coords_santa, x);
				let house = houses.entry(coords_santa).or_insert(0);
				*house += 1;
			} else {
				update_coords(&mut coords_robot, x);
				let house = houses.entry(coords_robot).or_insert(0);
				*house += 1;
			}
			index += 1;
		});
	houses.into_keys().count()
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part_2(input);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::part_2;

    #[test]
    fn part_2_houses_3() {
        let input = "^v";
        let result = part_2(input);
        assert_eq!(result, 3);
    }

	#[test]
	fn part_2_houses_3_same_pos() {
		let input = "^>v<";
        let result = part_2(input);
        assert_eq!(result, 3);
	}

	#[test]
	fn part_2_houses_11() {
		let input = "^v^v^v^v^v";
        let result = part_2(input);
        assert_eq!(result, 11);
	}
}