#[allow(unused_must_use)]

use std::fmt::{Display, Result, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp::min;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ListEntry{
    Num(u32),
    List(Vec<ListEntry>), // entry in list can either be another list or a number
}
// print pretty 
impl Display for ListEntry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ListEntry::Num(num) => write!(f, "{}", num),
            ListEntry::List(vec) => {
                write!(f, "[")?;
                if vec.len() == 1{
                    write!(f,"{}", vec[0])?;
                } else {
                    for i in 0..vec.len(){
                        if i != vec.len() -1 {
                            write!(f, "{},", vec[i])?; 
                        } else {
                            write!(f, "{}", vec[i])?; 
                        }
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

pub fn line_to_vector(line:&str) -> ListEntry {

    let mut vector:ListEntry = ListEntry::List(Vec::new());
    let mut open_count = 0;
    let mut skip_count = 0;
    for i in 1..line.len(){
        if skip_count != 0 {
            skip_count = skip_count -1;
            continue;
        }
        if open_count == 0 {
            if &line[i..i+1] == "["{
                match vector {
                    ListEntry::List(ref mut list) => list.push(line_to_vector(&line[i..line.len()-1])),
                    ListEntry::Num(_) => panic!("panic trying to push list to number"),
                };
                open_count = open_count + 1;
            } else if &line[i..i+1] == "]" {
                return vector
            } else if &line[i..i+1] != "," {
                let mut j = i;
                while &line[j..j+1] != "," && &line[j..j+1] != "]"{
                    j = j + 1;
                }
                skip_count = j-(i+1);
                match vector {
                    ListEntry::List(ref mut list) => list.push(ListEntry::Num(line[i..j].parse::<u32>().unwrap())),
                    ListEntry::Num(_) => panic!("panic trying to push list to number"),
                };
            }
        } else if &line[i..i+1] == "[" {
            open_count = open_count + 1; 
        } else if &line[i..i+1] == "]" {
            open_count = open_count - 1;
        }
    }
    vector
}

impl ListEntry {
    //recursive
    fn get_first(&self) -> Option<u32> {
        match self {
            ListEntry::Num(num) => Some(*num),
            ListEntry::List(vec) => {
                if vec.len() == 0 {
                    return None
                } else {
                    return vec[0].get_first()
                }
            },
        }
    }

    fn get_index(&self, index:usize) -> Option<&ListEntry> {
        match self {
            ListEntry::Num(_) => panic!("Tried to retrieve nth element from number"),
            ListEntry::List(vec) => {
                if vec.len() <= index {
                    return None
                } else {
                    return Some(&vec[index])
                }
            },
        }
    }
    fn len(&self) -> usize{
        match self {
            ListEntry::Num(_) => panic!("Tried to retrieve nth element from number"),
            ListEntry::List(vec) => return vec.len(),
        }
    }
}

fn compare_entries(list_l: &ListEntry, list_r: &ListEntry) -> u8 {
    //return codes 0: continue comparison, 1: correct order, 2: wrong order
    let (mut val_l, mut variant_l) = match list_l{
        ListEntry::Num(val) => (*val, 0),
        ListEntry::List(_) => (0, 1),
    };  

    let (mut val_r, mut variant_r) = match list_r{
        ListEntry::Num(val) => (*val, 0),
        ListEntry::List(_) => (0, 1),
    };  

    let mut toggle = 0; // 0 means both are just numbers
    if variant_l == 0 && variant_r == 1{
        match list_r.get_first() {
            Some(num) => val_r = num,
            None      => return 2,
        };
        if matches!(list_r, ListEntry::List(vec) if vec.len() == 1) {
            toggle = 0
        } else {
            toggle = 2;// right was a lsit
        }
        variant_r = 0;
    } else if variant_l == 1 && variant_r == 0 {
        match list_l.get_first() {
            Some(num) => val_l = num,
            None      => return 1,
        };
        if matches!(list_l, ListEntry::List(vec) if vec.len() == 1) {
            toggle = 0
        } else {
            toggle = 1; // left was a list 
        }
        variant_l = 0;
    }

    // both elements must now be lists or numbers
    if variant_l == 0 && variant_r == 0 {
        if val_l == val_r {
            if toggle == 1 {
                return 2
            } else if toggle == 2 {
                return 1
            } else {
                return 0
            }
        } else if val_l < val_r {
            return 1
        } else {
            return 2
        }
    }

    //check if either list is empty 
    if 0 >= min(list_l.len(), list_r.len()){
        if list_l.len() == list_r.len() {
            return 0
        } else if list_l.len() < list_r.len() {
            return 1
        } else {
            return 2
        }
    }

    //both elemetns are lists.
    let mut i = 0;
    loop {
        let item_l = list_l.get_index(i).unwrap();
        let item_r = list_r.get_index(i).unwrap();

        let val = compare_entries(item_l, item_r);

        if val != 0{
            return val
        }

        i = i+1;
        if i >= min(list_l.len(), list_r.len()){
            if list_l.len() == list_r.len() {
                return 0
            } else if list_l.len() < list_r.len() {
                return 1
            } else {
                return 2
            }
        }
    }
}

pub fn solve(problem: u8) {
    let input_path = "inputs/day_13.txt";
    let mut reader = BufReader::new(File::open(input_path).unwrap());

    let mut line1 = String::new();
    let mut line2 = String::new();

    let mut vec_lines: Vec<ListEntry> = Vec::new();
    let mut answer = 0;
    let mut i = 1;

    if problem == 1 {
    loop {
        let _status = reader.read_line(&mut line1);
        let _status = reader.read_line(&mut line2);

        let list1 = line_to_vector(&line1);
        let list2 = line_to_vector(&line2);

        let val = compare_entries(&list1, &list2);
        if val == 0 {
            panic!("got val = 0 for one two lists");
        } else if val == 1{
            answer = answer + i;
        }

        line1.clear();
        let _status = reader.read_line(&mut line1);
        if &line1[0..1] == "."{
            break;
        }
        line1.clear();
        line2.clear();
        i = i + 1;
    }
    print!("The answer to day 13 problem {} is: {}\n", problem, answer);
    } else {
        loop {
            let _status = reader.read_line(&mut line1);
            let _status = reader.read_line(&mut line2);

            vec_lines.push(line_to_vector(&line1));
            vec_lines.push(line_to_vector(&line2));

            line1.clear();
            let _status = reader.read_line(&mut line1);
            if &line1[0..1] == "."{
                break;
            }
            line1.clear();
            line2.clear();
            i = i + 1;
        }

        vec_lines.push(ListEntry::List(vec![ListEntry::List(vec![ListEntry::Num(2)])])); //divier bits
        vec_lines.push(ListEntry::List(vec![ListEntry::List(vec![ListEntry::Num(6)])])); //dibider bits
        
        let mut index_low = vec_lines.len() - 2;
        let mut index_high = vec_lines.len()- 1;
        let mut toggle;

        loop {
            toggle = true;
            for i in 1..vec_lines.len(){
                let val = compare_entries(&vec_lines[i-1], &vec_lines[i]);
                if val == 2 {
                    vec_lines.swap(i-1, i);
                    toggle = false;
                    if i == index_low {
                        index_low = index_low - 1;
                    } else if i == index_low + 1 {
                        index_low = index_low + 1;
                    }
                    if i == index_high {
                        index_high = index_high -1;
                    } else if i == index_high + 1 {
                        index_high = index_high + 1;
                    }
                }
            }
            if toggle {
                break;
            }
        }
        print!("The answer to day 13 problem {} is: {}\n", problem, (index_low+1)*(index_high+1));
    }
}