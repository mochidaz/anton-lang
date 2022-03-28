use std::env;
use std::io::{Read, Write};
use std::fs::File;

#[derive(Debug)]
#[derive(Clone)]
enum OpCode {
    AntonIncPtr,
    AntonDecPtr,
    AntonInc,
    AntoncDec,
    // Write
    NovelSifo,
    // Read
    NovelSifoRead,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug)]
#[derive(Clone)]
enum Instruction {
    AntonIncPtr,
    AntonDecPtr,
    AntonInc,
    AntonDec,
    // Write
    NovelSifo,
    // Read
    NovelSifoRead,
    Loop(Vec<Instruction>)
}

fn convert_code(source: String) -> String {

    let mut converted_source = String::new();

    for op in source.chars() {
        let opcode = match op {
            '>' => "floss:)",
            '<' => "django:)",
            '+' => "wuzz:)",
            '-' => "pypy:)",
            '.' => "quantum:)",
            ',' => "wagtail:)",
            '[' => "avlinux:)",
            ']' => "mxedition:)",
            _ => ""
        };

        converted_source.push_str(opcode);
    }

    converted_source
}

fn lex(source: String) -> Vec<OpCode> {
    let mut operations = Vec::new();

    let splitted_code = source.split(":)").collect::<Vec<&str>>();

    for symbol in splitted_code {
        let op = match symbol {
            "floss" => Some(OpCode::AntonIncPtr),
            "django" => Some(OpCode::AntonDecPtr),
            "wuzz" => Some(OpCode::AntonInc),
            "pypy" => Some(OpCode::AntoncDec),
            "quantum" => Some(OpCode::NovelSifo),
            "wagtail" => Some(OpCode::NovelSifoRead),
            "avlinux" => Some(OpCode::LoopBegin),
            "mxedition" => Some(OpCode::LoopEnd),
            _ => None
        };

        match op {
            Some(op) => operations.push(op),
            None => ()
        }
    }

    operations
}

fn parse(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OpCode::AntonIncPtr => Some(Instruction::AntonIncPtr),
                OpCode::AntonDecPtr => Some(Instruction::AntonDecPtr),
                OpCode::AntonInc => Some(Instruction::AntonInc),
                OpCode::AntoncDec => Some(Instruction::AntonDec),
                OpCode::NovelSifo => Some(Instruction::NovelSifo),
                OpCode::NovelSifoRead => Some(Instruction::NovelSifoRead),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },

                OpCode::LoopEnd => panic!("loop ending at #{} has no beginning", i),
            };

            match instr {
                Some(instr) => program.push(instr),
                None => ()
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                },
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(opcodes[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!("loop that starts at #{} has no matching ending!", loop_start);
    }

    program
}

fn run(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, data_pointer: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::AntonIncPtr => *data_pointer += 1,
            Instruction::AntonDecPtr => *data_pointer -= 1,
            Instruction::AntonInc => tape[*data_pointer] += 1,
            Instruction::AntonDec => tape[*data_pointer] -= 1,
            Instruction::NovelSifo => print!("{}", tape[*data_pointer] as char),
            Instruction::NovelSifoRead => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("failed to read stdin");
                tape[*data_pointer] = input[0];
            },
            Instruction::Loop(nested_instructions) => {
                while tape[*data_pointer] != 0 {
                    run(&nested_instructions, tape, data_pointer)
                }
            }
        }
    }
}

fn main() {
    // Determine which file to execute
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: at <file.at> or at convert file.bf");
        std::process::exit(1);
    }

    let input = &args[1];

    match input.as_str() {
        "convert" => {
            if args.len() < 3 {
                println!("usage: at <file.at> or at convert file.bf");
                std::process::exit(1);
            }

            let mut bf_source = String::new();

            let mut bf_file = match File::open(&args[2]) {
                Ok(v) => v,
                Err(_) => panic!("Cannot open bf file!"),
            };

            bf_file.read_to_string(&mut bf_source).unwrap();

            let mut file = match File::create(&args[2].replace(".bf", ".at")) {
                Ok(f) => f,
                Err(_) => panic!("Cannot create file!"),
            };

            let convert = convert_code(bf_source);

            file.write_all(convert.as_bytes()).unwrap();
        }
        _ => {
            let mut source = String::new();

            match File::open(input) {
                Ok(mut v) => {
                    v.read_to_string(&mut source).expect("failed to read program file");
                }
                Err(_) => {
                    // To interpret directly
                    source.push_str(input);
                }
            };

            let opcodes = lex(source);

            let program = parse(opcodes);

            let mut tape: Vec<u8> = vec![0; 1024];
            let mut data_pointer = 512;

            run(&program, &mut tape, &mut data_pointer);
        }
    }
}