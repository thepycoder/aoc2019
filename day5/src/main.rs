use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // let input: Vec<&str> = "COM)B
    //                         B)C
    //                         C)D
    //                         D)E
    //                         E)F
    //                         B)G
    //                         G)H
    //                         D)I
    //                         E)J
    //                         J)K
    //                         K)L".split("\n").collect();
    let filestring = fs::read_to_string("src/input.txt")
    .expect("Something went wrong reading the file");
    let input = filestring.split("\n").collect();

    part1(input);


    let input: Vec<&str> = "COM)B
                            B)C
                            C)D
                            D)E
                            E)F
                            B)G
                            G)H
                            D)I
                            E)J
                            J)K
                            K)L
                            K)YOU
                            I)SAN".split("\n").collect();
    let filestring = fs::read_to_string("src/input.txt")
    .expect("Something went wrong reading the file");
    let input = filestring.split("\n").collect();
    part2(input);
}

fn part1(input: Vec<&str>) {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in input {
        let entry: Vec<&str> = line.trim().split(")").collect();
        // println!("{}", entry[0]);

        orbits.insert(entry[1], entry[0]);
    }

    let mut counter = 0;
    for (mut left, mut right) in &orbits {
        // println!("{} - {}", left, right);
        while left != &"COM" {
            counter += 1;
            right = &left;
            // println!("{:?} - {}", orbits, &left);
            left = orbits.get(left).unwrap();
            // println!("Left: {}", left);
            // println!("Right: {}", right);
        }
    }
    println!("{}", counter);
}


fn part2(input: Vec<&str>) {
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    let mut src_path = HashSet::new();
    let mut dst_path = HashSet::new();


    for line in input {
        let entry: Vec<&str> = line.trim().split(")").collect();
        // println!("{}", entry[0]);

        orbits.insert(entry[1], entry[0]);
    }

    // src
    let mut right = "YOU";
    let mut left = orbits.get(right).unwrap();

    // println!("{}", left);

    while left != &"COM" {
        right = &left;
        // println!("{:?} - {}", orbits, &left);
        left = orbits.get(left).unwrap();
        // println!("Left: {}", left);
        // println!("Right: {}", right);
        src_path.insert((left, right));
    }

    // dst
    let mut right = "SAN";
    let mut left = orbits.get(right).unwrap();

    // println!("{}", left);

    while left != &"COM" {
        right = &left;
        // println!("{:?} - {}", orbits, &left);
        left = orbits.get(left).unwrap();
        // println!("Left: {}", left);
        // println!("Right: {}", right);
        dst_path.insert((left, right));
    }

    // println!("{:?}", src_path);
    // println!("{:?}", dst_path);

    let transfers: HashSet<_> = src_path.symmetric_difference(&dst_path).collect();

    println!("{:?}", transfers.len());

    // let mut counter = 0;
    // for (mut left, mut right) in &orbits {
    //     // println!("{} - {}", left, right);
    //     while left != &"COM" {
    //         counter += 1;
    //         right = &left;
    //         // println!("{:?} - {}", orbits, &left);
    //         left = orbits.get(left).unwrap();
    //         // println!("Left: {}", left);
    //         // println!("Right: {}", right);
    //     }
    // }
    // println!("{}", counter);
}
