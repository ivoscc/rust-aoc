use std::{
    collections::HashMap,
    fmt::{Display, Result},
    str::Chars,
};

use itertools::Itertools;

enum PacketBody {
    Literal(usize),
    OpSum(Vec<Packet>),
    OpProduct(Vec<Packet>),
    OpMinimum(Vec<Packet>),
    OpMaximum(Vec<Packet>),
    OpGreaterThan(Vec<Packet>),
    OpLessThan(Vec<Packet>),
    OpEqual(Vec<Packet>),
}

struct Packet {
    version: usize,
    type_id: usize,
    body: PacketBody,
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let body: String = match &self.body {
            PacketBody::Literal(literal) => format!("{}", literal),
            PacketBody::OpSum(subpackets)
            | PacketBody::OpProduct(subpackets)
            | PacketBody::OpMinimum(subpackets)
            | PacketBody::OpMaximum(subpackets)
            | PacketBody::OpGreaterThan(subpackets)
            | PacketBody::OpLessThan(subpackets)
            | PacketBody::OpEqual(subpackets) => {
                subpackets.into_iter().map(|s| format!("{}", s)).join(", ")
            }
        };
        write!(f, "P<v={},t={}>[{}]", self.version, self.type_id, body)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1012, output_part_1);
    assert_eq!(2223947372407, output_part_2);
}

fn parse_input(input: &str) -> String {
    let nibble_map = HashMap::<char, [char; 4]>::from_iter([
        ('0', ['0', '0', '0', '0']),
        ('1', ['0', '0', '0', '1']),
        ('2', ['0', '0', '1', '0']),
        ('3', ['0', '0', '1', '1']),
        ('4', ['0', '1', '0', '0']),
        ('5', ['0', '1', '0', '1']),
        ('6', ['0', '1', '1', '0']),
        ('7', ['0', '1', '1', '1']),
        ('8', ['1', '0', '0', '0']),
        ('9', ['1', '0', '0', '1']),
        ('A', ['1', '0', '1', '0']),
        ('B', ['1', '0', '1', '1']),
        ('C', ['1', '1', '0', '0']),
        ('D', ['1', '1', '0', '1']),
        ('E', ['1', '1', '1', '0']),
        ('F', ['1', '1', '1', '1']),
    ]);

    input
        .trim()
        .chars()
        .flat_map(|hex_digit| nibble_map.get(&hex_digit).unwrap())
        .collect()
}

fn binary_char_iterator_to_number<T>(chars: &mut T) -> usize
where
    T: Iterator<Item = char>,
{
    usize::from_str_radix(&chars.collect::<String>(), 2).unwrap()
}

fn parse_literal_value_packet_body(chars: &mut Chars) -> (PacketBody, usize) {
    let mut literal: Vec<char> = vec![];
    let mut consumed_count = 0;
    loop {
        let mut group = chars.take(5);
        consumed_count += 5;
        let last_group = Some('0') == group.next();
        literal.extend(group);
        if last_group {
            break;
        }
    }
    (
        PacketBody::Literal(binary_char_iterator_to_number(&mut literal.into_iter())),
        consumed_count,
    )
}

fn parse_multiple_packets_of_size(chars: &mut Chars, expected_size: usize) -> (Vec<Packet>, usize) {
    let mut consumed_size = 0;
    let mut packets = vec![];
    while consumed_size < expected_size {
        let (packet, packet_size) = parse_packet(chars);
        packets.push(packet);
        consumed_size += packet_size;
    }
    (packets, consumed_size)
}

fn parse_n_packets(chars: &mut Chars, expected_packets: usize) -> (Vec<Packet>, usize) {
    let mut packets = vec![];
    let mut consumed_size = 0;
    for _ in 0..expected_packets {
        let (packet, packet_size) = parse_packet(chars);
        consumed_size += packet_size;
        packets.push(packet)
    }
    (packets, consumed_size)
}

