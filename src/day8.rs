// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use std::collections::HashMap;

const INPUT: &str = include_str!("day8/input.txt");

#[derive(Debug)]
struct LineInfo {
    signals:Vec<String>,
    output:Vec<String>
}

fn main() -> anyhow::Result<()> {
    let input = INPUT
            .split('\n')
            .filter_map(|s| parse_input_line(s))
            .collect::<Vec<LineInfo>>();


    // digit -> segmentcount, segments
    // 0 -> 6, a b c _ e f g
    // 1 -> 2, _ _ c _ _ f _  *
    // 2 -> 5, a _ c d e _ g
    // 3 -> 5, a _ c d _ f g
    // 4 -> 4, _ b c d _ f _  *
    // 5 -> 5, a b _ d _ f g
    // 6 -> 6, a b _ d e f g
    // 7 -> 3, a _ c _ _ f _  *
    // 8 -> 7, a b c d e f g  *
    // 9 -> 6, a b c d _ f g

    // freqs, e=4, f=9, b=6
    ////dbg!(&input);



//     0:      1:      2:      3:      4:
//     aaaa    ....    aaaa    aaaa    ....
//    b    c  .    c  .    c  .    c  b    c
//    b    c  .    c  .    c  .    c  b    c
//     ....    ....    dddd    dddd    dddd
//    e    f  .    f  e    .  .    f  .    f
//    e    f  .    f  e    .  .    f  .    f
//     gggg    ....    gggg    gggg    ....

//      5:      6:      7:      8:      9:
//     aaaa    aaaa    aaaa    aaaa    aaaa
//    b    .  b    .  .    c  b    c  b    c
//    b    .  b    .  .    c  b    c  b    c
//     dddd    dddd    ....    dddd    dddd
//    .    f  e    f  .    f  e    f  .    f
//    .    f  e    f  .    f  e    f  .    f
//     gggg    gggg    ....    gggg    gggg


    let mut count = 0;
    for line in input.iter() {
        let (one, four, seven, eight) = find_initial_digits(&line.signals);

        //dbg!(&one, &four, &seven, &eight);

        let a = diff(&one, &seven);

        //dbg!(&a);

        let (e, f, b) = find_signals_by_frequency_analysis(&line.signals);

        //dbg!(&e, &f, &b);

        let c =  diff(&one, &f);

        //dbg!(&one);
        //dbg!(&c);

        let mut bcf = String::from("");
        bcf.push_str(&b);
        bcf.push_str(&c);
        bcf.push_str(&f);

        let d = diff(&bcf, &four);

        //dbg!(&d);

        let mut abcdef = String::from("");
        abcdef.push_str(&a);
        abcdef.push_str(&b);
        abcdef.push_str(&c);
        abcdef.push_str(&d);
        abcdef.push_str(&e);
        abcdef.push_str(&f);

        let g = diff(&abcdef, &eight);
        //dbg!(&g);

        let zero = String::from_iter([a.clone(),b.clone(),c.clone(),e.clone(),f.clone(),g.clone()]);
        let two = String::from_iter([a.clone(),c.clone(),d.clone(),e.clone(),g.clone()]);
        let three = String::from_iter([a.clone(),c.clone(),d.clone(),f.clone(),g.clone()]);
        let five = String::from_iter([a.clone(),b.clone(),d.clone(),f.clone(),g.clone()]);
        let six = String::from_iter([a.clone(),b.clone(),d.clone(),e.clone(),f.clone(),g.clone()]);
        let nine = String::from_iter([a.clone(),b.clone(),c.clone(),d.clone(),f.clone(),g.clone()]);

        //dbg!(&zero);
        //dbg!(&one);
        //dbg!(&two);
        //dbg!(&three);
        //dbg!(&four);
        //dbg!(&five);
        //dbg!(&six);
        //dbg!(&seven);
        //dbg!(&eight);
        //dbg!(&nine);


        let mut result = String::from("");
        for digit in line.output.iter() {
            if eq(digit, &zero) {
                result.push_str("0");
            } else if eq(digit, &one) {
                result.push_str("1");
            }else if eq(digit, &two) {
                result.push_str("2");
            }else if eq(digit, &three) {
                result.push_str("3");
            }else if eq(digit, &four) {
                result.push_str("4");
            }else if eq(digit, &five) {
                result.push_str("5");
            }else if eq(digit, &six) {
                result.push_str("6");
            }else if eq(digit, &seven) {
                result.push_str("7");
            }else if eq(digit, &eight) {
                result.push_str("8");
            }else if eq(digit, &nine) {
                result.push_str("9");
            }
        }

        count += result.parse::<i64>().unwrap();
    }

    dbg!(count);

    ////dbg!(count);


    Ok(())
}

