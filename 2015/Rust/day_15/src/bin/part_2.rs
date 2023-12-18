use regex::Regex;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn get_ingredient(input: &str) -> Ingredient {
    let words = Regex::new(
        r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap();

	let capture = words.captures(input).unwrap();

    Ingredient {
        capacity: capture[2].parse().unwrap(),
        durability: capture[3].parse().unwrap(),
        flavor: capture[4].parse().unwrap(),
        texture: capture[5].parse().unwrap(),
        calories: capture[6].parse().unwrap(),
    }
}

fn part_2(input: &str) -> i32 { //This is a brute force method
    let mut ingredients: Vec<Ingredient> = Vec::new();

    input
        .lines()
        .for_each(|x| ingredients.push(get_ingredient(x)));
    
	let mut biggest_score = 0;
	// There's only 4 possible ingredients
	for a in 0..=100 {
		for b in 0..=100 {
			for c in 0..=100 {
				if a + b + c > 100 {
					continue ;
				}

				let d = 100 - (a + b + c);
				let quantities = [a, b, c, d];

				let calories: i32 = ingredients
					.iter()
					.enumerate()
					.map(|(index, x)| x.calories * quantities[index])
					.sum();
				if calories != 500 {
					continue;
				}

				let capacity: i32 = ingredients
					.iter()
					.enumerate()
					.map(|(index, x)| x.capacity * quantities[index])
					.sum();
				if capacity <= 0 {
					continue;
				}

				let durability: i32 = ingredients
					.iter()
					.enumerate()
					.map(|(index, x)| x.durability * quantities[index])
					.sum();
				if durability <= 0 {
					continue;
				}

				let flavor: i32 = ingredients
					.iter()
					.enumerate()
					.map(|(index, x)| x.flavor * quantities[index])
					.sum();
				if flavor <= 0 {
					continue;
				}

				let texture: i32 = ingredients
					.iter()
					.enumerate()
					.map(|(index, x)| x.texture * quantities[index])
					.sum();
				if texture <= 0 {
					continue;
				}

				let total = capacity * durability * flavor * texture;
				if total > biggest_score {
					biggest_score = total;
				}
			}
		}
	}
	biggest_score
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
    fn it_works() {
		let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
		let result = part_2(input);

		assert_eq!(result, 57600000);
}
}
