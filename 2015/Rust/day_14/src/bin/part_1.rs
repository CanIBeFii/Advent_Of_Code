#[derive(Debug, PartialEq)]
struct Reindeer {
	name: String,
	flying_speed: u16,
	flying_time: u16,
	resting_time: u16,
}

impl Reindeer {
	fn new(name: &str, fly_speed: u16, fly_time: u16, rest_time: u16) -> Reindeer {
		Reindeer{
			name: name.to_string(),
			flying_speed: fly_speed,
			flying_time: fly_time,
			resting_time: rest_time,
		}
	}

	fn distance_traveled_in_x_seconds(&self, time: u16) -> u16 {
		let cicle_time = self.flying_time + self.resting_time;
		let number_of_cicles = time / cicle_time;
		let remaining_time = time % cicle_time;

		let mut distance_traveled = number_of_cicles * self.flying_time * self.flying_speed;

		println!("In {number_of_cicles} with traveled {distance_traveled}");

		if remaining_time > self.flying_time {
			distance_traveled += self.flying_time * self.flying_speed;
		} else {
			distance_traveled += remaining_time * self.flying_speed;
		}
		distance_traveled
	}
}

fn parse_line(line: &str) -> Reindeer {
	let line: Vec<&str> = line.split(' ').collect();
	
	Reindeer::new(
		line.first().unwrap(),
		line.get(3).unwrap().parse().unwrap(),
		line.get(6).unwrap().parse().unwrap(),
		line.get(13).unwrap().parse().unwrap(),
	)
}

fn part_1(input: &str) -> u16 {
	let reindeers: Vec<Reindeer> = input.lines().map(parse_line).collect();

	let mut max_distance = 0;
	for r in reindeers {
		let distance = r.distance_traveled_in_x_seconds(2503);
		if distance > max_distance {
			max_distance = distance;
		}
	}
	max_distance
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
	fn reindeer_new_works() { // Obviously it will work but idk test it anyway
		let expected = Reindeer{
			name: "Ola".to_string(),
			flying_speed: 1,
			flying_time: 2,
			resting_time: 3,
		};
		let result = Reindeer::new("Ola", 1, 2, 3);

		assert_eq!(result, expected);
	}

    #[test]
    fn parse_line_works() {
        let input = "Dasher can fly 4 km/s for 16 seconds, but then must rest for 55 seconds.";
        let result = parse_line(input);
		let expected = Reindeer::new("Dasher", 4, 16, 55);
        assert_eq!(result, expected);
    }

	#[test]
	fn distance_traveled_in_x_seconds_works() {
		let rein = Reindeer::new("Comet", 14, 10, 127);
		let result = rein.distance_traveled_in_x_seconds(1000);

		assert_eq!(result, 1120);
	}
}