fn eq(first: &String, second: &String) -> bool {
    return diff(first, second).len() == 0
}

fn _find_cf_adg(signals:&Vec<String>) -> (String, String) {
    // 2 -> 5, a _ c d e _ g
    // 3 -> 5, a _ c d _ f g
    // 5 -> 5, a b _ d _ f g
    //         3 1 2 3 1 2 3
    // freq 2 -> cf
    // freq 3 -> adg

    let mut candidates:Vec<String> = Vec::new();
    let mut freqs = HashMap::new();
    let mut cf = String::from("");
    let mut adg = String::from("");
    for s in signals.iter() {
        match s.len() {
            5 => candidates.push(s.to_string()),
            _ => {},
        }
    }

    //dbg!(&candidates);

    for signal in candidates.iter() {
        for ch in signal.chars() {
            let f = *freqs.entry(ch).or_insert(0);
            freqs.insert(ch, f + 1);
        }
    }

    //dbg!( &freqs);
    for (segment, count) in freqs.iter() {
        match count {
            2 => cf.push(segment.clone()),
            3 => adg.push(segment.clone()),
            _ => {},
        }
    }

    return (cf, adg);
}


fn find_signals_by_frequency_analysis(signals:&Vec<String>) -> (String, String, String) {
    let mut e = String::from("");
    let mut f = String::from("");
    let mut b = String::from("");

    let mut freqs = HashMap::new();

    for signal in signals.iter() {
        for ch in signal.chars() {
            let f = *freqs.entry(ch).or_insert(0);
            freqs.insert(ch, f + 1);
        }
    }
    //dbg!( &freqs);
    for (segment, count) in freqs.iter() {
        match count {
            4 => e=segment.to_string(),
            9 => f= segment.to_string(),
            6 => b=segment.to_string(),
            _ => {},
        }
    }

    return (e,f,b);
}

fn _uni(first:&String, second:&String) -> String {
    let mut result = String::from("");

    for ch in first.chars() {
        if second.contains(ch) {
            result.push(ch);
        }
    }
    result
}

fn diff(first:&String, second:&String) -> String {
    let mut result = String::from("");

    for ch in first.chars() {
        if !second.contains(ch) {
            result.push(ch);
        }
    }

    for ch in second.chars() {
        if !first.contains(ch) {
            result.push(ch);
        }
    }

    result
}

fn find_initial_digits(signals:&Vec<String>) -> (String, String, String, String) {
    let mut one = String::from("");
    let mut four = String::from("");
    let mut seven = String::from("");
    let mut eight = String::from("");

    for s in signals.iter() {
        match s.len() {
            2 => one = s.to_string(),
            4 => four = s.to_string(),
            3 => seven = s.to_string(),
            7 => eight = s.to_string(),
            _ => {},
        }
    }

    return (one, four, seven, eight);
}

fn parse_input_line(s:&str) -> Option<LineInfo> {
    let parts = s.split_once('|');

    match parts {
        Some((si, ou)) =>
            return Some(LineInfo{
                signals: si.trim().split(' ').map(|v| v.to_string()).collect::<Vec<String>>(),
                output:  ou.trim().split(' ').map(|v| v.to_string()).collect::<Vec<String>>()
            }),
        _ => return None,
    }


}
