// most of this code is taken from https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
fn main() -> anyhow::Result<()> {
    let s = count_number_of_increases(
        include_str!("day1/input.txt")
            .split('\n')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    );

    dbg!(s);

    let s2 = count_number_of_increases_in_windows(
        include_str!("day1/input.txt")
            .split('\n')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    );

    dbg!(s2);

    Ok(())
}


fn count_number_of_increases(s: Vec<i64>) -> i64 {
    let mut previous_num = std::i64::MAX;
    let mut increases = 0;
    for i in 0..s.len() {
        let num = s[i];
        if num > previous_num{
            increases += 1;
        }
        previous_num = num
    }
    return increases;
}

fn count_number_of_increases_in_windows(s: Vec<i64>) -> i64 {
    let mut previous_num = std::i64::MAX;
    let mut increases = 0;
    for i in 2..s.len() {

        let num = s[i] + s[i-1] + s[i-2];
        if num > previous_num{
            increases += 1;
        }
        previous_num = num
    }
    return increases;
}
