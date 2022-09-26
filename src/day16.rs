use color_eyre::eyre::{eyre, Result};
use num::{NumCast, PrimInt};
use std::{collections::HashMap, fmt::Display, fs};

fn bits_to_num<T: PrimInt>(val: &[char]) -> T {
    let mut ret = 0;
    for (i, x) in val.into_iter().rev().enumerate() {
        ret += (2u64).pow(i as u32) * (*x as u64 - 0x30);
    }
    NumCast::from(ret).expect(format!("{:?}", ret).as_str())
}

#[derive(Debug)]
enum Packetable {
    Operator(OperatorPacket),
    Literal(String),
}

#[derive(Debug)]
struct OperatorPacket {
    packets: Vec<Packet>,
    length_type_id: char,
}

impl OperatorPacket {
    fn new(bits: &[char]) -> Self {
        let length_type_id = bits[0];
        if length_type_id == '0' {
            let length_bits = &bits[1..16];
            let length: u32 = bits_to_num(length_bits);
            // dbg!(length);

            let mut packets = Vec::new();
            let mut total_bits = 0;
            while total_bits < length {
                let packet = Packet::new(&bits[(16 + total_bits as usize)..]);
                total_bits += packet.total_bits();
                packets.push(packet);
            }
            assert!(total_bits == length);
            return OperatorPacket {
                packets,
                length_type_id,
            };
        } else {
            let num_packets_bits = &bits[1..12];
            let mut num_packets: u32 = bits_to_num(num_packets_bits);
            // dbg!(num_packets);

            let mut packets = Vec::new();
            let mut total_bits = 0;
            while num_packets > 0 {
                let packet = Packet::new(&bits[(12 + total_bits as usize)..]);
                total_bits += packet.total_bits();
                packets.push(packet);
                num_packets -= 1;
            }
            return OperatorPacket {
                packets,
                length_type_id,
            };
        }
    }

    fn total_bits(&self) -> u32 {
        self.packets.iter().map(|p| p.total_bits()).sum::<u32>()
            + if self.length_type_id == '0' { 16 } else { 12 }
    }

    fn eval(&self, type_id: u8) -> u64 {
        let mut evals = self.packets.iter().map(|p| p.eval());
        match type_id {
            0 => evals.sum(),
            1 => evals.product(),
            2 => evals.min().unwrap(),
            3 => evals.max().unwrap(),
            // 4 => ,
            5 => {
                if evals.next().unwrap() > evals.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if evals.next().unwrap() < evals.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if evals.next().unwrap() == evals.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    body: Packetable,
}

impl Packet {
    fn new(bits: &[char]) -> Self {
        // println!("{:?}", bits.iter().collect::<String>());
        let version = bits_to_num(&bits[0..3]);
        let type_id = bits_to_num(&bits[3..6]);
        // dbg!(version, type_id);
        if type_id == 4 {
            let mut string = String::new();
            for window in bits[6..].chunks_exact(5) {
                string.push_str(window[1..].iter().collect::<String>().as_str());
                if window[0] == '0' {
                    break;
                }
            }
            Packet {
                version,
                type_id,
                body: Packetable::Literal(string),
            }
        } else {
            Packet {
                version,
                type_id,
                body: Packetable::Operator(OperatorPacket::new(&bits[6..])),
            }
        }
    }

    fn total_bits(&self) -> u32 {
        6 + match &self.body {
            Packetable::Operator(operator) => operator.total_bits(),
            Packetable::Literal(l) => l.len() as u32 / 4 * 5,
        }
    }

    fn version_sum(&self) -> u32 {
        self.version as u32
            + match &self.body {
                Packetable::Operator(operator) => {
                    operator.packets.iter().map(|p| p.version_sum()).sum()
                }
                Packetable::Literal(_) => 0,
            }
    }

    fn eval(&self) -> u64 {
        match &self.body {
            Packetable::Operator(operator) => operator.eval(self.type_id),
            Packetable::Literal(literal) => bits_to_num(&literal.chars().collect::<Vec<_>>()[..]),
        }
    }
}

fn get_input() -> Result<Packet> {
    let contents = fs::read_to_string("inputs/day16.txt").expect("");
    let mut lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let bits = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    Ok(Packet::new(
        &lines
            .next()
            .unwrap()
            .chars()
            .map(|v| *bits.get(&v).unwrap())
            .collect::<String>()
            .chars()
            .collect::<Vec<char>>()[..],
    ))
}

fn solve_p1() -> Result<()> {
    let packet = get_input()?;
    // dbg!(&bits);
    dbg!(packet.total_bits());
    dbg!(packet.version_sum());

    Ok(())
}

fn solve_p2() -> Result<()> {
    let packet = get_input()?;
    dbg!(packet.eval());

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
