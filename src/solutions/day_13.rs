use std::fmt::{Write, Display, Result, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub enum ListEntry{
    Num(u32),
    List(Vec<ListEntry>),//  : Vec<List_entry>,
}

impl Display for ListEntry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return match self {
            ListEntry::Num(num) => write!(f, "{}", num),
            ListEntry::List(vec) => {
                write!(f, "{:?}", vec)}
        }
    }
}

// impl Display for VecOfEntry {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         for i in 0..self.vec.len(){
//             write!(f, "[{}, ]", self[i])
//         }
//     }
// }

pub fn line_to_vector(line:&str) -> Vec<ListEntry> {

    let mut vector:Vec<ListEntry> = Vec::new();
    let mut open_count = 0;
    for i in 1..line.len(){
        if open_count == 0 {

            if &line[i..i+1] == "["{
                vector.push(ListEntry::List(line_to_vector(&line[i..line.len()-1])));
                open_count = open_count + 1;
            } else if &line[i..i+1] == "]" {
                return vector
            } else if &line[i..i+1] != "," {
                vector.push(ListEntry::Num(line[i..i+1].parse::<u32>().unwrap()))
            }
        } else if &line[i..i+1] == "[" {
            open_count = open_count + 1; 
        } else if &line[i..i+1] == "]" {
            open_count = open_count - 1;
        }
    }
    vector
}

pub fn solve(problem: u8) {
    let input_path = "sample_inputs/day_13_sample.txt";
    let mut reader = BufReader::new(File::open(input_path).unwrap());

    let mut line1 = String::new();
    let mut line2 = String::new();

    loop {
        reader.read_line(&mut line1);
        reader.read_line(&mut line2);

        let mut list1 = line_to_vector(&line1);
        let mut list2 = line_to_vector(&line2);

        print!("{:?}\n", list1);
        print!("{:?}\n", list2);

        line1.clear();
        reader.read_line(&mut line1);
        if &line1[0..1] == "."{
            break;
        }
        line1.clear();
        line2.clear();
    }
}