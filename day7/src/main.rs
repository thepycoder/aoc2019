use itertools::Itertools;
use permutohedron::heap_recursive;
use std::iter::once;
use std::*;

// 55270102 too high

enum Op {
    Add,
    Mul,
    In,
    Out,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}
enum OpCode {
    Arith(Op),
    Halt,
}

fn parse_opcode(opcode: i64) -> (OpCode, Vec<usize>) {
    let opstring = opcode.to_string();
    let mut opvec: Vec<usize> = opstring
        .split("")
        .filter(|x| x != &"")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    opvec.reverse();
    while opvec.len() != 5 {
        opvec.push(0);
    }
    opvec.reverse();

    let real_opcode = opvec[3] * 10 + opvec[4];
    let mut param_modes = vec![opvec[0], opvec[1], opvec[2]];
    // println!("Opvec: {:?}", opvec);
    // println!("Opcode: {:?}", real_opcode);
    // println!("Parameter modes: {:?}", param_modes);
    let return_op: OpCode;
    match real_opcode {
        1 => return_op = OpCode::Arith(Op::Add),
        2 => return_op = OpCode::Arith(Op::Mul),
        3 => {
            return_op = OpCode::Arith(Op::In);
            param_modes[0] = 42;
            param_modes[1] = 42;
        }
        4 => {
            return_op = OpCode::Arith(Op::Out);
            param_modes[0] = 42;
            param_modes[1] = 42;
        }
        5 => {
            return_op = OpCode::Arith(Op::JumpIfTrue);
            param_modes[0] = 42;
            // param_modes[2] = 0;
        }
        6 => {
            return_op = OpCode::Arith(Op::JumpIfFalse);
            param_modes[0] = 42;
            // param_modes[2] = 0;
        }
        7 => {
            return_op = OpCode::Arith(Op::LessThan);
        }
        8 => {
            return_op = OpCode::Arith(Op::Equals);
        }
        99 => return_op = OpCode::Halt,
        _ => panic!("Unknown opcode: {}", real_opcode),
    }
    (return_op, param_modes)
}

fn run_program(state: &mut Vec<i64>, invec: &mut Vec<i64>, mut power_level: i64) -> i64 {
    let mut pointer: usize = 0;
    let mut phase_setting: bool = true;

    loop {
        // println!("{:?}", state);
        let (opcode, param_modes) = parse_opcode(state[pointer]);
        match opcode {
            OpCode::Arith(op) => {
                // println!("Pointer: {}", pointer);

                let x: usize;
                let y: usize;
                let target: usize;

                match param_modes[2] {
                    0 => x = state[pointer + 1] as usize,
                    1 => x = pointer + 1,
                    _ => x = 0,
                }
                match param_modes[1] {
                    0 => y = state[pointer + 2] as usize,
                    1 => y = pointer + 2,
                    _ => y = 0,
                }
                match param_modes[0] {
                    0 => target = state[pointer + 3] as usize,
                    1 => target = pointer + 2,
                    _ => target = 0,
                }

                let jump;
                // println!("x: {}", x);
                // println!("y: {}", y);
                // println!("target: {}", target);

                match op {
                    Op::Add => {
                        let z = state[x] + state[y];
                        state[target] = z;
                        jump = 4;
                    }
                    Op::Mul => {
                        let z = state[x] * state[y];
                        state[target] = z;
                        jump = 4;
                    }
                    Op::In => {
                        let input: i64;
                        // println!("[invec]: {:?}", invec);
                        if phase_setting {
                            phase_setting = false;
                            input = invec.pop().unwrap();
                        // println!("POP!");
                        } else {
                            input = power_level;
                            phase_setting = true;
                        }
                        // println!("[IN]: {}", input);
                        // println!("[invec]: {:?}", invec);
                        state[x] = input;
                        jump = 2;
                    }
                    Op::Out => {
                        // println!("[OUT] {}", state[x]);
                        power_level = state[x];
                        jump = 2;
                    }
                    Op::JumpIfTrue => {
                        if state[x] != 0 {
                            pointer = state[y] as usize;
                            jump = 0;
                        } else {
                            jump = 3;
                        }
                    }
                    Op::JumpIfFalse => {
                        if state[x] == 0 {
                            pointer = state[y] as usize;
                            jump = 0;
                        } else {
                            jump = 3;
                        }
                    }
                    Op::LessThan => {
                        if state[x] < state[y] {
                            state[target] = 1;
                        } else {
                            state[target] = 0;
                        }
                        jump = 4;
                    }
                    Op::Equals => {
                        if state[x] == state[y] {
                            state[target] = 1;
                        } else {
                            state[target] = 0;
                        }
                        jump = 4;
                    }
                };
                // if state[pointer] != current_pointer_val {
                //     jump = 0;
                // }
                pointer += jump;
                // println!("\n",);
            }
            OpCode::Halt => return power_level,
        }
    }
}

