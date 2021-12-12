// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use std::collections::HashSet;

const INPUT: &str = include_str!("day11/input.txt");

fn main() -> anyhow::Result<()> {
    let mut input = INPUT
        .split('\n')
        .filter_map(|s| parse_input_line(s))
        .collect::<Vec<Vec<i64>>>();

    let neighbor_offsets = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let steps = 1000;
    let mut flash_count = 0;

    let row_count = input.len() as i64;
    let column_count = input[0].len() as i64;

    dbg!(row_count, column_count);
    let in_bounds = |row: i64, col: i64| -> bool {
        return (row >= 0 && row < row_count) && (col >= 0 && col < column_count);
    };

    for step_idx in 0..steps {
        let mut flashers: HashSet<(i64, i64)> = HashSet::new();
        // let mut map = String::from("");
        for row in 0..row_count {
            for col in 0..column_count {
                input[row as usize][col as usize] += 1;
            }
        }
        loop {
            let mut did_flash = false;

            for row in 0..row_count {
                for col in 0..column_count {
                    let point = (row, col);
                    let energy = input[row as usize][col as usize];

                    if flashers.contains(&point) {
                        continue;
                    }

                    if energy > 9 {
                        flashers.insert(point);
                        flash_count += 1;

                        did_flash = true;

                        for (row_offset, col_offset) in neighbor_offsets.iter() {
                            if !in_bounds(row + row_offset, col + col_offset) {
                                continue;
                            }
                            input[(row + row_offset) as usize][(col + col_offset) as usize] += 1;
                        }
                    }
                }
            }
            if !did_flash {
                break;
            }
        }
        if flashers.len() == (row_count * column_count) as usize {
            dbg!(step_idx +1);
            break;
        }

        for flash in flashers.drain() {
            let (row, col) = flash;
            input[row as usize][col as usize] = 0;
        }

        // for row in 0..row_count {
        //     for col in 0..column_count {
        //         map += &input[row as usize][col as usize].to_string();
        //     }
        //     map += "\n";
        // }

        // println!("{}", map);
    }

    dbg!(flash_count);

    Ok(())
}

fn parse_input_line(s: &str) -> Option<Vec<i64>> {
    if s.trim().len() == 0 {
        return None;
    }
    let mut result = Vec::new();

    for ch in s.chars() {
        match ch.to_string().parse::<i64>() {
            Ok(num) => result.push(num),
            Err(_) => return None,
        }
    }

    Some(result)
}
