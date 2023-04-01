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

    let mut part_1_score = 0;
    let mut part_2_score = 0;

    for line in reader.lines() {
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
    print!{"achieved score using first method: {}\n", part_1_score}
    print!{"achieved score using second method: {}\n", part_2_score}
}