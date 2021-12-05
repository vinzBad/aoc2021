// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use regex::Regex;


const INPUT: &str = include_str!("day5/input.txt");


fn main() -> anyhow::Result<()> {
    let vent_lines =
        INPUT
            .split('\n')
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>();

    let mut _vent_map:[i64; 1000*1000] = [0;1000*1000];
    let mut _overlaps = 0;

    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();

    for vent_line in vent_lines.iter() {
        let coords = re.captures(vent_line).unwrap();
        let mut x1 = coords["x1"].parse::<i64>().unwrap();
        let x2 = coords["x2"].parse::<i64>().unwrap();
        let mut y1 = coords["y1"].parse::<i64>().unwrap();
        let y2 = coords["y2"].parse::<i64>().unwrap();

        // general bresenham
        let delta_x = (x2 - x1).abs();
        let delta_y = -(y2- y1).abs();

        //dbg!((x1, x2, y1, y2));
        dbg!((delta_x, delta_y));

        //if (delta_x != 0) && (delta_y != 0) {
        //   continue;
        //}

        let s_x;
        if x1 < x2 {
            s_x = 1;
        } else {
            s_x = -1;
        }

        let s_y;
        if y1 < y2 {
            s_y = 1;
        } else {
            s_y = -1;
        }

        let mut b_err = delta_x + delta_y;

        loop {
            let map_idx = (x1 + y1 * 1000) as usize;
            _vent_map[map_idx] += 1;

            if _vent_map[map_idx] == 2 {
                _overlaps += 1;
            }

            if (x1==x2) && (y1==y2) {
                break;
            }

            let b_err_2 = 2* b_err;

            if b_err_2 >= delta_y {
                b_err += delta_y;
                x1 += s_x;
            }

            if b_err_2 <= delta_x {
                b_err += delta_x;
                y1 += s_y;
            }
        }

    }

    dbg!(_overlaps);



    Ok(())
}
