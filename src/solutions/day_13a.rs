use std::fs::File;
use std::io::{prelude::*, BufReader};

fn compare_items(item_1: &str, item_2: &str) -> u8 {
    // 0 = continue
    // 1 = right order
    // 2 = wrong order

    if item_1 == item_2 {
        return 0
    } else if item_1 == ""{
        return 1
    } else if item_2 == ""{
        return 2
    } else {
        let val_1 = item_1.parse::<u32>().unwrap();
        let val_2 = item_2.parse::<u32>().unwrap();
        if val_1 < val_2 {
            return 1
        } else if val_1 > val_2 {
            return 2
        } else {
            return 0
        }
    }
}

fn compare_lists(list_1: &str, list_2: &str) -> u8 {

    if list_1 == list_2 {
        return 0
    }

    let mut order = 0;
    for i in 0..100 { // need to fix this limit...
        let (item_1, list_1_bool) = extract_item(list_1, i);
        let (item_2, list_2_bool) = extract_item(list_2, i);

        if list_1_bool && list_2_bool{
            order = compare_lists(item_1, item_2);

        } else if list_1_bool && !list_2_bool {
            if item_2 == ""{
                return 2
            }
            let  (mut item_11, mut list_11_bool) =  extract_item(item_1, 0);
            while list_11_bool {
                (item_11, list_11_bool) = extract_item(item_11, 0);
            }
            order = compare_items(item_11, item_2);

        } else if !list_1_bool && list_2_bool {
            if item_1 == ""{
                return 1
            }
            let (mut item_22, mut list_22_bool) =  extract_item(item_2, 0);
            while list_22_bool {
                (item_22, list_22_bool) = extract_item(item_22, 0);
            }
            order = compare_items(item_1, item_22);

        } else {
            order = compare_items(item_1, item_2)
        }

        if order == 1 {
            return 1
        } else if order == 2 {
            return 2
        }
    }
    0
}

fn extract_list(list: &str) -> &str{
    let mut level = 0;
    for i in 0..list.len() {
        if &list[i..i+1] == "[" {
            level = level + 1;
        } else if &list[i..i+1] == "]" {
            level = level - 1;
        }

        if level == 0 {
            return &list[0..i+1]
        }
    }
    return &"";
}

fn extract_item(list : &str, item_index: usize) -> (&str, bool) {
    if list == "[]" { // slot is an empty list.
        return (&"", false)
    }

    let mut open_count = 0;
    let mut index = 0;

    for i in 1..list.len() {
        if index == item_index {
            if &list[i..i+1] == "[" { // slot contains a list
                let item = extract_list(&list[i..list.len()]);
                return (item, true)

            // } else if &list[i+1..i+2] == "]" { // slot is an empty list.
            //     return (&"", true) // this logic is not correct and dangerous

            }   else { //numberical item
                for j in i..list.len() {
                    if &list[j..j+1] == "," || &list[j..j+1] == "]"{
                        return (&list[i..j], false)
                    }
                }
            }
        } else if &list[i..i+1] == "["{
            open_count = open_count + 1;
        } else if &list[i..i+1] == "]" {
            open_count = open_count -1 ;
        } else if open_count == 0 && &list[i..i+1] == ","{
            index = index + 1;
        }
    }   //items being returned with comma...
    (&"", false)
}

//if two inputs are equal then dont need to check rest of list!.

pub fn solve(problem: u8) {
    let input_path = "sample_inputs/day_13_sample.txt";
    let mut reader = BufReader::new(File::open(input_path).unwrap());

    let mut line1 = String::new();
    let mut line2 = String::new();

    let mut i = 1;
    let mut answer = 0;
    loop {
        reader.read_line(&mut line1);
        reader.read_line(&mut line2);
        let order = compare_lists(&line1, &line2);

        //print!("{:?}\n", correct_order);
        if order == 1{
            answer = answer + i;
        }
        i = i +1;

        line1.clear();
        reader.read_line(&mut line1);
        if &line1[0..1] == "."{
            break;
        }

        line1.clear();
        line2.clear();
    }
    print!("{}\n", answer);

}