// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("day14/input.txt");

fn main() -> anyhow::Result<()> {
    let mut input = INPUT.split('\n').filter(|s| s.len() > 0);

    let re = Regex::new(r"(\w\w) -> (\w)").unwrap();

    // NNCB

    // CH -> B
    // HH -> N

    let initial_state:Vec<char> = input.next().unwrap().chars().collect(); // yes!
    let mut pairs:HashMap<String, i64> = HashMap::new();
    for idx in 1..initial_state.len() {
        let pair = format!("{}{}", initial_state[idx-1], initial_state[idx]);
        let count = *pairs.entry(pair.clone()).or_insert(0);
        pairs.insert(pair, count + 1);
    }

    let mut freqs:HashMap<String, i64> = HashMap::new();
    for ch in initial_state {
        let elem = format!("{}", ch);
        let freq = *freqs.entry(elem.clone()).or_insert(0);
        freqs.insert(elem, freq + 1);
    }

    let mut rules:HashMap<String, String> = HashMap::new();

    for line in input {
        match re.captures(line) {
            None => {},
            Some(caps) => {
                rules.insert(caps[1].to_owned().to_string(), caps[2].to_owned().to_string());
            }
        }
    }


    let steps = 40;
    dbg!(&pairs);
    dbg!(&freqs);

    for _ in 0..steps {
        let mut new_pairs:HashMap<String, i64> = HashMap::new();


        for (pair, count) in pairs.iter() {
            let inserter = &rules[pair];

            let mut new_pair_1 = pair.clone();
            let mut new_pair_2 = new_pair_1.split_off(1);

            new_pair_1 = format!("{}{}", new_pair_1, inserter);
            new_pair_2 = format!("{}{}", inserter, new_pair_2);

            let new_count = *new_pairs.entry(new_pair_1.clone()).or_insert(0);
            new_pairs.insert(new_pair_1, count + new_count);

            let new_count = *new_pairs.entry(new_pair_2.clone()).or_insert(0);
            new_pairs.insert(new_pair_2, count + new_count);

            let freq = *freqs.entry(inserter.clone()).or_insert(0);
            freqs.insert(inserter.clone(), freq + count);
        }

        pairs = new_pairs;

        dbg!(&pairs);
        dbg!(&freqs);
    }



    let mut highest_freq = std::i64::MIN;
    let mut lowest_freq = std::i64::MAX;

    for (_, freq) in freqs.iter() {
        if freq < &lowest_freq {
            lowest_freq = *freq;
        }

        if freq > &highest_freq {
            highest_freq = *freq;
        }
    }

    dbg!(highest_freq-lowest_freq);



    Ok(())
}
