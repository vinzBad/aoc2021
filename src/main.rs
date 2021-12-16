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
    ptype:PacketType
}

#[derive(Debug)]
enum PacketType {
    Value (u32),
    Operator {
        operator_id: u8,
        packets:Vec<Packet>
     }
}


const INPUT: &str = include_str!("day16/test_input2.txt");

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




    println!("{}\n", result_viz);

    let result = parse_packets(&bits);
    dbg!(&result);
    let mut sum = 0;
    for r in result {
        sum += r.version;
    }
    dbg!(sum);
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
                println!("got value packet at {}", idx);
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
                println!("\t its last 4bits stop at {}", &idx);
                // move to next 4bit boundary
                idx = idx + (4 - (idx % 4));
                println!("\t padded to to next boundary it's {}", &idx);

            }
            _ => {
                println!("got operator packet at {}", idx);
                let length_type = msg[idx];
                idx += 1;
                if length_type { // next 11 bits are a number that represents the number of sub-packets immediately contained
                    let num_packets = parse_bits(msg, idx, 11);
                    println!("\t its encoding the number of packets to {}", &num_packets);
                    idx+=11;

                    let payload = get_sub_msg(msg, idx, 8);

                    result.push(Packet{
                        version:version as u8,
                        type_id: type_id as u8,
                        payload: payload
                    });

                } else { // next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
                    let length_packets = parse_bits(msg, idx, 15);
                    println!("\t its encoding the length of packets to {}", &length_packets);
                    idx += 15;
                    let payload = get_sub_msg(msg, idx, 8);

                    result.push(Packet{
                        version:version as u8,
                        type_id: type_id as u8,
                        payload: payload
                    });
                }
            }
        }
        println!("finished parsing a packet at {}\n", idx);


        if idx >= msg.len() {
            break;
        }
    }

    result
}


fn get_sub_msg(msg:&BitVec, from:usize, count:usize) -> BitVec {
    let mut result = BitVec::with_capacity(count);

    for idx in from..from + count {
        result.push(msg[idx]);
    }

    return result;
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
