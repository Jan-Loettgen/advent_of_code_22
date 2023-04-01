use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

pub fn solve() {
    const NUM_STACKS: usize = 9;

    //let mut stack_heights: [u8; NUM_STACKS] = [0; NUM_STACKS];
    let mut stacks: [Vec<u8>; NUM_STACKS] = Default::default();

    let mode = 2; // 1 means problem 1, means problem 2.

    let input_path = Path::new("inputs/day_5.txt");
    let file = match File::open(&input_path){
        Err(why) => panic!{"Couldnt find file: {}", why},
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut problem_start = false;

    for line in lines{
        if let Ok(line_contents) = line {

            if line_contents.len() == 0 {
                problem_start = true;
                for i in 0..NUM_STACKS{
                    stacks[i].reverse();
                }
                continue;

            }else if line_contents.as_bytes()[1] == 49 {
                continue;
            }
            
            if problem_start == false {
                for j in 0..NUM_STACKS{
                    if line_contents.as_bytes()[1+j*4] != b' ' {
                        stacks[j].push(line_contents.as_bytes()[1+j*4])
                    } 
                }
            } else {
                let instructions = line_contents.split(' ').collect::<Vec<_>>();
                let num: usize = (instructions[1].parse::<i32>().unwrap() as usize).try_into().unwrap();
                let from:usize = (instructions[3].parse::<i32>().unwrap() as usize).try_into().unwrap();
                let to:usize = (instructions[5].parse::<i32>().unwrap() as usize).try_into().unwrap();

                if mode == 1 {
                    for i in 0..num{
                        let element =stacks[from -1].pop();
                        match element {
                            Some(element) =>  stacks[to -1].push(element),
                            None => panic!("please help"),
                        }
                    }
                } else {
                    let len_before = stacks[from -1].len();
                    for i in 0..num{
                        //let index = (len_before - num);
                        let index_usize:usize = (len_before - num) as usize;
                        let element = stacks[from -1].remove(index_usize);
                        stacks[to -1].push(element);
                    }
                }
            }
        }
    }
    let mut answer = String::new();
    for i in 0..NUM_STACKS {
        let element:char = stacks[i].pop().unwrap() as char;
        answer.push(element)
    }
    print!("The answer is : {}\n", answer);
}


// use vector as stack!!!