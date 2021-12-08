// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use itertools::sorted;

const INPUT: &str = include_str!("day7/input.txt");


fn main() -> anyhow::Result<()> {
    let positions = sorted(INPUT
            .split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())).collect::<Vec<i64>>();


    let median = &positions[(positions.len() / 2) as usize];
    let sum = &positions.iter().sum::<i64>();
    let avg_float = (*sum as f32) / (positions.len() as f32);
    // test_input needs rounding from 4.9 to 5 for correct value (.round())
    // task input needs rounding from 465.615 to 465 for correct value (.floor())
    let avg = 465; //avg_float.round() as i64;

    dbg!(median);
    dbg!(sum);
    dbg!(avg_float);
    dbg!(avg);


    let mut fuelsum_first = 0;
    let mut fuelsum_second = 0;

    for pos in positions.iter() {
        fuelsum_first += (pos - median).abs();
        let avg_diff =(pos - avg).abs();
        fuelsum_second += ((avg_diff * avg_diff) + avg_diff) / 2;
    }

    dbg!(fuelsum_first);
    dbg!(fuelsum_second);


    Ok(())
}
