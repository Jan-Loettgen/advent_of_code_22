use std::fs::File;
use std::io::{prelude::*, BufReader};

fn ascii_to_index(ascii_code: usize) -> usize {

    let mut index = ascii_code-65;

    if index > 25 {
        index = index -6;
    }
    if index >= 26 {
        index = index - 26;
    } else {
        index = index + 26
    }
    index
}


pub fn solve(problem: u8) {
    let input_path = "inputs/day_3.txt";
    let lines = BufReader::new(File::open(input_path).unwrap()).lines();

    let mut sum_prioirty = 0;
    let mut group_index = 0;
    let mut hash_map: [u8; 52]  = [0; 52];

    if problem == 1 {
        for line in lines {
            if let Ok(line_content) = line {
            
                let mut hash_map: [u8; 52]  = [0; 52];
        
                let line = line_content.as_bytes();
        
                for line_index in 0..line.len()/2{
                    let hashmap_index = ascii_to_index(line_content.as_bytes()[line_index] as usize);
                    hash_map[hashmap_index] = 1;
                }
                for line_index in line.len()/2..line.len(){
                    let hashmap_index = ascii_to_index(line_content.as_bytes()[line_index] as usize);
                    if hash_map[hashmap_index] == 1 {
                        //print!("{}, {}\n", line_content.as_bytes()[line_index] as char, hashmap_index);
                        sum_prioirty += hashmap_index+1;
                        hash_map[hashmap_index] = 0;
                    }
                }
            }
        }
        print!("The answer to day 3 problem {} is: {}\n", problem, sum_prioirty);

    } else {
        for line in lines {
            if let Ok(line_content) = line {
                let line = line_content.as_bytes();
                
                let mut hashmap_added: [u8; 52] = [0; 52];
                for line_index in 0..line.len(){
                    let hashmap_index = ascii_to_index(line_content.as_bytes()[line_index] as usize);

                    if hashmap_added[hashmap_index] == 0 {
                        hashmap_added[hashmap_index] = 1;
                        hash_map[hashmap_index] += 1;
                    }
                }

                if group_index == 2 {
                    for hashmap_index in 0..52 {
                        if hash_map[hashmap_index] == 3 {
                            sum_prioirty += hashmap_index + 1;
                            break;
                        }
                    }
                    group_index = 0;
                    for i in 0..52{
                        hash_map[i] = 0;
                    }
                } else {
                    group_index += 1
                }
            }
        }
        print!("The answer to day 3 problem {} is: {}\n", problem, sum_prioirty);
    }
}