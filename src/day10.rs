// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use std::collections::HashMap;

const INPUT: &str = include_str!("day10/input.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT
        .split('\n')
        .filter(|s| s.len() > 0)
        .collect::<Vec<&str>>();

    //     {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
    //     [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
    //     [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
    //     [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
    //     <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.

    // Stop at the first incorrect closing character on each corrupted line.

    // ): 3 points.
    // ]: 57 points.
    // }: 1197 points.
    // >: 25137 points.



// ): 1 point.
// ]: 2 points.
// }: 3 points.
// >: 4 points.

    // dbg!(&input);

    let open = HashMap::from([('(', ')'), ('<', '>'), ('{', '}'), ('[', ']')]);
    let close = open
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<&char, &char>>();
    let values = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137), ('(', 1), ('<', 4), ('{', 3), ('[', 2)]);

    let mut score = 0;
    let mut scores_auto = Vec::new();

    for line in input.iter() {
        let mut chunk_accu = Vec::new();
        //dbg!(&line);
        let mut has_illegal = false;
        let mut illegal_char = ' ';
        for ch in line.chars() {
            if open.contains_key(&ch) {
                chunk_accu.push(ch)
            } else if close.contains_key(&ch) {
                if chunk_accu[chunk_accu.len() - 1] == *close[&ch] {
                    chunk_accu.pop();
                } else {
                    // thats illegal
                    has_illegal = true;
                    illegal_char = ch;
                    score += values[&ch];
                    break;
                }
            }
        }

        if !has_illegal {
            let len = chunk_accu.len();
            let mut score_auto:i64 = 0;
            for _ in 0..len {
                score_auto *= 5;
                score_auto += values[&chunk_accu.pop().unwrap()];
            }
            scores_auto.push(score_auto);
        }

        // if has_illegal {
        //     dbg!(illegal_char);
        // }
    }
    dbg!(score);
    scores_auto.sort();
    dbg!(scores_auto[scores_auto.len() / 2]);
    Ok(())
}

fn get_accu_delta(ch: char) -> (char, i64) {
    match ch {
        '(' | '<' | '{' | '[' => return (ch, 1),
        ')' => return ('(', -1),
        '>' => return ('<', -1),
        '}' => return ('{', -1),
        ']' => return ('[', -1),
        _ => panic!("unexpected input"),
    }
}
