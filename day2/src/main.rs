use std::fs;

fn main() {
    let filename = "src/input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // let split = contents.split("\n");
    let mut opcodes: Vec<usize> = contents.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    opcodes[1] = 12;
    opcodes[2] = 2;

    let muh = part1(opcodes);
    println!("{}", muh);
    // part1(vec![1,0,0,0,99])
    // part1(vec![1,1,1,4,99,5,6,0,99])

    let filename = "src/input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // let split = contents.split("\n");
    let opcodes: Vec<usize> = contents.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    part2(opcodes);



}

fn part1(mut opcodes: Vec<usize>) -> usize {

    let mut i: usize = 0;
    let f;

    loop {
        match opcodes.get(i) {
            Some(x) => {
                if i < opcodes.len() - 3 {
                    let n1 = opcodes[opcodes[i + 1]];
                    let n2 = opcodes[opcodes[i + 2]];
                    let dest = opcodes[i + 3];
                    if *x == 1 {
                        opcodes[dest] = n1 + n2
                    } else if *x == 2 {
                        opcodes[dest] = n1 * n2
                    } else {
                        // println!("Vector: {:?}", opcodes);
                        f = opcodes[0];
                        break;
                    }
                } else {
                    // println!("Vector: {:?}", opcodes);
                    f = opcodes[0];
                    break;
                }
            },
            None => {
                f = opcodes[0];
                break;
            }
        }
        i += 4;
    }
    f
}

fn part2(opcodes: Vec<usize>) {
    let mut opcodes_2: Vec<usize> = opcodes.clone();
    'outer: for noun in 0..100 {
        'inner: for verb in 0..100 {
            // println!("{} - {}", noun, verb);
            // if noun == 12 && verb == 2 {
            //     println!("Found it! {}", 100 * noun * verb);
            //     break 'outer;
            // }
            opcodes_2[1] = noun;
            opcodes_2[2] = verb;
            let result = part1(opcodes_2.clone());
            // println!("{} - {}: {}", noun, verb, result);
            if result == 19690720 {
                println!("Found it! {}", 100 * noun + verb);
                break 'outer;
            }
            opcodes_2 = opcodes.clone();
        }
    }
}