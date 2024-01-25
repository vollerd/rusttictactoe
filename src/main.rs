mod grid;

use std::collections::HashMap;
use std::io::{self, Write};

use console::Term;
use colored::*;

fn main() {
    let mut array: [[i32; 3]; 3] = [[0; 3]; 3];
    let mut dic = HashMap::new();
    let term = Term::stdout();
    let mut coord = String::new();
    let mut ask_input = true;

    dic.insert(0, '　');
    dic.insert(1, '〇');
    dic.insert(2, 'Ｘ');


    loop {
        term.clear_screen().unwrap();
        grid::draw_grid(&dic, &array);
        grid::check_winner(array);

        while ask_input{
            ask_input = false;
            coord.clear();
            print!("Enter a coordinate {}: ", "(e.g. B3)".black());
            io::stdout().flush().unwrap(); // flush stdout to ensure the print! message appears before the input
            io::stdin().read_line(&mut coord).unwrap();
            match coord.trim() {
                "A3" => { array[0][0] = 1; },
                "A2" => { array[1][0] = 1; },
                "A1" => { array[2][0] = 1; },
                "B3" => { array[0][1] = 1; },
                "B2" => { array[1][1] = 1; },
                "B1" => { array[2][1] = 1; },
                "C3" => { array[0][2] = 1; },
                "C2" => { array[1][2] = 1; },
                "C1" => { array[2][2] = 1; },
                _ => ask_input = true,
            }
        }
        ask_input = true;

        grid::check_winner(array);
        grid::enemy_turn(&mut array);
    }
}
