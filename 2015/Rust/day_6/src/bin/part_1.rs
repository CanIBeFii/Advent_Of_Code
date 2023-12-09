use std::collections::HashSet;

enum LightAction {
    TurnOn,
    TurnOff,
    Toggle,
}

fn part_1(input: &str) -> u32 {
    let mut lights = HashSet::<(i32, i32)>::new();
    input.lines().for_each(|line| {
        if line.contains("turn on ") {
			parse_line(&mut lights, line, LightAction::TurnOn);
        } else if line.contains("toggle ") {
			parse_line(&mut lights, line, LightAction::Toggle);
        } else {
			parse_line(&mut lights, line, LightAction::TurnOff);
        }
    });
    lights.len() as u32
}

fn parse_line(lights: &mut HashSet<(i32, i32)>, line: &str, action: LightAction) {
    let mut fields = line.split(' ');

	if line.contains("on") || line.contains("off") {
		fields.next();
	}
    fields.next();
    let mut begin = fields.next().unwrap().split(',');
    fields.next();
    let mut end = fields.next().unwrap().split(',');

	
    let mut begin: (i32, i32) = (
		begin.next().unwrap().parse::<i32>().unwrap(),
        begin.next().unwrap().parse::<i32>().unwrap(),
    );
	
    let end: (i32, i32) = (
		end.next().unwrap().parse::<i32>().unwrap(),
        end.next().unwrap().parse::<i32>().unwrap(),
    );
	println!("Coords: {},{} -> begin and {},{} -> end", begin.0, begin.1, end.0, end.1);

	let temp_begin_y = begin.1;

	while begin.0 <= end.0 {
		begin.1 = temp_begin_y;
		while begin.1 <= end.1 {
			match action {
				LightAction::TurnOn => { lights.insert(begin); },
				LightAction::TurnOff => { lights.remove(&begin); },
				LightAction::Toggle => {
					if !lights.insert(begin) {
						lights.remove(&begin);
					}
				}
			}
			begin.1 += 1;
		}
		lights.iter().len();
		begin.0 += 1;
	}
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
    fn number_of_lights_3992() {
        let input = "toggle 0,0 through 999,0
toggle 0,0 through 0,999
toggle 0,999 through 999,999
toggle 999,0 through 999,999";
        let result = part_1(input);

        assert_eq!(result, 3992);
    }

	#[test]
	fn number_of_lights_1000000() {
		let input = "turn on 0,0 through 999,999
";
		let result = part_1(input);

		assert_eq!(result, 1000000);
	}
}
