use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

fn main() {
    let input_path = Path::new("input.txt");
    
    let file = match File::open(&input_path){
        Err(why) => panic!{"Couldnt find file: {}", why},
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut cal_cur = 0;
    let mut storage_vector: Vec<i32> = Vec::new();

    for line in reader.lines() {
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
    print!{"maximum carried by a single elf: {}\n", storage_vector[num_elfs-1]}
    print!{"sum of 3 most elements: {}\n", storage_vector[num_elfs-1]+storage_vector[num_elfs-2]+storage_vector[num_elfs-3]}
    
}
