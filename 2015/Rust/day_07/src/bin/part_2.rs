use std::collections::HashMap;

enum LogicGate {
    And,
    Or,
    LShift,
	RShift,
	Not,
	None,
}

fn part_2(input: &str) -> u16 {
	let mut wires: HashMap<&str, u16> = HashMap::new();
	let mut lines: Vec<&str> = input.lines().collect();
	let mut index;

	loop {
		if lines.is_empty() {
			break;
		}
		index = 0;
		for line in lines.clone() {
			let line: Vec<&str> = line.split(' ').collect();
			if try_to_execute_line(line, &mut wires) {
				lines.remove(index);
			} else {
				index += 1;
			}
		}
	}
	*wires.get("a").unwrap()
}

fn try_to_execute_line<'a>(line: Vec<&'a str>, wires: &mut HashMap<&'a str, u16>) -> bool {
	match line[..] {
		[a,op,b,_,c] => {
			if let Ok(x) = a.parse::<u16>() {wires.insert(a, x);}
			if let Ok(x) = b.parse::<u16>() {wires.insert(b, x);}

			let wire_1;
			match wires.get(a) {
				Some(num) => { wire_1 = *num;},
				None => {return false},
			};
			
			let gate = get_logic_gate(op);
			match gate {
				LogicGate::And => {
					let wire_2;
					match wires.get(b) {
						Some(num) => {wire_2 = *num;},
						None => {return false},
					};
					let wire_3 = wires.entry(c).or_insert(0);
					*wire_3 = wire_1 & wire_2;

					true
				},
				LogicGate::Or => {
					let wire_2;
					match wires.get(b) {
						Some(num) => {wire_2 = *num;},
						None => {return false},
					};
					let wire_3 = wires.entry(c).or_insert(0);
					*wire_3 = wire_1 | wire_2;

					true
				},
				LogicGate::LShift => {
					let value = b.parse::<u16>().unwrap();
					let wire_3 = wires.entry(c).or_insert(0);
					*wire_3 = wire_1 << value;
					true
				},
				LogicGate::RShift => {
					let value = b.parse::<u16>().unwrap();
					let wire_3 = wires.entry(c).or_insert(0);
					*wire_3 = wire_1 >> value;
					true
				},
				_ => {unreachable!();},
			}
		},
		[_,a,_,b] => {
			let wire_1;
			
			match wires.get(a) {
				Some(num) => {wire_1 = *num;},
				None => {return false},
			};
			let wire_2 = wires.entry(b).or_insert(0);
			*wire_2 = !wire_1;
			true
		},
		[a,_,b] => {
			let num;
			if let Ok(x) = a.parse::<u16>() {
				num = x;
			} else {
				match wires.get(a) {
					Some(x) => {num = *x;},
					None => {return false},
				}
			}

			wires.insert(b, num);
			true
		},
		_ => {unreachable!();},
	}
}

fn get_logic_gate(gate: &str) -> LogicGate {
	if gate == "NOT" {
		LogicGate::Not
	} else if gate == "AND" {
		LogicGate::And
	} else if gate == "OR" {
		LogicGate::Or
	} else if gate == "LSHIFT" {
		LogicGate::LShift
	} else if gate == "RSHIFT" {
		LogicGate::RShift
	} else {
		LogicGate::None
	}
}

fn main() {
    let input = include_str!("./input_2.txt");
    let result = part_2(input);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_works() {
        let input = "123 -> x
x AND y -> de
1 AND y -> e
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
456 -> y
NOT y -> a";
        let result = part_2(input);

        assert_eq!(result, 65079);
    }
}