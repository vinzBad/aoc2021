// very much inspired by https://fasterthanli.me/series/advent-of-code-2020/part-1
// thanks amos!

use std::collections::HashMap;
use std::collections::HashSet;

use indicatif::ProgressBar;
use bit_vec::BitVec;



type Point = (i64, i64);



fn main() -> anyhow::Result<()> {


    // (20,  -5) <-> (30,  -5)
    //
    // (20, -10) <-> (30, -10)

    let target_area_top_left:Point = (155, -67);
    let target_area_bottom_right:Point = (182, -117);

    let mut highest_y_trajectory = std::i64::MIN;
    let mut stylish_vel = (0,0);
    let mut valid_vels = 0;
    for bx_traj in 0..1000 {
        let x_traj = bx_traj - 500;
        for by_traj in 0..4000 {
            let y_traj = by_traj - 2000;
            let (success, hyt) = calc_trajectory((x_traj, y_traj), target_area_top_left, target_area_bottom_right);
            if success {
                valid_vels += 1;
                //print_trajectory((x_traj, y_traj), target_area_top_left, target_area_bottom_right);
                //println!("{} - ({},{})", hyt, x_traj, y_traj);
                //println!("==============\n\n");
            }
            if success && hyt > highest_y_trajectory {
                highest_y_trajectory = hyt;

                stylish_vel = (x_traj, y_traj);
            }
        }
    }

    dbg!(highest_y_trajectory);
    dbg!(stylish_vel);
    dbg!(valid_vels);
    Ok(())
}



fn print_trajectory(initial_velocity:Point, target_area_top_left:Point, target_area_bottom_right:Point) {
    let mut position:Point = (0, 0);
    let mut velocity = initial_velocity;

    let mut path:Vec<Point> = Vec::new();



    let (lowest_x, highest_y) = target_area_top_left;
    let (highest_x, lowest_y) = target_area_bottom_right;

    for _ in 0..200 {
        path.push(position);

        let (x,y) = position;


        if y < highest_y {
            break;
        }

        if y >= lowest_y && y <= highest_y && x >= lowest_x && x <= highest_x {
            break
        }

        let (x_vel, y_vel) = velocity;

        position = (x+x_vel, y+y_vel);

        if x_vel > 0 {
            velocity = (x_vel-1, y_vel -1);
        } else if x_vel < 0 {
            velocity = (x_vel+1, y_vel -1);
        } else {
            velocity = (x_vel, y_vel -1);
        }

    }

    let mut result_viz = String::from("");

    for by in 0..25 {
        let y = 10 - by;

        for x in 0..40 {


            if path.contains(&(x,y)) {
                result_viz += "#";
            } else if y >= lowest_y && y <= highest_y && x >= lowest_x && x <= highest_x {
                result_viz += "T";
            } else {
                result_viz += ".";
            }
        }
        result_viz += &y.to_string();
        result_viz += "\n"
    }

    println!("{}", result_viz);
}


fn calc_trajectory(initial_velocity:Point, target_area_top_left:Point, target_area_bottom_right:Point) -> (bool, i64) {
    let mut position = (0, 0);
    let mut velocity = initial_velocity;
    let mut highest_y_trajectory = std::i64::MIN;

    let (lowest_x, highest_y) = target_area_top_left;
    let (highest_x, lowest_y) = target_area_bottom_right;

    for _ in 0..1000 {
        let (x,y) = position;

        if y > highest_y_trajectory {
            highest_y_trajectory = y;
        }

        if y >= lowest_y && y <= highest_y && x >= lowest_x && x <= highest_x {
            return (true, highest_y_trajectory)
        }



        let (x_vel, y_vel) = velocity;

        position = (x+x_vel, y+y_vel);

        if x_vel > 0 {
            velocity = (x_vel-1, y_vel -1);
        } else if x_vel < 0 {
            velocity = (x_vel+1, y_vel -1);
        } else {
            velocity = (x_vel, y_vel -1);
        }



    }
    return (false, highest_y_trajectory)

}
