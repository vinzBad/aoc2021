// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!

use std::collections::HashMap;
use std::collections::HashSet;

use indicatif::ProgressBar;
use bit_vec::BitVec;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    payload:BitVec
}


const INPUT: &str = include_str!("day16/test_input5.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT
        .split('\n')
        .filter(|s| s.len() > 0)
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let mut bits = BitVec::with_capacity(input.len() * 4);

    for hex in input.iter() {
        let bin = hex.to_digit(16).unwrap() as u8;

        bits.push(
            0 != (bin & (1 << 3))
        );
        bits.push(
            0 != (bin & (1 << 2))
        );
        bits.push(
            0 != (bin & (1 << 1))
        );
        bits.push(
            0 != (bin & (1 << 0))
        );
    }

    let mut result_viz = String::from("");

    for bit in bits.iter() {
        if bit {
            result_viz += "1"
        } else {
            result_viz += "0"
        }
    }




    println!("{}", result_viz);

    dbg!(parse_packets(&bits));
    Ok(())

}


fn parse_packets(msg:&BitVec) -> Vec<Packet> {
    let mut result = Vec::new();
    let mut idx = 0;
    loop {

        let version = parse_bits(msg, idx, 3);
        let type_id = parse_bits(msg, idx+3, 3);
        idx += 6;


        match type_id {
            4 => {
                let mut value_bits = BitVec::new();
                loop {
                    let is_last_group = ! msg[idx];
                    for i in 1..5 {
                        value_bits.push(msg[idx + i])
                    }

                    idx += 5;

                    if is_last_group {
                        break;
                    }

                }

                result.push(Packet{
                    version:version as u8,
                    type_id: type_id as u8,
                    payload: value_bits
                });

            }
            _ => {

            }
        }
        // move to next 4bit boundary
        idx = idx + (4 - (idx % 4));

        if idx >= msg.len() {
            break;
        }
    }

    result
}




fn parse_bits(msg:&BitVec, from:usize, count:usize ) -> u32 {
    let mut val = String::from("");
    for i in from..from+count {
        if msg[i] {
            val += "1"
        } else {
            val += "0"
        }
    }
    return u32::from_str_radix(&val, 2).unwrap()
}
