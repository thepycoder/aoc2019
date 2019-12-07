use std::env;
use std::fs;
use math::round;

fn main() {
    // Part 1
    let filename = "src/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // let split = contents.split("\n");
    let mut vec: Vec<f64> = contents.split("\n").map(|x| x.parse::<f64>().unwrap()).collect();

    let mut total = 0.0;

    for s in vec {
        // let weight = s.parse::<f64>().unwrap();
        let weight = s;
        let fuel = round::floor(weight / 3.0, 0) - 2.0;
        total += fuel;
    }

    println!("{}", total);

    let mut total_2 = 0.0;
    let mut vec: Vec<f64> = contents.split("\n").map(|x| x.parse::<f64>().unwrap()).collect();

    // Part 2
    while let Some(weight) = vec.pop() {
        // let weight = s.parse::<f64>().unwrap();
        // let weight = s;
        let fuel = round::floor(weight / 3.0, 0) - 2.0;

        if fuel >= 0.0 {
            total_2 += fuel;
            vec.push(fuel);
        }
    }

    println!("{}", total_2);
}
