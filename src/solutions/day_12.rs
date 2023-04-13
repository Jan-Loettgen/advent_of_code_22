use std::fs::File;
use std::io::{prelude::*, BufReader};
use arr_macro::arr;

//dims 43 rows 143 cols

// djistras algorithm?
// A* algorithm?

struct Node {
    connections : [u8; 4],
    height      : u8,
    visited     : bool,
}

impl Node {
    fn new() -> Node {
        Node{
            connections : [0; 4],
            height      : 0,
            visited     : false,
        }
    }
}

pub fn solve(_problem: u8) {
    let input_path = "inputs/day_12.txt";
    let reader = BufReader::new(File::open(input_path).unwrap());

    const rows: usize = 41;
    const cols: usize = 143;

    let mut target_x:u8 = 0;
    let mut target_y: u8 = 0;
    let mut map: [[Node; cols]; rows] = arr![arr![Node::new(); 143]; 41];

    let mut line_contents: String = String::new();
    let mut line_prev   : String;

    for (i, line) in reader.lines().enumerate(){

        line_prev = line_contents;
        line_contents = line.unwrap();

        let heights_row_prev = line_prev.as_bytes();
        let heights_row = line_contents.as_bytes();

        for j in 0..cols {
            if j > 0 && heights_row[j] >= heights_row[j -1] - 1 && heights_row[j] <= heights_row[j-1] + 1 {
                map[i][j].connections[2] = 1;
            }
            if j < rows-1 && heights_row[j] >= heights_row[j + 1] - 1 && heights_row[j] <= heights_row[j + 1] + 1 {
                map[i][j].connections[0] = 1;
            }

            if i > 0 {
                if heights_row[j] >= heights_row_prev[j] - 1 && heights_row[j] <= heights_row_prev[j] + 1 {
                    map[i][j].connections[1] = 1;
                    map[i-1][j].connections[3] = 1;
                }
            }
            print!("{}\n",i);
            // NEED TO DO PREVIOUS ROWS VERTICAL UP CONNECTION ON NEXT ROW
            if i == rows-1 && (heights_row[j] >= heights_row_prev[j] - 1) && (heights_row[j] <= heights_row_prev[j] + 1) {
                map[i][j].connections[1] = 1;
            }
        }
    }

    print!("{:?}\n", map[0][0].connections);
    print!("{:?}\n", map[rows-1][0].connections);
    print!("{:?}\n", map[0][cols-1].connections);
    print!("{:?}\n", map[rows-1][cols-1].connections);
    print!("{:?}\n", map[1][1].connections);
}