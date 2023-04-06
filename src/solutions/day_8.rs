use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(problem: u8) {

    let input_path = "inputs/day_8.txt";
    let input_path = "sample_inputs/day_8_sample.txt";
    let mut reader = BufReader::new(File::open(input_path).unwrap());

    let mut gridsize: (u8, u8) = (0, 0);
    if problem == 1 {
        gridsize = (5, 5)
    } else {
        gridsize = (99, 99)
    }
    let mut line = String::new();
    let _status = reader.read_line(&mut line).unwrap();

    let mut mask = [[4; 5]; 5];
    print!("{:?}", mask);

    for i in range

}