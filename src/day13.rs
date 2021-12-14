// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("day13/input.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT.split('\n').filter(|s| s.len() > 0);

    let re = Regex::new(r"fold along (\w)=(\d+)").unwrap();

    let mut map = [false; 2000 * 2000];
    let mut folds: Vec<(String, i64)> = Vec::new();

    let mut row_count = 0;
    let mut col_count = 0;

    for line in input {
        match line.split_once(',') {
            Some((row, col)) => {
                let col_idx = row.parse::<i64>().unwrap();
                let row_idx = col.parse::<i64>().unwrap();

                if row_idx > row_count {
                    row_count = row_idx;
                }

                if col_idx > col_count {
                    col_count = col_idx;
                }

                map[(row_idx * 2000 + col_idx) as usize] = true;
            }
            None => match re.captures(line) {
                Some(caps) => {
                    folds.push((caps[1].to_owned(), caps[2].parse::<i64>().unwrap()));
                }
                None => panic!("invalid input line {}", line),
            },
        }
    }
    row_count += 1;
    col_count += 1;

    let mut map_vis = String::from("");
    let mut dot_count = 0;
    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            if map[(row_idx * 2000 + col_idx) as usize] {
                map_vis += "#";
                dot_count+=1;
            } else {
                map_vis += ".";
            }
        }
        map_vis += "\n";
    }

    //println!("{}", map_vis);

    for (axis, value) in folds {
        if axis == "x" {
            println!("doing x fold at {}", value);
            for row_idx in 0..row_count {
                for col_idx in value + 1..col_count {
                    if map[(row_idx * 2000 + col_idx) as usize] {
                        let left_col_idx = col_idx - ((col_idx - value) * 2);
                        //println!("moving {}, {} to {},{}", row_idx, col_idx, row_idx, left_col_idx);
                        map[(row_idx * 2000 + left_col_idx) as usize] = true;
                    }
                }
            }
            col_count = value;
        } else if axis == "y" {
            println!("doing y fold at {}", value);
            for row_idx in value + 1..row_count {
                for col_idx in 0..col_count {
                    if map[(row_idx * 2000 + col_idx) as usize] {
                        let upper_row_idx = row_idx - ((row_idx - value) * 2);
                        //println!("moving {}, {} to {},{}", row_idx, col_idx, upper_row_idx, col_idx);
                        map[(upper_row_idx * 2000 + col_idx) as usize] = true;
                    }
                }
            }
            row_count = value;
        } else {
            panic!("unkown axis {}", axis);
        }
        let mut map_vis = String::from("");
        dot_count = 0;
        for row_idx in 0..row_count {
            for col_idx in 0..col_count {
                if map[(row_idx * 2000 + col_idx) as usize] {
                    map_vis += "#";
                    dot_count+=1;
                } else {
                    map_vis += ".";
                }
            }
            map_vis += "\n";
        }

        println!("{}", map_vis);
        println!("dots: {}", dot_count);

    }

    dbg!(dot_count);
    Ok(())
}
