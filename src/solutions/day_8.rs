use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
fn print_map(map:&[[u8; 99]; 99]) {
    for i in 0..map.len() {
        print!("{:?}\n", map[i]);
    }
}

pub fn solve(problem: u8) {

    let input_path = "inputs/day_8.txt";
    //let input_path = "sample_inputs/day_8_sample.txt";
    let reader = BufReader::new(File::open(input_path).unwrap());

    let gridsize: usize = 99;

    let mut map: [[u8; 99]; 99] = [[0; 99]; 99];
    for (j, line) in reader.lines().enumerate(){
        let line_contents = line.unwrap();
        for i in 0..gridsize {
            map[j][i] = (&line_contents[i..i+1]).parse::<u8>().unwrap();
        }
    }


    if problem == 1 {
        let mut mask = [[4; 99]; 99];
        for i in 0..gridsize {
            mask[0][i] = 4;
            mask[gridsize-1][i] = 4;
            mask[i][0] = 4;
            mask[i][gridsize-1] = 4;
        }

        for i in 1..(gridsize-1) {
            let mut max_l = map[i][0];
            let mut max_r = map[i][gridsize-1];
            let mut max_up = map[0][i];
            let mut max_down = map[gridsize-1][i];
            for j in 1..(gridsize-1) {
                let left = map[i][j];
                let right = map[i][gridsize-1-j];
                let up = map[j][i];
                let down = map[gridsize-1-j][i];

                if left <= max_l { // seraching left to right
                    mask[i][j] = mask[i][j] -1;
                } else {
                    max_l = left;
                }

                if right <= max_r { // seraching right to left
                    mask[i][gridsize-1-j] = mask[i][gridsize-1-j] -1;
                } else {
                    max_r = right;
                }

                if up <= max_up { // seraching up to down
                    mask[j][i] = mask[j][i] -1;
                } else {
                    max_up = up;
                }

                if down <= max_down { // seraching down to up
                    mask[gridsize-1-j][i] = mask[gridsize-1-j][i] -1;
                } else {
                    max_down = down;
                }
            }
        }

        let mut count = 0;
        for i in 0..gridsize {
            for j in 0..gridsize {
                if mask[i][j] != 0 {
                    count = count + 1;
                }
            }
        }
        print!("{}\n", count);
    } else {

        let mut max_score = 0;

        for col in 0..(gridsize) {
            for row in 0..(gridsize) {
                let height = map[row][col];
                let mut dists = [0; 4];
                let mut blocked = [0; 4];
                for k in 1..gridsize {
                    if row+k < gridsize && blocked[0] == 0{
                        dists[0] = dists[0] + 1;
                        if height <= map[row+k][col]{
                            blocked[0] = 1;
                        }
                    }
                    if row >= k && blocked[1] != 1{
                        dists[1] = dists[1] + 1;
                        if height <= map[row-k][col]{
                            blocked[1] = 1;
                        }
                    }

                    if col+k < gridsize && blocked[2] != 1{
                        dists[2] = dists[2] + 1;
                        if height <= map[row][col +k]{

                            blocked[2] = 1;
                        }
                    }

                    if col >= k && blocked[3] != 1{
                        dists[3] = dists[3] + 1;
                        if height <= map[row][col-k]{
                            blocked[3] = 1;
                        }
                    }
                    
                    if row+k > gridsize  {
                        blocked[0] = 1;
                    }

                    if k > row  {
                        blocked[1] = 1;
                    }
                    
                    if col+k > gridsize  {
                        blocked[2] = 1;
                    }

                    if k > col  {
                        blocked[3] = 1;
                    }

                    if blocked[0] == 1 && blocked[1] == 1 && blocked[2] == 1 && blocked[3] == 1 {
                        break;
                    }


                }
                let score = dists[0]*dists[1]*dists[2]*dists[3];

                if score > max_score {
                    max_score = score;
                }
            }
        } print!("{}\n", max_score)

    }
}