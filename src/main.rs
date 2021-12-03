// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!

use std::collections::HashSet;

const INPUT: &str = include_str!("day3/input.txt");

fn main() -> anyhow::Result<()> {
    let report =
        INPUT
            .split('\n')
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>()

    ;


    let (g,e) = calculate_gamma_epsilon(&report);
    let _ = dbg!(g*e);

    let (o2,co2) = calculate_o2_co2_rating(&report);
    let _ = dbg!(o2*co2);
    Ok(())
}


fn calculate_o2_co2_rating(report: &Vec<&str>) -> (i64, i64){
    if report.len() < 1 {
        panic!("Report must be longer than 0");
    }

    let column_count = report[0].len();

    let mut co2_value = 0;
    let mut o2_value = 0;

    let mut o2_indexes = HashSet::new();
    let mut co2_indexes = HashSet::new();

    for row_idx in 0..report.len() {
        o2_indexes.insert(row_idx);
        co2_indexes.insert(row_idx);
    }

    for col_idx in 0..column_count {
        let mut o2_count_one = 0;
        let mut o2_count_zero = 0;

        let mut co2_count_one = 0;
        let mut co2_count_zero = 0;
        // analyze
        for row_idx in 0..report.len() {
            let row = report[row_idx];
            let chars = row.chars().collect::<Vec<char>>();

            if co2_indexes.contains(&row_idx) {
                match chars[col_idx] {
                    '0' => co2_count_zero += 1,
                    '1' => co2_count_one += 1,
                    _ => panic!("dunno")
                }
            }

            if o2_indexes.contains(&row_idx) {
                match chars[col_idx] {
                    '0' => o2_count_zero += 1,
                    '1' => o2_count_one += 1,
                    _ => panic!("dunno")
                }
            }
        }

        dbg!(col_idx);
        dbg!((o2_count_one, o2_count_zero));
        dbg!((co2_count_one, co2_count_zero));
        // collect
        for row_idx in 0..report.len() {
            let row = report[row_idx];
            let chars = row.chars().collect::<Vec<char>>();
            let is_one;


            match chars[col_idx] {
                '0' => is_one=false,
                '1' => is_one=true,
                _ => panic!("dunno")
            }


            if o2_count_one >= o2_count_zero {            // more or equal 1s than 0s -> want to keep 1s
                if is_one {
                    // keep this number
                }
                else {
                    o2_indexes.remove(&row_idx);
                }

            } else {  // less 1s than 0s -> want to keep 0s
                if is_one {
                    o2_indexes.remove(&row_idx);
                }
                else {
                    // keep this number
                }
            }


            if co2_count_zero <= co2_count_one {            // less or equal 0s than 1s -> want to keep 0s
                if is_one {
                    co2_indexes.remove(&row_idx);
                }
                else {
                    // keep this number
                }

            } else {  // more 0s than 1s -> want to keep 1s
                if is_one {
                    // keep this number
                }
                else {
                    co2_indexes.remove(&row_idx);
                }
            }
        }

        if co2_indexes.len() == 1 {
            let row_idx = co2_indexes.iter().collect::<Vec<&usize>>()[0].clone();
            co2_value = i64::from_str_radix(report[row_idx], 2).unwrap();
        }
        if o2_indexes.len() == 1 {
            let row_idx = o2_indexes.iter().collect::<Vec<&usize>>()[0].clone();
            o2_value = i64::from_str_radix(report[row_idx], 2).unwrap();
        }
    }

    return(co2_value,o2_value)
}

fn calculate_gamma_epsilon(report: &Vec<&str>) -> (i64, i64){
    if report.len() < 1 {
        panic!("Report must be longer than 0");
    }


    let mut column_counts:Vec<i64> = Vec::new();

    for _ in 0..report[0].len(){
        column_counts.push(0);
    }

    for row in report.iter()    {
        let chars = row.chars().collect::<Vec<char>>();
        for char_idx in 0..chars.len() {
            match chars[char_idx] {
                '0' => {},
                '1' => column_counts[char_idx] += 1,
                _ => panic!("dunno")
            }
        }
    }

    // this is what happens when you don't finish your compsci studies folks...

    let mut gamma_str = String::from("");
    let mut epsilon_str = String::from("");

    // it's a bit embarassing but it works
    for count in column_counts.iter(){
        if count > &((report.len() as i64) / 2){
            gamma_str.push_str("1");
            epsilon_str.push_str("0");
        }
        else {
            gamma_str.push_str("0");
            epsilon_str.push_str("1");
        }
    }

    // yes I should just `NOT` all bits, but I'm on a deadline

    match  (i64::from_str_radix(&gamma_str, 2), i64::from_str_radix(&epsilon_str, 2)) {
        (Ok(g), Ok(e)) => return (g, e),
        _ => panic!("can't parse {} or {}", gamma_str, epsilon_str)
    }
}
