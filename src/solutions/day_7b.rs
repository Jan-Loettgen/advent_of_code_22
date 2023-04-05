#![allow(dead_code)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_dir_size(file_vector: &mut Vec<u32>, reader:&mut BufReader<File>){
    
    let mut line = String::new();

    (*reader).read_line(&mut line);
    print!("{:?}\n", &line[0..4]);

    while true{
        print!("test{}\n", 5);
        line.clear();
        (*reader).read_line(&mut line);
    }

}


pub fn solve(_problem: u8) {

    let input_path = "sample_inputs/day_7_sample.txt";
    let mut reader = BufReader::new(File::open(input_path).unwrap());
    let mut file_vector = Vec::new();


    print!("{:?}\n",get_dir_size(&mut file_vector, &mut reader));


//     for line in lines {
//         let line_contents = line.unwrap();
//         if line_contents == "$ cd /" {
//             continue;
//         }

//         if line_contents.as_bytes()[0] != b'$'{
//             if line_contents.as_bytes()[0] == b'd' {
//                 active_dir.dirs.push(Dir::new());
//             } else {
//                 let num = line_contents.split(' ').collect::<Vec<_>>()[0].parse::<u32>().unwrap();
//                 active_dir.dir_size += num;
//                 active_dir.files.push(num);
//             } 
//         } else {
//             if &line_contents[0..4] == "$ cd" {
//                 active_dir = &mut active_dir.dirs[active_dir.dir_index];
//             }
//         }
//     }
}