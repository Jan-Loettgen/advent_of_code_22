use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
fn print_map(map:&[[u8; 99]; 99]) {
    for i in 0..map.len() {
        print!("{:?}\n", map[i]);
    }
}

pub fn solve(problem: u8) {

    let input_path = "inputs/day_8.txt";
    //let input_path = "sample_inputs/day_8_sample.txt";
    let reader = BufReader::new(File::open(input_path).unwrap());

    let gridsize: usize = 99;

    let mut map: [[u8; 99]; 99] = [[0; 99]; 99];
    for (j, line) in reader.lines().enumerate(){
        let line_contents = line.unwrap();
        for i in 0..gridsize {
            map[j][i] = (&line_contents[i..i+1]).parse::<u8>().unwrap();
        }
    }

    let mut mask = [[4; 99]; 99];
    for i in 0..gridsize {
        mask[0][i] = 4;
        mask[gridsize-1][i] = 4;
        mask[i][0] = 4;
        mask[i][gridsize-1] = 4;
    }

    for i in 1..(gridsize-1) {
        let mut max_l = map[i][0];
        let mut max_r = map[i][gridsize-1];
        let mut max_up = map[0][i];
        let mut max_down = map[gridsize-1][i];
        for j in 1..(gridsize-1) {
            let left = map[i][j];
            let right = map[i][gridsize-1-j];
            let up = map[j][i];
            let down = map[gridsize-1-j][i];

            if left <= max_l { // seraching left to right
                mask[i][j] = mask[i][j] -1;
            } else {
                max_l = left;
            }

            if right <= max_r { // seraching right to left
                mask[i][gridsize-1-j] = mask[i][gridsize-1-j] -1;
            } else {
                max_r = right;
            }

            if up <= max_up { // seraching up to down
                mask[j][i] = mask[j][i] -1;
            } else {
                max_up = up;
            }

            if down <= max_down { // seraching down to up
                mask[gridsize-1-j][i] = mask[gridsize-1-j][i] -1;
            } else {
                max_down = down;
            }
        }
    }

    let mut count = 0;
    for i in 0..gridsize {
        for j in 0..gridsize {
            if mask[i][j] != 0 {
                count = count + 1;
            }
        }
    }

    print!("{}\n", count);
}