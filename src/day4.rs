// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use regex::Regex;
use std::collections::HashSet;

const INPUT: &str = include_str!("day4/input.txt");

// Board:
// 10x HashSet of Columns and Rows
// 5x5 array of int64
// Logic
// Loop over bingo input -> add to HashSet -> find Board Hashset that is_subset of Input Hashset

#[derive(Debug)]
struct Board {
    rows_and_columns: Vec<HashSet<i64>>,
    data: [i64; 5*5]
}



fn main() -> anyhow::Result<()> {
    let mut board_data =
        INPUT
            .split('\n')
            .filter(|s| s.len() > 0);

    let mut boards: Vec<Board> = Vec::new();

    let mut input_vec: Vec<i64> = Vec::new();


    // 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
    for input_str in board_data.next().unwrap().split(',') {
        input_vec.push(input_str.parse::<i64>().unwrap());
    }

    // 22 13 17 11  0
    // 8  2 23  4 24
    // 21  9 14 16  7
    // 6 10  3 18  5
    // 1 12 20 15 19
    let re = Regex::new(r"(\d+)").unwrap();
    let mut finished_parse = false;
    loop {
        let mut rows_and_columns:Vec<HashSet<i64>> = Vec::with_capacity(10);
        for _ in 0..10 {
            rows_and_columns.push(HashSet::new());
        }
        let mut data: [i64; 5*5] = [-1; 5*5];

        for row_idx in 0..5 {
            let row;
            match board_data.next(){
                Some(v) => row = v,
                None => {
                    finished_parse = true;
                    break
                }
            }
            let nums = re.captures_iter(row)
                .map(|c| c.get(0).unwrap().as_str().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            for col_idx in 0..5 {
                let num = nums[col_idx];
                data[row_idx * 5 + col_idx] = num;
                rows_and_columns[row_idx].insert(num);
                rows_and_columns[5 + col_idx].insert(num);
            }
        }
        if data[24] != -1 {
            boards.push(
                Board{
                    rows_and_columns: rows_and_columns,
                    data: data
                }
            );
        }

        if finished_parse {
            break;
        }
    }

    let mut input_hash: HashSet<i64> = HashSet::new();
    let mut input_hash_first_winning_board: HashSet<i64> = HashSet::new();
    let mut input_hash_last_winning_board: HashSet<i64> = HashSet::new();
    let mut found_winning_board = false;
    let mut winning_board_index:Option<usize> = Option::None;
    let mut last_winning_board_index:Option<usize> = Option::None;
    let mut winning_boards: HashSet<usize> = HashSet::new();
    let mut last_number;
    let mut last_number_first_winning_board = -1;
    let mut last_number_last_winning_board = -1;
    for input_num in input_vec.iter() {
        input_hash.insert(*input_num);
        last_number = *input_num;
        for board_idx in 0..boards.len() {
            if winning_boards.contains(&board_idx) {
                continue;
            }
            let board = &boards[board_idx];
            for row_or_column in board.rows_and_columns.iter() {
                if row_or_column.is_subset(&input_hash) {
                    winning_boards.insert(board_idx);
                    last_winning_board_index = Option::Some(board_idx);
                    last_number_last_winning_board = last_number;
                    input_hash_last_winning_board = input_hash.clone();
                    if !found_winning_board {
                        found_winning_board = true;
                        input_hash_first_winning_board = input_hash.clone();
                        last_number_first_winning_board = last_number;
                        winning_board_index = Option::Some(board_idx);
                    }
                    break;
                }
            }
        }
    }


    dbg!(winning_board_index);
    dbg!(last_winning_board_index);

    match winning_board_index {
        Some(idx) => {
            let board = &boards[idx];
            let mut unmarked_sum = 0;
            for num_idx in 0..25 {
                if input_hash_first_winning_board.contains(&board.data[num_idx]) {
                    // number is marked, ignore it
                }
                else {
                    unmarked_sum += &board.data[num_idx];
                }
            }
            dbg!(unmarked_sum * last_number_first_winning_board);
        }
        None => {dbg!("no winning board :(");}
    }

    match last_winning_board_index {
        Some(idx) => {
            let board = &boards[idx];
            let mut unmarked_sum = 0;
            for num_idx in 0..25 {
                if input_hash_last_winning_board.contains(&board.data[num_idx]) {
                    // number is marked, ignore it
                }
                else {
                    unmarked_sum += &board.data[num_idx];
                }
            }
            dbg!(unmarked_sum * last_number_last_winning_board);
        }
        None => {dbg!("no winning board :(");}
    }



    Ok(())
}
