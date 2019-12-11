#[macro_use] extern crate text_io;
use ::std::*;

// 223 too low


enum Op {
    Add,
    Mul,
    In,
    Out,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals
}
enum OpCode {
    Arith(Op),
    Halt,
}

fn parse_opcode(opcode: i32) -> (OpCode, Vec<usize>) {
    let opstring = opcode.to_string();
    let mut opvec: Vec<usize> = opstring.split("").filter(|x| x != &"").map(|x| x.parse::<usize>().unwrap()).collect();

    opvec.reverse();
    while opvec.len() != 5 {
        opvec.push(0);
    }
    opvec.reverse();

    let real_opcode = opvec[3] * 10 + opvec[4];
    let mut param_modes = vec![opvec[0], opvec[1], opvec[2]];
    // println!("Opvec: {:?}", opvec);
    println!("Opcode: {:?}", real_opcode);
    // println!("Parameter modes: {:?}", param_modes);
    let return_op: OpCode;
    match real_opcode {
        1 => return_op = OpCode::Arith(Op::Add),
        2 => return_op = OpCode::Arith(Op::Mul),
        3 => {
            return_op = OpCode::Arith(Op::In);
            param_modes[0] = 42;
            param_modes[1] = 42;
        },
        4 => {
            return_op = OpCode::Arith(Op::Out);
            param_modes[0] = 42;
            param_modes[1] = 42;
        },
        5 => {
            return_op = OpCode::Arith(Op::JumpIfTrue);
            param_modes[0] = 42;
            // param_modes[2] = 0;
        },
        6 => {
            return_op = OpCode::Arith(Op::JumpIfFalse);
            param_modes[0] = 42;
            // param_modes[2] = 0;
        },
        7 => {
            return_op = OpCode::Arith(Op::LessThan);
        },
        8 => {
            return_op = OpCode::Arith(Op::Equals);
        },
        99 => return_op = OpCode::Halt,
        _ => panic!("Unknown opcode: {}", real_opcode),
    }
    (return_op, param_modes)
}

fn run_program(state: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut pointer: usize = 0;
    loop {
        // println!("{:?}", state);
        let mut current_pointer_val = state[pointer];
        let (opcode, param_modes) = parse_opcode(state[pointer]);
        match opcode {
            OpCode::Arith(op) => {

                println!("Pointer: {}", pointer);

                let x: usize;
                let y: usize;
                let target: usize;

                match param_modes[2] {
                    0 => x = state[pointer + 1] as usize,
                    1 => x = pointer + 1,
                    _ => x = 0
                }
                match param_modes[1] {
                    0 => y = state[pointer + 2] as usize,
                    1 => y = pointer + 2,
                    _ => y = 0
                }
                match param_modes[0] {
                    0 => target = state[pointer + 3] as usize,
                    1 => target = pointer + 2,
                    _ => target = 0
                }

                let mut jump;
                println!("x: {}", x);
                println!("y: {}", y);
                println!("target: {}", target);

                match op {
                    Op::Add => {
                        let z = state[x] + state[y];
                        state[target] = z;
                        jump = 4;
                    },
                    Op::Mul => {
                        let z = state[x] * state[y];
                        state[target] = z;
                        jump = 4;
                    },
                    Op::In => {
                        println!("ID: ");
                        let input: i32 = read!();
                        // println!("[IN] {}", input);
                        state[x] = input;
                        jump = 2;
                    },
                    Op::Out => {
                        println!("[OUT] {}", state[x]);
                        jump = 2;
                    },
                    Op::JumpIfTrue => {
                        if state[x] != 0 {
                            pointer = state[y] as usize;
                            jump = 0;
                        } else {
                            jump = 3;
                        }
                    },
                    Op::JumpIfFalse => {
                        if state[x] == 0 {
                            pointer = state[y] as usize;
                            jump = 0;
                        } else {
                            jump = 3;
                        }
                    },
                    Op::LessThan => {
                        if state[x] < state[y] {
                            state[target] = 1;
                        } else {
                            state[target] = 0;
                        }
                        jump = 4;
                    },
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
                println!("\n", );
            }
            OpCode::Halt => return state,
        }
    }
}

fn main() {
    // let raw: String = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
    let filename = "src/input2.txt";

    let raw = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut input: Vec<i32> = raw.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    // println!("{:?}", input);

    let test = run_program(&mut input);
    // println!("{:?}", test);
}
