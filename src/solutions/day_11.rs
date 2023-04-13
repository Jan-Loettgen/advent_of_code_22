use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;

#[derive(Debug)]
struct Item {
    instructions    : Vec<(u8, u128)>
}

impl Item{
    fn find_remainder(&self, divisor:u128) -> u128 {
        let mut remainder:u128 = start%divisor;

        for i in self.instructions.len(){
            let instruction = self.instructions[0];
            let item = self.instructions[i][1];

            if instruction == 1 { // addition
                remainder = (remainder + (item % divisor)) % divisor;
            } else if instruction == 2 { //multiplication
                remainder = (remainder * (item % divisor)) % divisor;
            } else instruction == 3 { // square
                remainder = (remainder * remainder) % divisor;
            }
        }   
        remainder
    }   
}

#[derive(Debug)]
struct Monke{
    items           : VecDeque<u128>,
    operation       : u8, // 1: addition, 2: multiplication, 3: square
    operation_comp  : u128,
    divisor         : u128,
    false_monke     : usize,
    true_monke      : usize,
    items_inspected : u32
}

impl Monke {
    fn new() -> Monke {
        Monke{
            items           : VecDeque::new(), 
            operation       : 4, 
            operation_comp  : 0, 
            divisor         : 0, 
            false_monke     : 0, 
            true_monke      : 0,
            items_inspected : 0}
    }

    fn give_item(&mut self, item:u128) {
        self.items.push_back(item);
        self.items_inspected = self.items_inspected +1;
    }
}

fn parse_input(input_path: &str, monkes: &mut Vec<Monke>) {
    let mut reader = BufReader::new(File::open(input_path).unwrap());
    let mut line = String::new();

    loop {
        let status = reader.read_line(&mut line).unwrap();

        if status == 0 {
            line.clear();
            break;
        } else if status == 1 {
            line.clear();
            continue;
        }
        
        if &line[0..6] == "Monkey"{
            line.clear();
            let status = reader.read_line(&mut line).unwrap();
            //line.pop(); // remove \r
            line.pop();  // remove \n
            let item_iter = line[18..status-1].split(", ");
            let mut starting_items: VecDeque<u128> = VecDeque::new();

            for item in item_iter{
                match item.parse::<u128>() {
                    Ok(num) => starting_items.push_back(num),
                    _ => (),
                };
                //starting_items.push_back(item.parse::<u128>().unwrap());
            }
            line.clear();
            let status = reader.read_line(&mut line).unwrap();

            let mut operation = 0;

            if &line[23..24] == "+"{
                operation = 1;
            } else {
                operation = 2;
            }
            let mut operation_comp_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
            //let operation_comp = operation_comp_str[0..operation_comp_str.len()-2].parse::<u32>();

            let operation_comp = match operation_comp_str[0..operation_comp_str.len()-1].parse::<u128>() {
                Ok(op_comp) => op_comp,
                Err(error) => {
                    operation = 3;
                    0},
            };
            line.clear();
            let status = reader.read_line(&mut line).unwrap();
            let mut divisor_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
            let divisor = divisor_str[0..divisor_str.len()-1].parse::<u128>().unwrap();

            line.clear();
            let status = reader.read_line(&mut line).unwrap();
            let mut true_monke_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
            let true_monke = true_monke_str[0..true_monke_str.len()-1].parse::<usize>().unwrap();

            line.clear();
            let status = reader.read_line(&mut line).unwrap();
            let mut false_monke_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
            let false_monke = false_monke_str[0..false_monke_str.len()-1].parse::<usize>().unwrap();
            
            line.clear();

            monkes.push(Monke{
                items           : starting_items, 
                operation       : operation,
                operation_comp  : operation_comp,
                divisor         : divisor,
                true_monke      : true_monke,
                false_monke     : false_monke,
                items_inspected : 0});
        }
    }
}

pub fn solve(problem: u8) {
    let input_path = "sample_inputs/day_11_sample2.txt";
    let mut monkes: Vec<Monke> = Vec::new();

    let mut test_queue: VecDeque<usize> = VecDeque::new();

    parse_input(input_path, &mut monkes);

    for i in 0..30 {
        for j in 0..monkes.len() {
            for k in 0..monkes[j].items.len() {

                let mut target_monke: usize = 0;
                let mut worry: u128 = 0;
                {
                    let item = monkes[j].items.pop_front().unwrap();
                    monkes[j].items_inspected = monkes[j].items_inspected + 1; 

                    if monkes[j].operation == 1  {
                        worry = item + monkes[j].operation_comp;
                    } else if monkes[j].operation == 2 {
                        worry = item*monkes[j].operation_comp;
                    } else {
                        worry = item * item;
                    }

                    if problem == 1 {
                        worry = worry /3;
                    }

                    if worry % monkes[j].divisor == 0 {
                        target_monke = monkes[j].true_monke;
                    } else {
                        target_monke = monkes[j].false_monke;
                    }
                }
                monkes[target_monke].items.push_back(worry);
            }
        }
        for (i, monke) in (&monkes).iter().enumerate() {
            print!("{}, {:?}\n", i, monke);
        }
        print!("-------------------------------\n");
    }


    for monke in &monkes {
        print!("{:?}\n", monke);
    }
    //print!("{:?}", monkes[1].items_inspected * monkes[7].items_inspected);


}