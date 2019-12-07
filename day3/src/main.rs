use std::fs;
use indexmap::IndexSet;

// 117 too low
// 43769 too low pt2
// 45784 too high pt2

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    // let input = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
    // let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string();
    // let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string();
    part1(input);
}


fn get_points(wire: String) -> IndexSet<Point<i64>> {
    let mut points = IndexSet::new();

    let mut x = 0;
    let mut y = 0;

    for instruction in wire.split(",") {
        // println!("{}", instruction);
        let (dir, str_amount) = instruction.split_at(1);
        let amount = str_amount.parse::<i64>().unwrap();

        for _i in 0..amount {
            match dir {
                "U" => y += 1,
                "D" => y -= 1,
                "L" => x -= 1,
                "R" => x += 1,
                _ => println!("Euh, something went wrong")
            }
            let new_point = Point{x, y};
            points.insert(new_point);
        }
    }
    points
}


fn part1(str_input: String) -> i64 {

    let wires: Vec<&str> = str_input.split("\n").collect();
    let wire1 = wires[0].to_string();
    let wire2 = wires[1].to_string();

    let vec_1 = get_points(wire1);
    let vec_2 = get_points(wire2);

    let intersection: IndexSet<_> = vec_1.intersection(&vec_2).collect();

    // println!("{:?}", intersection);
    // println!("{:?}", all);

    let mut distance;
    let mut answer = 99999;

    // let mut steps_1: usize;
    // let mut steps_2: usize;

    let mut steps: usize;
    let mut answer_2: usize = 99999;

    // let mut _el;
    // let mut _el2;

    for element in &intersection {
        distance = element.x.abs() + element.y.abs();
        if distance < answer {
            answer = distance;
        }

        let (steps_1, _el) = vec_1.get_full(element.to_owned()).unwrap();
        let (steps_2, _el2) = vec_2.get_full(element.to_owned()).unwrap();
        steps = steps_1 + steps_2;
        if steps < answer_2 {
            answer_2 = steps;
        }
    }

    println!("{}", answer);
    println!("{}", answer_2 + 2);
    answer
}


#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "D2,R4\nR2,D5".to_string();
        assert_eq!(part1(input), 4);

        let input = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
        assert_eq!(part1(input), 6);

        let input = "L8,D5,R5,U3\nD7,L6,U4,R4".to_string();
        assert_eq!(part1(input), 6);

        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string();
        assert_eq!(part1(input), 135);

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string();
        assert_eq!(part1(input), 159);
    }
}
