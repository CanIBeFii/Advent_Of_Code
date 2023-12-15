use serde_json::Value;

fn part_1(input: &str) -> i64 {
	let result: Value = serde_json::from_str(input).unwrap();
	read_value(result)
}

fn read_value(value: Value) -> i64 {
	match value {
		Value::Null => 0,
		Value::Bool(_) => 0,
		Value::Number(x) => x.as_i64().unwrap(),
		Value::String(_) => 0,
		Value::Array(arr) => {
			arr.iter().fold(0, |count, x| count + read_value(x.clone()))
		},
		Value::Object(obj) => {
			let mut count = 0;
			for x in obj.values() {
				count += read_value(x.clone());
			}
			count
		}
	}
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part_1(input);
    dbg!(result);
}
