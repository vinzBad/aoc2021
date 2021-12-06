// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!

const INPUT: &str = include_str!("day6/input.txt");


fn main() -> anyhow::Result<()> {
    let  population = INPUT
            .split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect::<Vec<i64>>();

    //dbg!(&population);

    let mut popcount: [i64;9] = [0;9];



    for pop in population.iter() {
        popcount[*pop as usize] += 1;
    }

    dbg!(popcount);

    for day in 0..256 {
        let prev_popcounts = popcount.clone();

        popcount[0] = prev_popcounts[1]; // 1 d
        popcount[1] = prev_popcounts[2];
        popcount[2] = prev_popcounts[3];
        popcount[3] = prev_popcounts[4];
        popcount[4] = prev_popcounts[5];
        popcount[5] = prev_popcounts[6];
        popcount[6] = prev_popcounts[7] + prev_popcounts[0];
        popcount[7] = prev_popcounts[8];
        popcount[8] = prev_popcounts[0];

        println!("day {}", day);
        for idx in 0..popcount.len() {
            println!("{}:{}", idx, popcount[idx]);
        }
        println!();
    }

   // dbg!(popcount);
    dbg!(popcount.iter().sum::<i64>());

    Ok(())
}
