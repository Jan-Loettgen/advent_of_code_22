use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;

#[derive(Debug)]
struct Item {
    base            : u128,
    remainders      : Vec<u128>, // keep track of cuurrent remainder with respect to all divisors
}

impl Item{
    fn get_remainder(&mut self, divisors: &Vec<u128>, divisor_cur:u128, operation: u8, operation_comp: u128) -> u128 {
        let mut remainder_return: u128 = 0;

        for i in 0..divisors.len(){
            let divisor = divisors[i];
            //let mut remainder:u128 = self.remainders[i]

            if operation == 1 {         // addition
                self.remainders[i] = (self.remainders[i] + (operation_comp )) % divisor;
            } else if operation == 2 {  //multiplication
                self.remainders[i] = (self.remainders[i] * (operation_comp )) % divisor;
            } else {     // square
                self.remainders[i] = (self.remainders[i] * self.remainders[i]) % divisor;
            }
            //remainders[i] = remainder;

            if divisor == divisor_cur {
                remainder_return = self.remainders[i];
            }
        }   
        remainder_return
    }
    
    fn new(base:u128) -> Item{

        Item{
            base            : base,
            remainders      : vec![0; 8],
        }
    }
    
    fn init_remainders(&mut self, divisors: &Vec<u128>) {
        for i in 0..divisors.len(){
            self.remainders[i] = self.base % divisors[i];
        }
    }
}

#[derive(Debug)]
struct Monke{
    items           : VecDeque<Item>,
    operation       : u8, // 1: addition, 2: multiplication, 3: square
    operation_comp  : u128,
    divisor         : u128,
    false_monke     : usize,
    true_monke      : usize,
    items_inspected : u128,
}

fn parse_input(input_path: &str, monkes: &mut Vec<Monke>) -> Vec<u128>{
    let mut reader = BufReader::new(File::open(input_path).unwrap());
    let mut line = String::new();

    let mut divisors = Vec::new();

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
            let mut starting_items: VecDeque<Item> = VecDeque::new();

            for item in item_iter{
                match item.parse::<u128>() {
                    Ok(num) => starting_items.push_back(Item::new(num)),
                    _ => (),
                };
            }

            line.clear();
            reader.read_line(&mut line).unwrap();

            let mut operation;

            if &line[23..24] == "+"{
                operation = 1;
            } else {
                operation = 2;
            }
            let operation_comp_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
            //let operation_comp = operation_comp_str[0..operation_comp_str.len()-2].parse::<u32>();

            let operation_comp = match operation_comp_str[0..operation_comp_str.len()-1].parse::<u128>() {
                Ok(op_comp) => op_comp,
                Err(_) => {
                    operation = 3;
                    0},
            };
            line.clear();
            reader.read_line(&mut line).unwrap();
            let divisor_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
            let divisor = divisor_str[0..divisor_str.len()-1].parse::<u128>().unwrap();
            divisors.push(divisor);

            line.clear();
            reader.read_line(&mut line).unwrap();
            let true_monke_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
            let true_monke = true_monke_str[0..true_monke_str.len()-1].parse::<usize>().unwrap();

            line.clear();
            reader.read_line(&mut line).unwrap();
            let false_monke_str = line.split(' ').collect::<Vec<_>>().pop().unwrap();
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
    for i in 0..monkes.len() {
        for j in 0..monkes[i].items.len() {
            monkes[i].items[j].init_remainders(&divisors);
        }
    }
    divisors
}

pub fn solve(problem: u8) {
    let input_path = "inputs/day_11.txt";
    let mut monkes: Vec<Monke> = Vec::new();

    let divisors = parse_input(input_path, &mut monkes);
    
    if problem == 1 {
        for _i in 0..20 {
            for j in 0..monkes.len() {
                for _k in 0..monkes[j].items.len() {
                    let target_monke: usize;
                    let mut item: Item;
                    {
                        item = monkes[j].items.pop_front().unwrap();
                        monkes[j].items_inspected = monkes[j].items_inspected + 1;

                        if monkes[j].operation == 1  {
                            item.base = item.base + monkes[j].operation_comp;
                        } else if monkes[j].operation == 2 {
                            item.base = item.base*monkes[j].operation_comp;
                        } else {
                            item.base = item.base * item.base;
                        }
                        item.base = item.base /3;

                        if item.base % monkes[j].divisor == 0 {
                            target_monke = monkes[j].true_monke;
                        } else {
                            target_monke = monkes[j].false_monke;
                        }
                    }
                    monkes[target_monke].items.push_back(item)
                }
            }
        }
    } else {
        for _i in 0..10000 {
            for j in 0..monkes.len() {
                for _k in 0..monkes[j].items.len() {

                    let target_monke: usize;
                    let mut item = monkes[j].items.pop_front().unwrap();
                    {
                        monkes[j].items_inspected = monkes[j].items_inspected + 1; 

                        let remainder = item.get_remainder(&divisors, monkes[j].divisor, monkes[j].operation, monkes[j].operation_comp);

                        
                        if remainder == 0{
                            target_monke = monkes[j].true_monke;
                        } else {
                            target_monke = monkes[j].false_monke;
                        }
                    }
                    monkes[target_monke].items.push_back(item);
                    }
                }
            }
    }

    let mut max_1:u128  = 0;
    let mut max_2:u128  = 0;
    for monke in &monkes {
        if monke.items_inspected > max_1{
            max_2 = max_1;
            max_1 = monke.items_inspected;
        } else if monke.items_inspected > max_2 {
            max_2 = monke.items_inspected;
        }
    }

    print!("The answer to day 6 problem {} is: {}\n", problem, max_1*max_2);
}