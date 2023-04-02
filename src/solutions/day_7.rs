#![allow(dead_code)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Dir<'a>{
    dirs: Vec<Dir<'a>>,
    files: Vec<u32>,
    dir_index: usize,
    dir_size: u32,
    parent: Option<&'a Dir<'a>>,
}

impl Dir<'_> {
    fn new () -> Dir<'static>{
        Dir{ dirs : Vec::new(), files : Vec::new(), dir_index: 0, dir_size: 0, parent: None}
    }
}

pub fn solve(_problem: u8) {

    let input_path = "sample_inputs/day_7_sample.txt";
    let lines = BufReader::new(File::open(input_path).unwrap()).lines();

    let mut dir_tree = Dir::new();
    let mut active_dir = &mut dir_tree;

    for line in lines {
        let line_contents = line.unwrap();
        if line_contents == "$ cd /" {
            continue;
        }

        if line_contents.as_bytes()[0] != b'$'{
            if line_contents.as_bytes()[0] == b'd' {
                active_dir.dirs.push(Dir::new());
            } else {
                let num = line_contents.split(' ').collect::<Vec<_>>()[0].parse::<u32>().unwrap();
                active_dir.dir_size += num;
                active_dir.files.push(num);
            } 
        } else {
            if &line_contents[0..4] == "$ cd" {
                active_dir = &mut active_dir.dirs[active_dir.dir_index];
            }
        }
    }
}