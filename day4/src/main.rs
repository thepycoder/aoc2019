// fn check_vec(number: Vec) -> bool {

// }

use std::collections::HashSet;

fn main() {
    let mut answer1 = 0;
    let mut answer2 = 0;

    for i in 402328..864247 {
        if part1(i.to_string()) {
            answer1 += 1;
        }
        if part2(i.to_string()) {
            answer2 += 1;
        }
    }
    println!("Answer part 1: {}", answer1);
    println!("Answer part 2: {}", answer2);
}

fn part1(input: String) -> bool {
    let vec: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
    let mut seen = HashSet::new();
    seen.insert(vec[0].clone());

    for i in 1..vec.len() {
        let num = vec[i];
        let prev_num = vec[i-1];
        if num < prev_num {
            return false;
        } else {
            seen.insert(num);
        }
    }
    // println!("{:?}", vec);
    // println!("{:?}", seen);
    if seen.len() < vec.len() {
        return true;
    } else {
        return false;
    }
}

fn part2(input: String) -> bool {
    let vec: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
    let mut seen = HashSet::new();
    seen.insert(vec[0].clone());

    for i in 1..vec.len() {
        let num = vec[i];
        let prev_num = vec[i-1];
        if num < prev_num {
            return false;
        } else {
            seen.insert(num);
        }
    }

    for nr in &seen {
        let count: u32 = vec.iter().filter(|x| x==&nr).count() as u32;
        if count == 2 {
            return true;
        }
    }

    return false;
}
