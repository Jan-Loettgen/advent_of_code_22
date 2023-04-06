#![allow(dead_code)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

// SOLUTION APPROACH IS RECURSION

fn get_dir_size(file_vector: &mut Vec<u32>, reader:&mut BufReader<File>) -> u32{
    
    let mut line = String::new();
    let mut sum_size: u32 = 0;
    let _status = (*reader).read_line(&mut line).unwrap();

    loop{

        if line.len() == 0 {
            return sum_size
        } else if &line[0..1] == "$" {
            if line.len() == 9  && &line[..9] == "$ cd ..\r\n"{
                return sum_size 
            } else if line.len() >= 7 && &line[..4] == "$ cd"{
                let dir_size = get_dir_size(file_vector, reader);
                file_vector.push(dir_size);

                sum_size = sum_size + dir_size;
            }
        } else {
            if line.len() >= 7 && &line[..4] != "dir "{
                let file_size = line.split(' ').collect::<Vec<_>>()[0].parse::<u32>().unwrap();
                sum_size = sum_size +file_size
            }
        }
        line.clear();
        let _status = (*reader).read_line(&mut line).unwrap();
    }
}

pub fn solve(problem: u8) {

    let input_path = "inputs/day_7.txt";
    let mut reader = BufReader::new(File::open(input_path).unwrap());
    let mut file_vector = Vec::new();


    get_dir_size(&mut file_vector, &mut reader);

    let mut answer = 0;
    if problem == 1  {
        for element in file_vector {
            if element <= 100000 {
                answer = answer + element;
            }
        }
    } else {
        let required = 30000000 - (70000000 - file_vector[file_vector.len()-1]);

        let mut max = file_vector[file_vector.len()-1];
        for element in file_vector {
            if element >= required  && element < max{
                answer = element;
                max = element
            }
        }
    }
    print!("The answer to day 6 problem {} is: {}\n", problem, answer);
}