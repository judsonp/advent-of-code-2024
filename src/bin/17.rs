use std::collections::HashSet;

use itertools::Itertools;
use scan_fmt::scan_fmt_some;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let input = parse_input(input);

    let (output, _) = run_program(&input.initial_state, &input.program);

    Some(output.iter().join(","))
}

// The input program disassembles to the following:
// 00: bst a
// 02: bxl 3
// 04: cdv [2^b]
// 06: bxc 1
// 08: bxl 3
// 0a: adv 8
// 0c: out [b % 8]
// 0e: jnz 0

// This is basically:
// while a != 0:
//   b = a & 8
//   b ^= 3
//   c = a / (1 << b)
//   b ^= c
//   b ^= 3
//   a /= 8
//   output b

// Key pieces of this general structure:
//   * outputs one symbol per iteration
//   * divides a by 8 (3 bits) each iteration
//   * b and c are set, so their state coming in to the loop doesn't matter

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let shortened_program = &input.program[..input.program.len() - 2];

    let mut options = HashSet::new();
    options.insert(0);
    for target in input.program.iter().rev() {
        options = options
            .iter()
            .flat_map(|a| produce(shortened_program, *target, *a))
            .collect();
    }

    let result = options
        .iter()
        .filter(|a| {
            let mut state = input.initial_state.clone();
            state.a = **a;
            let (output, _) = run_program(&state, &input.program);
            output.eq(&input.program)
        })
        .min()
        .unwrap();

    Some(*result)
}

fn run_program(state: &MachineState, program: &[u8]) -> (Vec<u8>, MachineState) {
    let mut state = state.clone();
    let mut output = Vec::new();

    while state.pc < program.len() {
        let opcode = program[state.pc];
        let operand = program[state.pc + 1];

        let pc = match opcode {
            0 => {
                // adv
                let divisor = 1 << combo_operand(operand, &state);
                state.a /= divisor;
                None
            }
            1 => {
                // bxl
                state.b ^= operand as u64;
                None
            }
            2 => {
                // bst
                state.b = combo_operand(operand, &state) % 8;
                None
            }
            3 => {
                // jnz
                if state.a == 0 {
                    None
                } else {
                    Some(operand as usize)
                }
            }
            4 => {
                // bxc
                state.b ^= state.c;
                None
            }
            5 => {
                // out
                output.push((combo_operand(operand, &state) % 8) as u8);
                None
            }
            6 => {
                // bdv
                let divisor = 1 << combo_operand(operand, &state);
                state.b = state.a / divisor;
                None
            }
            7 => {
                // cdv
                let divisor = 1 << combo_operand(operand, &state);
                state.c = state.a / divisor;
                None
            }
            _ => panic!("Invalid opcode"),
        };

        state.pc = pc.unwrap_or(state.pc + 2);
    }

    (output, state)
}

fn combo_operand(operand: u8, state: &MachineState) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("Invalid combo operand"),
    }
}

fn produce(program: &[u8], target: u8, starting_a: u64) -> Vec<u64> {
    (0..8)
        .map(|a_offset| {
            let state = MachineState {
                a: (starting_a << 3) + a_offset,
                b: 0,
                c: 0,
                pc: 0,
            };

            let (output, state) = run_program(&state, program);
            let output = output[0];

            (a_offset, output, state.a)
        })
        .filter(|(_, output, a)| *output == target && *a == starting_a)
        .map(|(a_offset, _, _)| (starting_a << 3) + a_offset)
        .collect()
}

#[allow(dead_code)]
fn disassemble_program(program: &[u8]) -> String {
    let mut pc = 0;
    let mut output = String::new();

    while pc < program.len() {
        let opcode = program[pc];
        let operand = program[pc + 1];

        let instruction = match opcode {
            0 => format!("adv {}", format_divisor(operand)),
            1 => format!("bxl {}", operand),
            2 => format!("bst [{} % 8]", format_combo_operand(operand)),
            3 => format!("jnz {}", operand),
            4 => format!("bxc {}", operand),
            5 => format!("out [{} % 8]", format_combo_operand(operand)),
            6 => format!("bdv {}", format_divisor(operand)),
            7 => format!("cdv {}", format_divisor(operand)),
            _ => panic!("Invalid opcode"),
        };

        output.push_str(&format!("{:02x}: {}\n", pc, instruction));

        pc += 2;
    }

    output
}

fn format_divisor(operand: u8) -> String {
    match operand {
        0..=3 => format!("{}", 1 << operand),
        4 => "[2^a]".to_owned(),
        5 => "[2^b]".to_owned(),
        6 => "[2^c]".to_owned(),
        _ => panic!("Invalid combo operand"),
    }
}

fn format_combo_operand(operand: u8) -> String {
    match operand {
        0..=3 => operand.to_string(),
        4 => "a".to_owned(),
        5 => "b".to_owned(),
        6 => "c".to_owned(),
        _ => panic!("Invalid combo operand"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MachineState {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
}

struct Input {
    initial_state: MachineState,
    program: Vec<u8>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let a = scan_fmt_some!(lines.next().unwrap(), "Register A: {d}", u64);
    let b = scan_fmt_some!(lines.next().unwrap(), "Register B: {d}", u64);
    let c = scan_fmt_some!(lines.next().unwrap(), "Register C: {d}", u64);
    lines.next().unwrap();
    let text = scan_fmt_some!(lines.next().unwrap(), "Program: {s}", String).unwrap();
    let program_text = text.split(",").map(|x| x.parse::<u8>().unwrap()).collect();

    Input {
        initial_state: MachineState {
            a: a.unwrap(),
            b: b.unwrap(),
            c: c.unwrap(),
            pc: 0,
        },
        program: program_text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(input.initial_state.a, 729);
        assert_eq!(input.initial_state.b, 0);
        assert_eq!(input.initial_state.c, 0);
        assert_eq!(input.program, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