fn parse_operator_packet_body(type_id: usize, chars: &mut Chars) -> (PacketBody, usize) {
    let length_type_id = chars.next().unwrap();
    let mut total_consumed = 1;
    let (packets, inner_consumed) = if length_type_id == '0' {
        let total_subpacket_length = binary_char_iterator_to_number(&mut chars.take(15));
        total_consumed += 15;
        parse_multiple_packets_of_size(chars, total_subpacket_length)
    } else {
        let number_of_contained_subpackets = binary_char_iterator_to_number(&mut chars.take(11));
        total_consumed += 11;
        parse_n_packets(chars, number_of_contained_subpackets)
    };
    let body = match type_id {
        0 => PacketBody::OpSum(packets),
        1 => PacketBody::OpProduct(packets),
        2 => PacketBody::OpMinimum(packets),
        3 => PacketBody::OpMaximum(packets),
        5 => PacketBody::OpGreaterThan(packets),
        6 => PacketBody::OpLessThan(packets),
        7 => PacketBody::OpEqual(packets),
        _ => panic!("Unknown opearation"),
    };
    (body, total_consumed + inner_consumed)
}

fn parse_packet(chars: &mut Chars) -> (Packet, usize) {
    let version = binary_char_iterator_to_number(&mut chars.take(3));
    let type_id = binary_char_iterator_to_number(&mut chars.take(3));
    let (body, consumed_count) = match type_id {
        4 => parse_literal_value_packet_body(chars),
        _ => parse_operator_packet_body(type_id, chars),
    };
    (
        Packet {
            version,
            type_id,
            body,
        },
        consumed_count + 6,
    )
}

fn get_version_numbers_sum(packet: &Packet) -> usize {
    let mut version_sum: usize = packet.version;
    match &packet.body {
        PacketBody::OpSum(subpackets)
        | PacketBody::OpProduct(subpackets)
        | PacketBody::OpMinimum(subpackets)
        | PacketBody::OpMaximum(subpackets)
        | PacketBody::OpGreaterThan(subpackets)
        | PacketBody::OpLessThan(subpackets)
        | PacketBody::OpEqual(subpackets) => {
            version_sum += subpackets
                .iter()
                .map(|s| get_version_numbers_sum(&s))
                .sum::<usize>();
        }
        _ => {}
    }
    version_sum
}

fn part_1(input: &str) -> usize {
    let binary = parse_input(input);
    let (main_packet, _) = parse_packet(&mut binary.chars());
    get_version_numbers_sum(&main_packet)
}

fn execute(packet: &Packet) -> usize {
    match &packet.body {
        PacketBody::Literal(value) => *value,
        PacketBody::OpSum(packets) => packets.iter().map(|x| execute(x)).sum(),
        PacketBody::OpProduct(packets) => packets.iter().map(|x| execute(x)).product(),
        PacketBody::OpMinimum(packets) => packets.iter().map(|x| execute(x)).min().unwrap(),
        PacketBody::OpMaximum(packets) => packets.iter().map(|x| execute(x)).max().unwrap(),
        PacketBody::OpGreaterThan(packets) => {
            if execute(&packets[0]) > execute(&packets[1]) {
                1
            } else {
                0
            }
        }
        PacketBody::OpLessThan(packets) => {
            if execute(&packets[0]) < execute(&packets[1]) {
                1
            } else {
                0
            }
        }
        PacketBody::OpEqual(packets) => {
            if execute(&packets[0]) == execute(&packets[1]) {
                1
            } else {
                0
            }
        }
    }
}

fn part_2(input: &str) -> usize {
    let binary = parse_input(input);
    let (main_packet, _) = parse_packet(&mut binary.chars());
    execute(&main_packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(part_1("8A004A801A8002F478"), 16);
        assert_eq!(part_1("620080001611562C8802118E34"), 12);
        assert_eq!(part_1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_1("A0016C880162017C3686B18A3D4780"), 31);
    }
}
