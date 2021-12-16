// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use indicatif::ProgressBar;
use ordered_map::OrderedMap;

type Point = (i64, i64);
const INPUT: &str = include_str!("day15/input.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT
        .split('\n')
        .filter(|s| s.len() > 0)
        .collect::<Vec<&str>>();


    let src_height = input.len();
    let src_width = input[0].len();

    let height = src_height * 5;
    let width = src_width * 5;

    let mut map: Vec<i64> = Vec::new();
    map.reserve(height * width);

    for y in 0..src_height {
        let chars = input[y].chars().collect::<Vec<char>>();
        for x in 0..src_width {
            let risk = chars[x].to_digit(10).unwrap() as i64;

            map.push(risk);
        }
    }

    let neighbor_offset = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let neighbors = |p: Point| -> Vec<Point> {
        let (x, y) = p;
        let mut result = Vec::new();
        for (offset_x, offset_y) in neighbor_offset {
            let neighbor_x = x + offset_x;
            let neighbor_y = y + offset_y;

            if neighbor_x < 0 || neighbor_x >= (width as i64) {
                continue;
            }

            if neighbor_y < 0 || neighbor_y >= (height as i64) {
                continue;
            }

            result.push((neighbor_x, neighbor_y));
        }

        return result;
    };

    let weight = |p: Point| -> i64 {
        let (x, y) = p;

        let tile_x = x / src_width as i64;
        let tile_y = y / src_height as i64;
        let src_x = x - (tile_x * src_width as i64);
        let src_y = y - (tile_y * src_height as i64);
        let src_weight = map[(src_y * (src_height as i64) + src_x) as usize];
        let mut result_weight = src_weight + (tile_x + tile_y);

        //result_weight -= (result_weight / 9) * 9;
        loop {
            if result_weight > 9 {
                result_weight -= 9;
            } else {
                break;
            }
        }
        return result_weight;
    };

    let mut distances = HashMap::new();
    let mut distances_sorted = OrderedMap::new(|c| *c);
    let mut previous_points: HashMap<Point, Point> = HashMap::new();
    let mut visited_points: HashSet<Point> = HashSet::new();

    let start:Point = (0, 0);
    let end = ((width - 1) as i64, (height - 1) as i64);

    distances.insert(start, 0);
    distances_sorted.insert(start, 0);
    let pb = ProgressBar::new((width * height) as u64);


    loop {
        pb.inc(1);

        /*
        let active = point_with_lowest_dist_not_in(&distances, &visited_points);
        distances.remove(&active);
        */
        let active:Point = *distances_sorted.descending_keys().last().unwrap();
        distances_sorted.remove(&active);

        visited_points.insert(active.clone());

        if active == end {
            break;
        }

        for neighbor in neighbors(active) {
            if visited_points.contains(&neighbor) {
                continue;
            }
            let neighbor_dist =
                *distances.entry(active.clone()).or_insert(std::i64::MAX) + weight(neighbor);

            if neighbor_dist < *distances.entry(neighbor.clone()).or_insert(std::i64::MAX) {
                distances.insert(neighbor, neighbor_dist);
                distances_sorted.insert(neighbor,neighbor_dist);
                previous_points.insert(neighbor, active);
            }
        }
    }
    pb.finish();
    let mut path: Vec<Point> = Vec::new();
    path.push(end);
    let mut active = end;
    let mut total_risk = 0;
    loop {
        if previous_points.contains_key(&active) {
            total_risk += weight(active);
            active = previous_points[&active];
            path.push(active);
        } else {
            break;
        }
    }

    let mut result_viz = String::from("");
    for y in 0..height {
        for x in 0..width {
            let p = (x as i64, y as i64);
            if path.contains(&p) {
                result_viz += &weight(p).to_string();
            } else {
                result_viz += ".";
            }
        }
        result_viz += "\n";
    }

    println!("{}", result_viz);
    dbg!(total_risk);

    Ok(())
}

fn point_with_lowest_dist_not_in(
    distances: &HashMap<Point, i64>,
    visited_points: &HashSet<Point>,
) -> Point {
    let mut lowest_dist = std::i64::MAX;
    let mut lowest_point = (0, 0);

    for (point, dist) in distances.iter() {
        if visited_points.contains(point) {
            continue;
        }

        if dist < &lowest_dist {
            lowest_dist = *dist;
            lowest_point = *point;
        }
    }

    if lowest_dist == std::i64::MAX {
        panic!("failed to find lowest dist point");
    }

    return lowest_point;
}
