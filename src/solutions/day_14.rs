use std::io::{prelude::*, BufReader};
use std::fs::File;

const HEIGHT :usize = 167;
const WIDTH  :usize = 90;
const WIDTH_START  : usize = 470;
const HEIGHT_START : usize = 0;

fn parse_maze(input_path: &str, problem: u8) -> [[char; WIDTH]; HEIGHT] {
    let reader = BufReader::new(File::open(input_path).unwrap());
    let mut maze:[[char; WIDTH]; HEIGHT] = [[' '; WIDTH]; HEIGHT];

    for line in reader.lines() {
        let line_content = line.unwrap();
        let vec = line_content.split(" -> ").collect::<Vec<_>>();

        for i in 1..vec.len() {
            let point_vec_1 = vec[i-1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let point_vec_2 = vec[i].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

            let x1 = point_vec_1[0] - WIDTH_START;
            let y1 = point_vec_1[1] - HEIGHT_START;
            let x2 = point_vec_2[0] - WIDTH_START;
            let y2 = point_vec_2[1] - HEIGHT_START;

            if x1 == x2 {
                if y1 < y2 {
                    for j in 0..(y2-y1){
                        maze[y1+j][x1] = '#';
                    }
                } else {
                    for j in 0..(y1-y2){
                        maze[y2+j][x1] = '#';
                    }
                }
            } else {
                if x1 < x2 {
                    for j in 0..(x2-x1)+1{
                        maze[y1][x1+j] = '#';
                    }
                } else {
                    for j in 0..(x1-x2)+1{
                        maze[y2][x2+j] = '#';
                    }
                }
            }
        }
    }
    maze[0][500-WIDTH_START] = '+';
    if problem != 1 {
        for i in 0..WIDTH{
            maze[HEIGHT-1][i] = '#'
        }
    }

    maze
}

fn draw_maze(maze: &[[char; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", maze[i][j]);
        }
        print!("\n");
    }
    for _ in 0..WIDTH {
        print!("-");
    }
    print!("\n");
}

pub fn solve(problem: u8, draw: bool) {
    let input_path = "inputs/day_14.txt";
    let mut maze = parse_maze(&input_path, problem);

    let mut x;
    let mut y;

    let mut left_counter  = 0;
    let mut right_counter = 0;

    let mut num_particles = 0;
    'outer: loop {
        num_particles = num_particles + 1;
        x = 500-WIDTH_START;
        y = 0;

        loop {
            if maze[y+1][x] == ' ' {
                y = y + 1;
            } else if problem == 2 && x == 1 && maze[y+1][x-1] == ' ' {
                if left_counter == HEIGHT - y - 3{
                    left_counter = 0;
                    maze[y+1][x-1] = '#';
                } else {
                    left_counter = left_counter + 1
                }
                break;
            } else if problem == 2 && x == WIDTH-2 && maze[y+1][x+1] == ' ' {

                if right_counter == HEIGHT - y - 3 {
                    right_counter = 0;
                    maze[y+1][x+1] = '#';
                    //draw_maze(&maze);
                } else {
                    right_counter = right_counter + 1
                }
                break;
            } else if maze[y+1][x-1] == ' '{
                y = y + 1;
                x = x - 1;
            } else if maze[y+1][x+1] == ' ' {
                y = y + 1;
                x = x + 1;
            } else {
                maze[y][x] = 'O';
                if problem == 2 && y == 0 && x == 500-WIDTH_START{
                    break 'outer
                } else {
                    break;
                }
            }
            if problem == 1 && y == HEIGHT-1 {
                num_particles = num_particles - 1;
                break 'outer;
            }
        }
    }
    if draw {
        draw_maze(&maze);
    }
    print!("The answer to day 14 problem {} is: {}\n", problem, num_particles);
}