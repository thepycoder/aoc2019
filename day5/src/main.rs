#[macro_use] extern crate text_io;
use ::std::*;


enum Op {
    Add,
    Mul,
    In,
    Out
}
enum OpCode {
    Arith(Op),
    Halt,
}

fn parse_opcode(opcode: i32) -> OpCode {
    let opstring = opcode.to_string();
    let opvec: Vec<&str> = opstring.split("").filter(|x| x != &"").collect();
    println!("{:?}", opvec);
    match opcode {
        1 => OpCode::Arith(Op::Add),
        2 => OpCode::Arith(Op::Mul),
        3 => OpCode::Arith(Op::In),
        4 => OpCode::Arith(Op::Out),
        99 => OpCode::Halt,
        _ => panic!("Unknown opcode"),
    }
}

fn run_program(state: &mut Vec<i32>) -> i32 {
    let mut pointer = 0;
    loop {
        let opcode = parse_opcode(state[pointer]);
        match opcode {
            OpCode::Arith(op) => {
                let x = state[state[pointer + 1] as usize];
                let y = state[state[pointer + 2] as usize];
                let target = state[pointer + 3] as usize;
                match op {
                    Op::Add => {
                        let z = x + y;
                        state[target] = z;
                    },
                    Op::Mul => {
                        let z = x * y;
                        state[target] = z;
                    },
                    Op::In => {
                        println!("ID: ");
                        let input: i32 = read!();
                        println!("[OUT] {}", input);
                        state[target] = input;
                    },
                    Op::Out => {
                        println!("[OUT] {}", x);
                    }
                };
                pointer += 4;
            }
            OpCode::Halt => return state[0],
        }
    }
}

fn main() {
    let raw: String = "1101,100,-1,4,0".to_string();
    let mut input: Vec<i32> = raw.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{:?}", input);

    let test = run_program(&mut input);
    println!("{}", test);
}
