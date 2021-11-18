use itertools::Itertools;
use log::debug;

#[derive(Debug, Clone)]
struct Context {
    register_a: usize,
    register_b: usize,
}

#[derive(Debug)]
enum RegisterType {
    A,
    B,
}

impl TryFrom<&str> for RegisterType {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = if let Some(input) = input.strip_suffix(",") {
            input
        } else {
            input
        };
        match input {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err(format!("Invalid register type {}", input)),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Hlf(RegisterType),
    Tpl(RegisterType),
    Inc(RegisterType),
    Jmp(i64),
    Jie(RegisterType, i64),
    Jio(RegisterType, i64),
}

type Program = Vec<Instruction>;

fn main() {
    env_logger::init();
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(184, output_part_1);
    assert_eq!(231, output_part_2);
}

fn parse_program(input: &str) -> Result<Program, String> {
    let mut program: Program = vec![];
    for line in input.lines() {
        let line_parts = line.split(" ").collect_vec();
        match line_parts[0] {
            "hlf" => program.push(Instruction::Hlf(RegisterType::try_from(line_parts[1])?)),
            "tpl" => program.push(Instruction::Tpl(RegisterType::try_from(line_parts[1])?)),
            "inc" => program.push(Instruction::Inc(RegisterType::try_from(line_parts[1])?)),
            "jmp" => program.push(Instruction::Jmp(
                line_parts[1]
                    .parse::<i64>()
                    .map_err(|_| "Can't parse input")?,
            )),
            "jie" => program.push(Instruction::Jie(
                RegisterType::try_from(line_parts[1])?,
                line_parts[2]
                    .parse::<i64>()
                    .map_err(|_| format!("Can't parse input {} into i64", line_parts[2]))?,
            )),
            "jio" => program.push(Instruction::Jio(
                RegisterType::try_from(line_parts[1])?,
                line_parts[2]
                    .parse::<i64>()
                    .map_err(|_| format!("Can't parse input {} into i64", line_parts[2]))?,
            )),
            _ => panic!("invalid instruction"),
        }
    }
    Ok(program)
}

fn execute_program(program: &Program, initial_context: &Context) -> Context {
    let mut context = initial_context.clone();
    let mut program_counter: i64 = 0;
    loop {
        if program_counter < 0 || program_counter as usize >= program.len() {
            debug!(
                "================================\nProgram exiting because PC is outside bounds.\nContext = {:?}",
                context
            );
            break;
        }
        debug!(
            "================================\nContext = {:?}\nPC = {} => {:?};",
            context, program_counter, &program[program_counter as usize]
        );
        match &program[program_counter as usize] {
            Instruction::Hlf(register) => {
                match register {
                    RegisterType::A => {
                        context.register_a /= 2;
                    }
                    RegisterType::B => {
                        context.register_b /= 2;
                    }
                };
                program_counter += 1;
            }
            Instruction::Tpl(register) => {
                match register {
                    RegisterType::A => {
                        context.register_a *= 3;
                    }
                    RegisterType::B => {
                        context.register_b *= 3;
                    }
                };
                program_counter += 1;
            }
            Instruction::Inc(register) => {
                match register {
                    RegisterType::A => {
                        context.register_a += 1;
                    }
                    RegisterType::B => {
                        context.register_b += 1;
                    }
                };
                program_counter += 1;
            }
            Instruction::Jmp(offset) => program_counter += offset,
            Instruction::Jie(register, offset) => {
                let register_value = match register {
                    RegisterType::A => context.register_a,
                    RegisterType::B => context.register_b,
                };
                if register_value % 2 == 0 {
                    program_counter += offset;
                } else {
                    program_counter += 1;
                }
            }
            Instruction::Jio(register, offset) => {
                let register_value = match register {
                    RegisterType::A => context.register_a,
                    RegisterType::B => context.register_b,
                };
                if register_value == 1 {
                    program_counter += offset;
                } else {
                    program_counter += 1;
                }
            }
        }
    }
    context
}

fn part_1(input: &str) -> usize {
    let program = parse_program(input).unwrap();
    let mut context = Context {
        register_a: 0,
        register_b: 0,
    };
    context = execute_program(&program, &context);
    context.register_b
}

fn part_2(input: &str) -> usize {
    let program = parse_program(input).unwrap();
    let mut context = Context {
        register_a: 1,
        register_b: 0,
    };
    context = execute_program(&program, &context);
    context.register_b
}
