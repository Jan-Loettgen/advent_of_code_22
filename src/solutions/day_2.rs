use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve(problem: u8) {
    let input_path = "inputs/day_2.txt";
    
    let lines = BufReader::new(File::open(input_path).unwrap()).lines();

    let mut part_1_score = 0;
    let mut part_2_score = 0;

    for line in lines {
        if let Ok(line_content) = line {
     
            let them = line_content.as_bytes()[0] as char;
            let me = line_content.as_bytes()[2] as char;

            if me == 'X'{
                if them == 'A'{
                    part_1_score += 4;
                    part_2_score += 3;
                }else if them == 'B'{
                    part_1_score += 1;
                    part_2_score += 1;
                }else {
                    part_1_score += 7;
                    part_2_score += 2;
                }
            }else if me == 'Y'{
                if them == 'A'{
                    part_1_score += 8;
                    part_2_score += 4;
                }else if them == 'B'{
                    part_1_score += 5;
                    part_2_score += 5;
                }else {
                    part_1_score += 2;
                    part_2_score += 6;
                }
            }else{
                if them == 'A'{
                    part_1_score += 3;
                    part_2_score += 8;
                }else if them == 'B'{
                    part_1_score += 9;
                    part_2_score += 9;
                }else {
                    part_1_score += 6;
                    part_2_score += 7;
                }
            }
        }
    }
    if problem == 1{
        print!("The answer to day 2 problem {} is: {}\n", problem, part_1_score);
    } else {
        print!("The answer to day 2 problem {} is: {}\n", problem, part_2_score);
    }
}