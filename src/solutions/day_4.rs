use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

fn is_fully_contained(l1:i32, u1:i32, l2:i32, u2:i32) -> i32{

    if l1 <= l2 && u1 >= u2 {
        1
    } else if l2 <= l1 && u2 >= u1 {
        1
    } else {
        0
    }
}

fn overlap(l1:i32, u1: i32, l2:i32, u2:i32) -> i32{

    if l2 > u1 || l1 > u2 {
        0
    } else {
        1
    }
}
 
fn byte_array_to_num(byte_array: &[u8]) -> i32 {
    
    let mut index_pow = 1;
    let mut num:i32 = 0;

    
    for i in 0..byte_array.len() {
        let num_btye: i32 = (byte_array[byte_array.len()-1-i] - 48) as i32;
        num += num_btye*index_pow;
        index_pow = index_pow*10
    }
    num
}


fn main() {
    let input_path = Path::new("input.txt");
    let file = match File::open(&input_path){
        Err(why) => panic!{"Couldnt find file: {}", why},
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        if let Ok(line_content) = line {
            let line = line_content.as_bytes();

            let mut num_1_index = 0;
            let mut num_2_index = 0;
            let mut elf_index = 0;

            for i in 0..line.len() {
                if line[i] == 45 {

                    if num_1_index == 0 {
                        num_1_index = i;
                    } else {
                        num_2_index = i;
                    }
                    
                } else if line[i] == 44 {
                    elf_index = i;
                }
            }
            let l1 = byte_array_to_num(&line[0..num_1_index]);
            let u1 = byte_array_to_num(&line[num_1_index+1..elf_index]);
            let l2 = byte_array_to_num(&line[elf_index+1..num_2_index]);
            let u2 = byte_array_to_num(&line[num_2_index+1..]);

            num_1_index = 0;

            sum += overlap(l1, u1, l2, u2);
        }
    }
    print!("{}\n",sum)
}