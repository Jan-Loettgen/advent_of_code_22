use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(problem: u8) {

    let input_path = "inputs/day_9.txt";
    let reader = BufReader::new(File::open(input_path).unwrap());

    const START_SIZE: usize = 512;
    const MAX_NEG: usize = START_SIZE/2 -1;

    let mut grid = vec![vec![0; START_SIZE]; START_SIZE]; // max negative will be -255

    if problem == 1 {
        let mut coords_tail: (i32, i32) = (MAX_NEG as i32, MAX_NEG as i32);
        let mut coords_head: (i32, i32) = (MAX_NEG as i32, MAX_NEG as i32);

        for line in reader.lines() {
            let line_contents = line.unwrap();
            let instruction = line_contents.split(' ').collect::<Vec<&str>>();

            let num = instruction[1].parse::<u8>().unwrap();

            for _i in 0..num {
                if instruction[0] == "R" {
                    coords_head.0 = coords_head.0 + 1;
                } else if instruction[0] == "U"{
                    coords_head.1 = coords_head.1 + 1;
                } else if instruction[0] == "L" {
                    coords_head.0 = coords_head.0 - 1;
                } else if instruction[0] == "D" {
                    coords_head.1 = coords_head.1 - 1;
                }

                let error_x: f32 = (coords_head.0 - coords_tail.0) as f32;
                let error_y:f32 = (coords_head.1 - coords_tail.1) as f32;

                if error_x*error_x + error_y*error_y >= 4.0 {
                    if error_x > 0.0 {
                        coords_tail.0 = coords_tail.0 + 1;
                    } else if error_x < 0.0 {
                        coords_tail.0 = coords_tail.0 - 1;
                    }

                    if error_y > 0.0 {
                        coords_tail.1 = coords_tail.1 + 1;
                    } else if error_y < 0.0 {
                        coords_tail.1 = coords_tail.1 - 1;
                    }
                }

                if coords_tail.0 >= (grid.len() as i32){
                    grid.push(vec![0; grid[0].len()]);
                }
                if coords_tail.1 >= (grid[0].len() as i32){
                    for i in 0..grid.len(){
                        grid[i].push(0);
                    }
                }
                grid[coords_tail.0 as usize][coords_tail.1 as usize] = 1;
            }
        }
    } else {
        let mut coor_vec: [(i32, i32); 10] = [(MAX_NEG as i32, MAX_NEG as i32); 10];

        for line in reader.lines() {
            let line_contents = line.unwrap();
            let instruction = line_contents.split(' ').collect::<Vec<&str>>();

            let num = instruction[1].parse::<u8>().unwrap();

            for _i in 0..num {
                if instruction[0] == "R" {
                    coor_vec[0].0 = coor_vec[0].0 + 1;
                } else if instruction[0] == "U"{
                    coor_vec[0].1 = coor_vec[0].1 + 1;
                } else if instruction[0] == "L" {
                    coor_vec[0].0 = coor_vec[0].0 - 1;
                } else if instruction[0] == "D" {
                    coor_vec[0].1 = coor_vec[0].1 - 1;
                }
                
                for j in 1..10 {

                    let error_x: f32 = (coor_vec[j-1].0 - coor_vec[j].0) as f32;
                    let error_y:f32 = (coor_vec[j -1].1 - coor_vec[j].1) as f32;

                    if error_x*error_x + error_y*error_y >= 4.0 {
                        if error_x > 0.0 {
                            coor_vec[j].0 = coor_vec[j].0 + 1;
                        } else if error_x < 0.0 {
                            coor_vec[j].0 = coor_vec[j].0 - 1;
                        }
    
                        if error_y > 0.0 {
                            coor_vec[j].1 = coor_vec[j].1 + 1;
                        } else if error_y < 0.0 {
                            coor_vec[j].1 = coor_vec[j].1 - 1;
                        }
                    }
                }

                if coor_vec[9].0 >= (grid.len() as i32){
                    grid.push(vec![0; grid[0].len()]);
                }

                if coor_vec[9].1 >= (grid[0].len() as i32){
                    for i in 0..grid.len(){
                        grid[i].push(0);
                    }
                }
                grid[coor_vec[9].0 as usize][coor_vec[9].1 as usize] = 1;
            }
        }
    }
    let mut answer = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                answer = answer + 1;
            }
        }
    }
    print!("The answer to day 9 problem {} is: {}\n", problem, answer);
}