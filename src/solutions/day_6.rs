
use std::fs::File;
use std::io::{ self, BufRead, BufReader };



pub fn solve() {

    let filename = "inputs/day_6.txt";
    const window_size:usize = 14; // 4 for problem 1, 14 for problem 2.

    let file = File::open(filename).unwrap(); 

    let mut line = String::new();
    let num_bytes = io::BufReader::new(file).read_line(&mut line).unwrap(); 

    let bytes = line.as_bytes();

    let mut window:[u8; window_size] = [0; window_size];
    let mut hash_tab:[u8; 26] = [0; 26];
    
    for i in 0..window_size{
        let elem_to_add =  bytes[i] - 97;
        window[i%window_size] = elem_to_add;
        hash_tab[elem_to_add as usize] += 1;
    }

    for i in window_size..num_bytes{
        let elem_to_add = bytes[i] - 97;
        let elem_to_pop = window[i%window_size];

        hash_tab[elem_to_pop as usize] -= 1;
        hash_tab[elem_to_add as usize] += 1;
        window[i%window_size] = elem_to_add; //overrides previous element

        //check new hash_table
        let mut max = 0;
        for j in 0..window_size{
            let count = hash_tab[window[j] as usize];
            if count > max{
                max = count;
            }
        }
        if max == 1{
            print!("{:?}\n, {:?}\n", window, hash_tab);
            print!("Four unique elements in buffer, at iteration: {}\n", i+1);
            break;
        }
    }
}