fn run_program_pt2(state: &mut Vec<i64>, invec: &mut Vec<i64>, mut power_level: i64, mut pointer: usize, mut phase_setting: bool) -> (i64, usize) {

    loop {
        // println!("{:?}", state);
        let (opcode, param_modes) = parse_opcode(state[pointer]);
        println!("{:?}", state[pointer]);
        match opcode {
            OpCode::Arith(op) => {
                println!("Pointer: {}", pointer);

                let x: usize;
                let y: usize;
                let target: usize;

                match param_modes[2] {
                    0 => x = state[pointer + 1] as usize,
                    1 => x = pointer + 1,
                    _ => x = 0,
                }
                match param_modes[1] {
                    0 => y = state[pointer + 2] as usize,
                    1 => y = pointer + 2,
                    _ => y = 0,
                }
                match param_modes[0] {
                    0 => target = state[pointer + 3] as usize,
                    1 => target = pointer + 2,
                    _ => target = 0,
                }

                let jump;
                // println!("x: {}", x);
                // println!("y: {}", y);
                // println!("target: {}", target);

                match op {
                    Op::Add => {
                        let z = state[x] + state[y];
                        state[target] = z;
                        jump = 4;
                    }
                    Op::Mul => {
                        let z = state[x] * state[y];
                        state[target] = z;
                        jump = 4;
                    }
                    Op::In => {
                        let input: i64;
                        // println!("[invec]: {:?}", invec);
                        if phase_setting {
                            phase_setting = false;
                            input = invec.pop().unwrap();
                        // println!("POP!");
                        } else {
                            input = power_level;
                        }
                        println!("[IN]: {}", input);
                        // println!("[invec]: {:?}", invec);
                        state[x] = input;
                        jump = 2;
                    }
                    Op::Out => {
                        println!("[OUT] {}", state[x]);
                        power_level = state[x];
                        jump = 2;
                        pointer += jump;
                        return (power_level, pointer)
                    }
                    Op::JumpIfTrue => {
                        if state[x] != 0 {
                            pointer = state[y] as usize;
                            jump = 0;
                        } else {
                            jump = 3;
                        }
                    }
                    Op::JumpIfFalse => {
                        if state[x] == 0 {
                            pointer = state[y] as usize;
                            jump = 0;
                        } else {
                            jump = 3;
                        }
                    }
                    Op::LessThan => {
                        if state[x] < state[y] {
                            state[target] = 1;
                        } else {
                            state[target] = 0;
                        }
                        jump = 4;
                    }
                    Op::Equals => {
                        if state[x] == state[y] {
                            state[target] = 1;
                        } else {
                            state[target] = 0;
                        }
                        jump = 4;
                    }
                };
                // if state[pointer] != current_pointer_val {
                //     jump = 0;
                // }
                pointer += jump;
                // println!("\n",);
            }
            OpCode::Halt => return (power_level, 9999)
        }
    }
}

fn main() {
    let raw: String = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string();
    // let filename = "src/input.txt";

    // let raw = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut input: Vec<i64> = raw
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    // println!("{:?}", input);

    // // let mut invec = vec![1,0,4,3,2];

    // let range = || (0..=4);

    // let all: Vec<Vec<i64>> = once(range())
    //     .chain(once(range()))
    //     .chain(once(range()))
    //     .chain(once(range()))
    //     .chain(once(range()))
    //     .multi_cartesian_product()
    //     .collect();

    // // println!("{:?}", all);

    // let mut data = [0, 1, 2, 3, 4];
    // let mut permutations = Vec::new();
    // heap_recursive(&mut data, |permutation| {
    //     permutations.push(permutation.to_vec())
    // });

    // let mut max = 0;
    // for mut invec in permutations {
    //     let cp = invec.clone();
    //     invec.reverse();
    //     power_level = 0;
    //     for _amp in 0..5 {
    //         power_level = run_program(&mut input, &mut invec, power_level);
    //     }
    //     if power_level > max {
    //         max = power_level;
    //     }
    //     if cp == vec![1, 0, 4, 3, 2] {
    //         println!("{:?}", cp);
    //         println!("{:?}\n", power_level);
    //     }
    // }

    // Part 2
    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });

    let mut power_level = 0;

    let mut max = 0;
    for mut invec in permutations {
        println!("{:?}", invec);
        let cp = invec.clone();
        let mut pointers: Vec<usize> = vec!(0,0,0,0,0);
        let mut phase_setting = true;
        invec.reverse();
        power_level = 0;

        while !pointers.contains(&9999) {
            for amp in 0..5 {
                println!("Amp {}", amp);
                println!("State: {:?}", input);
                let (temp_power, pointer) = run_program_pt2(&mut input, &mut invec, power_level, pointers[amp], phase_setting);
                println!("State: {:?}", input);
                pointers[amp] = pointer;
                power_level = temp_power;
                println!("power_level {}", power_level);
                println!("pointers {:?}", pointers);
                
                println!("\n");
            }
            phase_setting = false;
            // if power_level > 100 {
            //     break;
            // }
        }

        if power_level > max {
            max = power_level;
        }
        if cp == vec![9,8,7,6,5] {
            println!("{:?}", cp);
            println!("{:?}\n", power_level);
        }

        // break
    }

    println!("{:?}", max);
}
