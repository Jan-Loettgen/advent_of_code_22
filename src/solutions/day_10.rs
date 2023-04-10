use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(problem: u8) {

    let input_path = "inputs/day_10.txt";
    let mut reader = BufReader::new(File::open(input_path).unwrap());

    let mut cycle: i32 = 1;

    let mut x: i32 = 1;
    let mut cur_instruction: i32 = 0;

    let mut addx_progress = 0;
    let mut line = String::new();
    let mut answer = 0;

    let mut grid: [[char; 40]; 6] = [[' '; 40]; 6];
    let mut row_index:i32 = 0;
    let mut col_index:i32 = 0;

    loop {
        if problem == 1 {
            if cycle == 20 || (cycle + 20) % 40 == 0 {
                answer = answer + cycle*x;
            }
        } else {
            row_index = (cycle-1 as i32) / 40;
            col_index = (cycle-1) % 40;
            if col_index >= x - 1 && col_index <= x + 1{
                grid[row_index as usize][col_index as usize] = '#'
            }
        }

        if addx_progress == 0 {
            let _status = reader.read_line(&mut line).unwrap();

            if line.len() <3 {
                break; // used to exit out of loop
            }
            if &line[0..4] != "noop" {
                line.pop(); // remove \n 
                line.pop(); // remove \r
                cur_instruction = line.split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap();
                addx_progress = 1;
            }
            line.clear();
        } else {
            x = x + cur_instruction;
            addx_progress = 0;
        }
        cycle = cycle +1;
    }
    if problem == 1 {
        print!("The answer to day 10 problem {} is: {}\n", problem, answer);
    } else {
        // only nice formatting for output
        print!("The answer to day 10 problem {} is:\n", problem);
        print!("+");
        for i in 0..42{
            print!("-");
        }
        print!("+\n");
        for i in 0..6 {
            print!("| ");
            for j in 0..40 {
                print!("{}", grid[i][j]);
            }
            print!(" |\n");
        }
        print!("+");
        for i in 0..42{
            print!("-");
        }
        print!("+\n");
    }
}