use std::{env, fs, thread, time::Duration};

use crate::canvas::Canvas;

mod canvas;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut canvas = Canvas::new(64, 64);

    if args.len() >= 2 {
        let file = fs::read_to_string(args.get(1).unwrap().to_string()).expect("File not found.");
        let mut x = 0;
        let mut y = 0;

        for character in file.chars() {
            match character {
                '&' => {
                    canvas.pixels[x][y] = true;
                    x += 1;
                },
                '_' => {
                    canvas.pixels[x][y] = false;
                    x += 1;
                }
                '\n' => {
                    x = 0;
                    y += 1;
                },
                _ => panic!("Invalid file.")
            }
        }
    } else {
        for x in 0..64 {
            for y in 0..64 {
                canvas.pixels[x][y] = rand::random_bool(0.2);
            }
        }
    }

    let mut working_vec: Vec<Vec<bool>> = vec![vec![false; 64]; 64];
    
    canvas.push();
    loop {
        thread::sleep(Duration::from_secs_f32(0.1));

        for row in working_vec.iter_mut() {
            row.fill(false);
        }

        for x in 0..64 {
            for y in 0..64 {
                let prev_x = if x == 0 {
                    63
                } else {
                    x - 1
                };

                let prev_y = if y == 0 {
                    63
                } else {
                    y - 1
                };

                let next_x = if x == 63 {
                    0
                } else {
                    x + 1
                };

                let next_y = if y == 63 {
                    0
                } else {
                    y + 1
                };

                let mut neighbors = 0;
                if canvas.pixels[prev_x][prev_y] { neighbors += 1; }
                if canvas.pixels[x][prev_y] { neighbors += 1; }
                if canvas.pixels[next_x][prev_y] { neighbors += 1; }
                if canvas.pixels[prev_x][y] { neighbors += 1; }
                if canvas.pixels[next_x][y] { neighbors += 1; }
                if canvas.pixels[prev_x][next_y] { neighbors += 1; }
                if canvas.pixels[x][next_y] { neighbors += 1; }
                if canvas.pixels[next_x][next_y] { neighbors += 1; }

                if neighbors == 3 {
                    working_vec[x][y] = true;
                } else if neighbors == 2 && canvas.pixels[x][y] {
                    working_vec[x][y] = true;
                }
            }
        
        }

        canvas.pixels = working_vec.clone();
        canvas.push();
    }
}

