#[derive(Debug)]
struct Present {
    length: i32,
    width: i32,
    height: i32,
    largest: i32,
}

fn get_present(sizes: Vec<&str>) -> Present {
    let length = sizes.get(0).unwrap().parse::<i32>().unwrap();
    let width = sizes.get(1).unwrap().parse::<i32>().unwrap();
    let height = sizes.get(2).unwrap().parse::<i32>().unwrap();

    let largest = *[length + width, width + height, height + length].iter().min().unwrap();

    Present {
        length: length,
        width: width,
        height: height,
        largest: 2 * largest,
    }
}

fn part_2(input: &str) -> i32 {
    let input = input.to_string();
    let mut list_of_presents: Vec<Present> = Vec::new();

    let mut input = input.lines();

    let mut line = input.next();

    while line != None {
        let sizes: Vec<&str> = line.unwrap().split('x').collect();

		let present = get_present(sizes);
		// dbg!(present);
        list_of_presents.push(present);
        line = input.next();
    }

    list_of_presents.iter().fold(0, |count, p| {
		count + (p.height * p.length * p.width) + p.largest
    })
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
    fn it_works() {
        let input = "2x3x4
1x1x10";
        let result = part_2(input);
        assert_eq!(result, 48);
    }
}
