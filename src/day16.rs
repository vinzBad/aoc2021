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
    Value (u64),
    Operator {
        packets:Vec<Packet>
     }
}


const INPUT: &str = include_str!("day16/input.txt");
//const INPUT: &str = "9C0141080250320F1802104A08";

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

    let (result, bits_read) = parse_packets(&bits, 0, bits.len(), false, 0);
    dbg!(&result, &bits_read);



    dbg!(sum_packet_versions(&result));
    dbg!(eval(&result[0]));
    Ok(())

}

fn sum_packet_versions(packets:&Vec<Packet>) -> u32 {
    let mut sum = 0;

    for p in packets {
        match &p.ptype {
            PacketType::Value(_) => sum += p.version as u32,
            PacketType::Operator{packets} => sum += (p.version as u32) + sum_packet_versions(&packets),
        }
    }

    return sum
}


fn eval(packet:&Packet) -> u64 {
    match &packet.ptype {
            PacketType::Value(v) => return *v,
            PacketType::Operator{packets} => return op(packet.type_id, &packets),
        }
    }


fn op(type_id:u8, packets:&Vec<Packet> ) -> u64 {
    match type_id {
        0 => return sum(packets),
        1 => return product(packets),
        2 => return minimum(packets),
        3 => return maximum(packets),
        5 => return greater(packets),
        6 => return lesser(packets),
        7 => return equal(packets),
        _ => panic!("unknown type_id")
    }
}


fn sum(packets:&Vec<Packet>) -> u64 {
    let mut result = 0;
    for p in packets {
        result += eval(p);
    }
    result
}

fn product(packets:&Vec<Packet>) -> u64 {
    let mut result = 1;

    for p in packets {
        result *= eval(p)
    }
    result
}

fn minimum(packets:&Vec<Packet>) -> u64 {
    let mut result = std::u64::MAX;

    for p in packets {
        let val = eval(p);
        if val < result {
            result = val
        }
    }
    result
}

fn maximum(packets:&Vec<Packet>) -> u64 {
    let mut result = std::u64::MIN;

    for p in packets {
        let val = eval(p);
        if val > result {
            result = val
        }
    }
    result
}


fn greater(packets:&Vec<Packet>) -> u64 {
    assert_eq!(2, packets.len());

    let val_a = eval(&packets[0]);
    let val_b = eval(&packets[1]);

    if val_a > val_b {
        return 1;
    }

    return 0;
}

fn lesser(packets:&Vec<Packet>) -> u64 {
    assert_eq!(2, packets.len());

    let val_a = eval(&packets[0]);
    let val_b = eval(&packets[1]);

    if val_a < val_b {
        return 1;
    }

    return 0;
}

fn equal(packets:&Vec<Packet>) -> u64 {
    assert_eq!(2, packets.len());

    let val_a = eval(&packets[0]);
    let val_b = eval(&packets[1]);

    if val_a == val_b {
        return 1;
    }

    return 0;
}


fn parse_packets(msg:&BitVec, from:usize, count:usize, is_packet_count:bool, depth:u32) -> (Vec<Packet>, usize) {
    let mut result = Vec::new();
    let mut idx = from;
    let mut packet_count=0;
    let mut prefix = String::from("");
    for _ in 0..depth {
        prefix += "\t"
    }
    loop {
        let start_idx = idx;

        if !is_packet_count {
            let remaining_bits = count - (idx-from);
            if remaining_bits < 11 {
                println!("{} {} bits remaining unable to encode packet, stopping parse", prefix, remaining_bits );
                break;
            }
        }


        let version = parse_bits(msg, idx, 3);
        let type_id = parse_bits(msg, idx+3, 3);
        idx += 6;



        match type_id {
            4 => {
                println!("{} got value packet at {}", prefix, start_idx);
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
                let val = parse_bits(&value_bits, 0, value_bits.len());
                result.push(Packet{
                    version:version as u8,
                    type_id: type_id as u8,
                    ptype: PacketType::Value(val)
                });
                println!("{}\t with a value of {}", prefix, &val);
                println!("{}\t its last 4bits stop at {}", prefix, &idx);
                // move to next 4bit boundary
                //idx = idx + (4 - (idx % 4));
                //println!("{}\t padded to to next boundary it's {}", prefix, &idx);

            }
            _ => {
                println!("{}got operator packet at {}", prefix, start_idx);
                let length_type = msg[idx];
                idx += 1;
                if length_type { // next 11 bits are a number that represents the number of sub-packets immediately contained
                    let num_packets = parse_bits(msg, idx, 11) as usize;
                    println!("{}\t its encoding the number of packets to {}", prefix, &num_packets);
                    idx+=11;
                    let (packets, bits_read) = parse_packets(&msg, idx, num_packets, true, depth + 1);
                    idx += bits_read;
                    result.push(Packet{
                        version:version as u8,
                        type_id: type_id as u8,
                        ptype: PacketType::Operator{
                            packets: packets
                        }
                    });

                } else { // next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
                    let length_packets = parse_bits(msg, idx, 15) as usize;
                    println!("{}\t its encoding the length of packets to {}", prefix, &length_packets);
                    idx += 15;

                    let (packets, bits_read) = parse_packets(&msg, idx, length_packets, false, depth + 1);
                    idx += bits_read;

                    result.push(Packet{
                        version:version as u8,
                        type_id: type_id as u8,
                        ptype: PacketType::Operator{
                            packets: packets
                        }
                    });
                }
            }
        }

        println!("{} finished parsing a packet from {} to {}:{}bits\n", prefix, start_idx, idx, idx-start_idx);
        packet_count += 1;

        if is_packet_count {
            println!("{} its {} of {} expected packets\n", prefix, packet_count, count);
            if packet_count >= count {
                break;
            }
        } else {
            println!("{} its {} of {} expected bits\n", prefix, idx - from, count);
            if idx >= from + count {
                break;
            }
        }
    }

    (result, idx - from)
}

fn parse_bits(msg:&BitVec, from:usize, count:usize ) -> u64 {
    let mut val = String::from("");
    for i in from..from+count {
        if msg[i] {
            val += "1"
        } else {
            val += "0"
        }
    }
    return u64::from_str_radix(&val, 2).unwrap()
}
