use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(problem: u8) {
    let input_path = "inputs/day_1.txt";

    let lines = BufReader::new(File::open(input_path).unwrap()).lines();
    let mut cal_cur = 0;
    let mut storage_vector: Vec<i32> = Vec::new();

    for line in lines {
        if let Ok(line_content) = line {
            if line_content == ""{
                storage_vector.push(cal_cur);
                cal_cur = 0;
            }
            else{
                cal_cur += line_content.parse::<i32>().unwrap();
            }
        }
    }
    storage_vector.sort();
    let num_elfs = storage_vector.len();

    if problem == 1{
        print!("The answer to day 1 problem {} is: {}\n", problem, storage_vector[num_elfs-1]);
    } else {
        print!("The answer to day 1 problem {} is: {}\n", problem, storage_vector[num_elfs-1]+storage_vector[num_elfs-2]+storage_vector[num_elfs-3]);
    }
}
