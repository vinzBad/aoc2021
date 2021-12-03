// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!
const INPUT: &str = include_str!("day2/input.txt");

#[derive(Debug)]
enum SubmarineCommandDirection {
    Forward,
    Down,
    Up
}

#[derive(Debug)]
struct SubmarineCommand {
    direction: SubmarineCommandDirection,
    value: i64
}

fn day2() -> anyhow::Result<()> {
    let commands =
        INPUT
            .split('\n')
            .filter_map(|s| parse_submarine_command(s).ok())
            .collect::<Vec<SubmarineCommand>>()

    ;

    let (pos, depth) = compute_final_position(&commands);

    let _ = !dbg!(pos * depth);

    let (pos, depth) = compute_final_position_fixed(&commands);

    let _ = !dbg!(pos * depth);

    // let s2 = count_number_of_increases_in_windows(
    //     INPUT
    //         .split('\n')
    //         .filter_map(|s| s.parse::<i64>().ok())
    //         .collect()
    // );

    // dbg!(s2);

    Ok(())
}


fn compute_final_position_fixed(commands:&Vec<SubmarineCommand>) -> (i64, i64) {
    let mut horizontal_pos:i64 = 0;
    let mut depth:i64 = 0;
    let mut aim:i64 = 0;

    for command in commands.iter() {
        match command {
            SubmarineCommand {
                direction: SubmarineCommandDirection::Down, value:v
            } => aim += v,
            SubmarineCommand {
                direction: SubmarineCommandDirection::Up, value:v
            } => aim -= v,
            SubmarineCommand {
                direction: SubmarineCommandDirection::Forward, value:v
            } => {
                horizontal_pos += v;
                depth += aim * v
            }
        }
    }

    return (horizontal_pos, depth)
}

fn compute_final_position(commands:&Vec<SubmarineCommand>) -> (i64, i64) {
    let mut horizontal_pos:i64 = 0;
    let mut depth:i64 = 0;

    for command in commands.iter() {
        match command {
            SubmarineCommand {
                direction: SubmarineCommandDirection::Down, value:v
            } => depth += v,
            SubmarineCommand {
                direction: SubmarineCommandDirection::Up, value:v
            } => depth -= v,
            SubmarineCommand {
                direction: SubmarineCommandDirection::Forward, value:v
            } => horizontal_pos += v,
        }
    }

    return (horizontal_pos, depth)
}

fn parse_submarine_command(s: &str) -> anyhow::Result<SubmarineCommand>{
    let parts = s.trim().split_once(' ');
    let direction: SubmarineCommandDirection;
    let value: Result<i64, std::num::ParseIntError>;

    match parts {
        Some(("forward", v)) => {
            direction=SubmarineCommandDirection::Forward;
            value=v.parse::<i64>()
        },
        Some(("up", v)) => {
            direction=SubmarineCommandDirection::Up;
            value=v.parse::<i64>()
        },
        Some(("down", v)) => {
            direction=SubmarineCommandDirection::Down;
            value=v.parse::<i64>()
        },
        _ => return Err(anyhow::anyhow!("Unable to parse: {}", s)),

    }

    match value{
        Ok(v) => return Ok(SubmarineCommand{direction:direction, value:v}),
        Err(_) => return Err(anyhow::anyhow!("Unable to parse: {}", s)),
    }
}
