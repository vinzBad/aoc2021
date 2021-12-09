// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use std::collections::HashSet;

const INPUT: &str = include_str!("day9/input.txt");

type Point = (i64, i64);

fn main() -> anyhow::Result<()> {
    let input = INPUT
            .split('\n')
            .filter_map(|s| parse_input_line(s))
            .collect::<Vec<Vec<i64>>>();


    let row_count = input.len() as i64;
    let column_count = input[0].len() as i64;
    let mut risk_level = 0;
    let neighbor_offset = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut basins:Vec<HashSet<Point>> = Vec::new();

    dbg!(row_count, column_count);
    let in_bounds = |row:i64, col:i64| -> bool {
        return (row >= 0 && row < row_count) && (col >= 0 && col < column_count);
    };
    for row_idx in 0..row_count {
        for col_idx in 0..column_count {
            let height = input[row_idx as usize][col_idx as usize];
            let mut is_low_point = false;
            for (row_offset, col_offset) in neighbor_offset.iter() {
                if !in_bounds(row_idx + row_offset, col_idx + col_offset) {
                    continue;
                }

                if height < input[(row_idx + row_offset) as usize][(col_idx + col_offset) as usize] {
                    is_low_point = true;
                } else {
                    is_low_point = false;
                    break;
                }
            }
            if is_low_point {
                risk_level += height + 1;

                let mut basin:HashSet<Point> = HashSet::new();
                let mut candidates:Vec<Point> = vec![(row_idx, col_idx)];
                loop {
                    let candidate_count = candidates.len();
                    for _ in 0..candidate_count {
                        let (candidate_row, candidate_col) = candidates.pop().unwrap();
                        basin.insert((candidate_row, candidate_col));

                        let candidate_height = input[candidate_row as usize][candidate_col as usize];

                        for (row_offset, col_offset) in neighbor_offset.iter() {
                            let neighbor_row = candidate_row + row_offset;
                            let neighbor_col = candidate_col + col_offset;

                            if !in_bounds(neighbor_row, neighbor_col) {
                                continue;
                            }

                            let neighbor_height = input[neighbor_row as usize][neighbor_col as usize];

                            if neighbor_height == 9 {
                                continue;
                            }

                            dbg!(&neighbor_row, &neighbor_col);
                            dbg!(&neighbor_height);

                            if neighbor_height < candidate_height {
                                continue;
                            }

                            if basin.contains(&(neighbor_row, neighbor_col)) {
                                continue;
                            }

                            for existing_basin in basins.iter() {
                                if existing_basin.contains(&(neighbor_row, neighbor_col)) {
                                    continue;
                                }
                            }

                            candidates.push((neighbor_row, neighbor_col));
                        }
                    }
                    if candidates.len() == 0 {
                        basins.push(basin);
                        dbg!("basin complete");
                        break
                    }
                }
            }
        }
    }

    dbg!(risk_level);
    dbg!(basins.len());
    let mut basin_sizes = basins
        .iter()
        .map(|b| b.len())
        .collect::<Vec<usize>>();
    basin_sizes.sort();


    dbg!(basin_sizes[basin_sizes.len()-3] * basin_sizes[basin_sizes.len()-2] * basin_sizes[basin_sizes.len()-1]);

    // for basin in basins.iter() {
    //     let mut output = String::from("");
    //     for row_idx in 0..row_count {
    //         for col_idx in 0..column_count {
    //             if basin.contains(&(row_idx, col_idx)) {
    //                 output += &input[row_idx as usize][col_idx as usize].to_string()
    //             } else {
    //                 output += ".";
    //             }

    //         }
    //         output+="\n"
    //     }
    //     println!("{}",output);
    // }

    Ok(())
}


fn parse_input_line(s:&str) -> Option<Vec<i64>> {
    if s.trim().len() == 0 {
        return None;
    }
    let mut result = Vec::new();

    for ch in s.chars() {
        match ch.to_string().parse::<i64>(){
            Ok(num) => result.push(num),
            Err(_) => return None
        }
    }

    Some(result)
}
