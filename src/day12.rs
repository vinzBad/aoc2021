// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("day12/input.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT.split('\n').filter(|s| s.len() > 0);

    let re = Regex::new(r"(?P<start>\w+)-(?P<end>\w+)").unwrap();
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input {
        let caps = re.captures(line).unwrap();
        let start = caps[1].to_owned();
        let end = caps[2].to_owned();
        let ends = connections.entry(start.clone()).or_insert(HashSet::new());
        ends.insert(end.to_string());

        let starts = connections.entry(end).or_insert(HashSet::new());
        starts.insert(start.to_string());
    }
    let mut paths: Vec<Vec<String>> = Vec::new();

    for start in connections["start"].iter() {
        let starting_path = vec![String::from("start"), start.to_string()];
        paths.push(starting_path);
    }

    dbg!(&connections);

    let mut finishing_paths: Vec<Vec<String>> = Vec::new();
    for _ in 0..20 {
        let mut next_paths: Vec<Vec<String>> = Vec::new();

        for path in paths.iter_mut() {
            let current_cave = &path[path.len() - 1];
            dbg!(current_cave);
            for next_cave in connections[current_cave].iter() {
                if next_cave == "start" {
                    continue;
                }
                if next_cave == "end" {
                    let mut finished_path = path.clone();
                    finished_path.push(next_cave.clone());
                    finishing_paths.push(finished_path);
                    continue;
                }
                if &next_cave.to_lowercase().to_string() == next_cave {
                    let mut visits: HashMap<String, i64> = HashMap::new();
                    if path.contains(next_cave) {
                        let mut has_second_visit = false;
                        for visited_cave in path.iter() {
                            if &visited_cave.to_lowercase().to_string() == visited_cave {
                                let count = *visits.entry(visited_cave.to_string()).or_insert(0);
                                visits.insert(visited_cave.to_string(), count + 1);
                                if count + 1 == 2 {
                                    has_second_visit = true;
                                    break;
                                }
                            }
                        }
                        if has_second_visit {
                            continue;
                        }
                    }
                }
                let mut next_path = path.clone();
                next_path.push(next_cave.clone());
                next_paths.push(next_path);
            }
        }

        if next_paths.len() == 0 {
            break;
        }
        paths = next_paths;
    }

    dbg!(&finishing_paths);
    dbg!(finishing_paths.len());

    Ok(())
}
