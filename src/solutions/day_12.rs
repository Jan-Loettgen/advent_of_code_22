use std::fs::File;
use std::io::{prelude::*, BufReader};
use arr_macro::arr;
use priority_queue::PriorityQueue;

//dims 43 ROWS 143 COLS
// A algorithm?

const ROWS: usize = 41;
const COLS: usize = 143;

const TARGET_X :usize = 120;
const TARGET_Y :usize = 20;
const START_X  :usize = 0;
const START_Y  :usize = 20;

#[derive(PartialEq, Eq, Hash)]
struct Node {
    connections : [u8; 4],
    height      : u8,
    h_score     : u32,
    g_score     : u32,
    x           : usize,
    y           : usize,
    visited     : bool
}

impl Node {
    fn new() -> Node {
        Node{
            x           : 4294967295, // maximum value storeable in u32
            y           : 4294967295, // maximum value storeable in u32
            connections : [0; 4],     // initalise all connections as 0
            height      : 255,        // maximum height storeable in u8
            h_score     : 4294967295, // maximum value storeable in u32
            g_score     : 4294967295, // maximum value storeable in u32
            visited     : false,
        }
    }
}

fn build_map(input_path: &str) -> [[Node; COLS]; ROWS]{

    let reader = BufReader::new(File::open(input_path).unwrap());
    let mut map: [[Node; COLS]; ROWS] = arr![arr![Node::new(); 143]; 41];

    for (i, line) in reader.lines().enumerate(){
        let line_contents = line.unwrap();
        let heights_row = &mut line_contents.as_bytes();
        for j in 0..COLS {
            let mut height = heights_row[j];

            if height == b'E' {
                height = b'z';
            } else if height == b'S'{
                height = b'a';
                map[i][j].g_score = 0; // this wil be the starting point so initalise cost to come to 0.
            }

            height = height - 97;

            // calculate h_score
            map[i][j].h_score = (((TARGET_X as i32) - (j as i32)).abs() + (TARGET_Y as i32) -(i as i32).abs()).abs() as u32;

            map[i][j].x = j;
            map[i][j].y = i;
            map[i][j].height = height;
        }
    };

    for i in 0..ROWS {
        for j in 0..COLS{
            let height = map[i][j].height;
            if j > 0 && height +1 >= map[i][j-1].height { 
                map[i][j].connections[2] = 1;
            }
            if j < COLS-1 && height +1 >= map[i][j + 1].height { 
                map[i][j].connections[0] = 1;
            }

            if i > 0 {
                if  height + 1 >= map[i-1][j].height { 
                    map[i][j].connections[1] = 1;
                }
                if map[i-1][j].height + 1 >= height {
                    // NEED TO DO PREVIOUS ROWS VERTICAL UP CONNECTION ON NEXT ROW
                    map[i-1][j].connections[3] = 1;
                }
            }
        }
    }
    map
}

fn draw_map(map: &[[Node; COLS]; ROWS]) {
    //print!("\x1B[2J\x1B[1;1H");
    for _j in 0..COLS+4 {
        print!("-");
    }
    print!("\n");
    for i in 0..ROWS {
        print!("| ");
        for j in 0..COLS {

            if j == TARGET_X && i == TARGET_Y{
                print!("\x1b[41m{}\x1b[0m", map[i][j].height/3);
            } else if j == START_X && i == START_Y{
                print!("\x1b[42m{}\x1b[0m", map[i][j].height/3);
            } else if map[i][j].visited {
                print!("\x1b[93m{}\x1b[0m", map[i][j].height/3);
            } else {
                print!("{}", map[i][j].height/3);
            }
        }
        print!(" |\n");
    }
    for _j in 0..COLS + 4 {
        print!("-");
    }
    print!("\n");
    print!("\x1b[44A\r\n");
}

pub fn solve(_problem: u8, draw: bool) {
    let input_path = "inputs/day_12.txt";

    // components for A*
    let mut map = build_map(input_path);
    let mut pq:PriorityQueue<(usize, usize), u32> = PriorityQueue::new();

    if draw{
        draw_map(&map);
    }
    pq.push((START_X, START_Y), 4294967295);

    while pq.len() > 0 {
        let (x, y) = pq.pop().unwrap().0;

        if x == TARGET_X && y == TARGET_Y{
            break;
        }

        let connections: [u8; 4];
        let g_score;
        {// hacking the rust scopes becuase to I am to dumb to get it to work otherwise
        let curr_node   = &mut map[y][x];
        curr_node.visited = true;
        g_score = curr_node.g_score;
        connections = curr_node.connections;
        }

        if connections[0] == 1 {
            let node = &mut map[y][x + 1];

            let tenative_gscore = g_score + 1;
            if tenative_gscore < node.g_score {
                node.g_score = tenative_gscore;
                pq.push((node.x, node.y), 4294967295 - node.g_score - node.h_score);
            }
        }

        if connections[1] == 1 {
            let node = &mut map[y - 1][x];
            let tenative_gscore = g_score + 1;
            if tenative_gscore < node.g_score {
                node.g_score = tenative_gscore;
                pq.push((node.x, node.y), 4294967295 - node.g_score - node.h_score);
            }
        }

        if connections[2] == 1 {
            let node = &mut map[y][x - 1];
            let tenative_gscore = g_score + 1;
            if tenative_gscore < node.g_score {
                node.g_score = tenative_gscore;
                pq.push((node.x, node.y), 4294967295 - node.g_score - node.h_score);
            }
        }

        if connections[3] == 1 {
            let node = &mut map[y + 1][x];
            let tenative_gscore = g_score + 1;
            if tenative_gscore < node.g_score {
                node.g_score = tenative_gscore;
                pq.push((node.x, node.y), 4294967295 - node.g_score - node.h_score);
            }
        }
        if draw{
            draw_map(&map);
        }
    }
    if draw{
        draw_map(&map);
        print!("\x1b[43B\r\n");

    }
    print!("The answer to day 6 problem {} is: {}\n", _problem, map[TARGET_Y][TARGET_X].g_score);
}