use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(problem: u8) {

    let input_path = "inputs/day_9.txt";
    let reader = BufReader::new(File::open(input_path).unwrap());

    const START_SIZE: usize = 512;
    const MAX_NEG: usize = START_SIZE/2 -1;

    let mut coor_vec: Vec<(i32, i32)> = Vec::new();

    let mut coords_tail: (i32, i32) = (MAX_NEG as i32, MAX_NEG as i32);
    let mut coords_head: (i32, i32) = (MAX_NEG as i32, MAX_NEG as i32);


    let mut grid = vec![vec![0; START_SIZE]; START_SIZE]; // max negative will be -255

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let instruction = line_contents.split(' ').collect::<Vec<&str>>();

        let num = instruction[1].parse::<u8>().unwrap();

        for i in 0..num {
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


            if coords_tail.0 > grid.len() as i32{
                grid.push(vec![0; grid[0].len()]);
            }
            if coords_tail.1 > grid[0].len() as i32{
                for i in 0..grid.len(){
                    grid[i].push(0);
                }
            }

            grid[coords_tail.0 as usize][coords_tail.1 as usize] = 1;
            //coor_vec.push(coords_tail);
        }
        //print!("{:?}, {:?}\n", coords_head, coords_tail);
    }

}