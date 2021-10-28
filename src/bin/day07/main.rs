use core::{fmt, panic};
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Operator {
    Literal(usize),
    Wire(String),
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{}", value),
            Self::Wire(wire) => write!(f, "{}", wire),
        }
    }
}

impl From<&str> for Operator {
    fn from(s: &str) -> Operator {
        match s.parse::<usize>() {
            Ok(number) => Operator::Literal(number),
            _ => Operator::Wire(s.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
enum Input {
    Assign(Operator),
    Not(Operator),
    And((Operator, Operator)),
    Or((Operator, Operator)),
    Lshift((Operator, Operator)),
    Rshift((Operator, Operator)),
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input::Assign(operator) => write!(f, "{}", operator),
            Input::Not(operator) => write!(f, "NOT {}", operator),
            Input::And((operator0, operator1)) => write!(f, "{} AND {}", operator0, operator1),
            Input::Or((operator0, operator1)) => write!(f, "{} OR {}", operator0, operator1),
            Input::Lshift((operator0, operator1)) => {
                write!(f, "{} LSHIFT {}", operator0, operator1)
            }
            Input::Rshift((operator0, operator1)) => {
                write!(f, "{} RSHIFT {}", operator0, operator1)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    input: Input,
    output: Operator,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.input, self.output)
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let parts: Vec<&str> = s.split(" ").collect();
        match (parts[0], parts[1]) {
            ("NOT", _) => Instruction {
                input: Input::Not(parts[1].into()),
                output: parts[3].into(),
            },
            (_, "OR") => Instruction {
                input: Input::Or((parts[0].into(), parts[2].into())),
                output: parts[4].into(),
            },
            (_, "AND") => Instruction {
                input: Input::And((parts[0].into(), parts[2].into())),
                output: parts[4].into(),
            },
            (_, "LSHIFT") => Instruction {
                input: Input::Lshift((parts[0].into(), parts[2].into())),
                output: parts[4].into(),
            },
            (_, "RSHIFT") => Instruction {
                input: Input::Rshift((parts[0].into(), parts[2].into())),
                output: parts[4].into(),
            },
            (_, "->") => Instruction {
                input: Input::Assign(parts[0].into()),
                output: parts[2].into(),
            },
            _ => panic!("Invalid instruction."),
        }
    }
}

fn get_operator(operator: &Operator, context: &mut Context) -> usize {
    match &operator {
        &Operator::Wire(wire) => {
            let key = &Operator::Wire(wire.to_string());
            match &mut context.cache.get(key) {
                Some(cached_value) => cached_value.clone(),
                None => {
                    let result =
                        execute(&context.instructions_map.get(key).unwrap().clone(), context);
                    context.cache.insert(key.clone(), result);
                    result
                }
            }
        }
        &Operator::Literal(value) => *value,
    }
}

struct Context {
    instructions_map: HashMap<Operator, Instruction>,
    cache: HashMap<Operator, usize>,
}

fn execute(instruction: &Instruction, context: &mut Context) -> usize {
    match &instruction.input {
        Input::Assign(operator) => get_operator(operator, context),
        Input::Not(operator) => !get_operator(operator, context),
        Input::And((operator0, operator1)) => {
            let value0 = get_operator(operator0, context);
            let value1 = get_operator(operator1, context);
            value0 & value1
        }
        Input::Or((operator0, operator1)) => {
            let value0 = get_operator(operator0, context);
            let value1 = get_operator(operator1, context);
            value0 | value1
        }
        Input::Lshift((operator0, operator1)) => {
            let value0 = get_operator(operator0, context);
            let value1 = get_operator(operator1, context);
            value0 << value1
        }
        Input::Rshift((operator0, operator1)) => {
            let value0 = get_operator(operator0, context);
            let value1 = get_operator(operator1, context);
            value0 >> value1
        }
    }
}

fn parse_instructions(input: &str) -> Context {
    let mut context = Context {
        instructions_map: HashMap::new(),
        cache: HashMap::new(),
    };
    input.lines().for_each(|line| {
        let instruction = Instruction::from(line);
        context
            .instructions_map
            .insert(instruction.output.clone(), instruction);
    });
    context
}

fn part_1(input: &str) -> usize {
    let mut context = parse_instructions(input);
    let target = context
        .instructions_map
        .get(&Operator::Wire(String::from("a")))
        .unwrap()
        .clone();
    execute(&target, &mut context)
}

fn part_2(input: &str, b_wire_override: usize) -> usize {
    let mut context = parse_instructions(input);
    let target = context
        .instructions_map
        .get(&Operator::Wire(String::from("a")))
        .unwrap()
        .clone();
    context.instructions_map.insert(
        Operator::Wire(String::from("b")),
        Instruction {
            input: Input::Assign(Operator::Literal(b_wire_override)),
            output: Operator::Wire(String::from("b")),
        },
    );
    execute(&target, &mut context)
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input, output_part_1);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(3176, output_part_1);
    assert_eq!(14710, output_part_2);
}
