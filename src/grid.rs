use std::collections::HashMap;
use rand::Rng;

pub fn draw_grid(dic: &HashMap<i32, char>, array: &[[i32; 3]; 3]) {
    println!(
        "3 {}|{}|{}",
        dic.get(&array[0][0]).unwrap_or(&' '),
        dic.get(&array[0][1]).unwrap_or(&' '),
        dic.get(&array[0][2]).unwrap_or(&' ')
    );
    println!("  ー+ー+ー");
    println!(
        "2 {}|{}|{}",
        dic.get(&array[1][0]).unwrap_or(&' '),
        dic.get(&array[1][1]).unwrap_or(&' '),
        dic.get(&array[1][2]).unwrap_or(&' ')
    );
    println!("  ー+ー+ー");
    println!(
        "1 {}|{}|{}",
        dic.get(&array[2][0]).unwrap_or(&' '),
        dic.get(&array[2][1]).unwrap_or(&' '),
        dic.get(&array[2][2]).unwrap_or(&' ')
    );
    println!("　Ａ Ｂ Ｃ");
}

fn check_row(row: &mut [i32; 3], ones: usize, twos: usize, zeros: usize) -> bool {
    let one_count = row.iter().filter(|&&x| x == 1).count();
    let zero_count = row.iter().filter(|&&x| x == 0).count();
    let two_count = row.iter().filter(|&&x| x == 2).count();

    if one_count == ones && two_count == twos && zero_count == zeros {
        let empty_index = row.iter().position(|&x| x == 0).unwrap();
        row[empty_index] = 2;
        true
    } else {
        false
    }
}

fn check_col(col: usize, array: &mut [[i32; 3]; 3], ones: usize, twos: usize, zeros: usize) -> bool {
    let one_count = array.iter().filter(|&row| row[col] == 1).count();
    let zero_count = array.iter().filter(|&row| row[col] == 0).count();
    let two_count = array.iter().filter(|&row| row[col] == 2).count();

    if one_count == ones && two_count == twos && zero_count == zeros {
        let zero_index = array.iter().position(|&row| row[col] == 0).unwrap();
        array[zero_index][col] = 2;
        true
    } else {
        false
    }
}


pub fn enemy_turn(array: &mut [[i32; 3]; 3]) {
    // Check rows for winning move
    for row in array.iter_mut() {
        if check_row(row, 0, 2, 1) {
            return;
        }
    }
    
    // Check columns for winning move
    for col in 0..3 {
        if check_col(col, array, 0, 2, 1) {
            return;
        }
    }
    // Check main diagonal (from top-left to bottom-right)
    if (array[0][0] == 0 && array[1][1] == 2 && array[2][2] == 2) || (array[0][0] == 2 && array[1][1] == 0 && array[2][2] == 2) || (array[0][0] == 2 && array[1][1] == 2 && array[2][2] == 0) {
        array[0][0] = 2;
        array[1][1] = 2;
        array[2][2] = 2;
        return;
    }

    // Check secondary diagonal (from top-right to bottom-left)
    if (array[2][0] == 0 && array[1][1] == 2 && array[0][2] == 2) || (array[2][0] == 2 && array[1][1] == 0 && array[0][2] == 2) || (array[2][0] == 2 && array[1][1] == 2 && array[0][2] == 0) {
        array[2][0] = 2;
        array[1][1] = 2;
        array[0][2] = 2;
        return;
    }

    // Check rows for defensive move
    for row in array.iter_mut() {
        if check_row(row, 2, 0, 1) {
            return;
        }
    }
    // Check columns for defensive move
    for col in 0..3 {
        if check_col(col, array, 2, 0, 1) {
            return;
        }
    }
    // Check main diagonal (from top-left to bottom-right)
    let ones = (0..3).filter(|&i| array[i][i] == 1).count();
    let zeros = (0..3).filter(|&i| array[i][i] == 0).count();

    if ones == 2 && zeros == 1 {
        let zero_index = (0..3).position(|i| array[i][i] == 0).unwrap();
        array[zero_index][zero_index] = 2;
        return;
    }

    // Check secondary diagonal (from top-right to bottom-left)
    let ones = (0..3).filter(|&i| array[i][2 - i] == 1).count();
    let zeros = (0..3).filter(|&i| array[i][2 - i] == 0).count();

    if ones == 2 && zeros == 1 {
        let zero_index = (0..3).position(|i| array[i][2 - i] == 0).unwrap();
        array[zero_index][2 - zero_index] = 2;
        return;
    }

    // Check rows for decent move
    for row in array.iter_mut() {
        if check_row(row, 0, 1, 2) {
            return;
        }
    }

    // Check cols for decent move
    for col in 0..3 {
        if check_col(col, array, 0, 1, 2) {
            return;
        }
    }

    // Check main diagonal (from top-left to bottom-right)
    let twos = (0..3).filter(|&i| array[i][i] == 2).count();
    let zeros = (0..3).filter(|&i| array[i][i] == 0).count();

    if twos == 1 && zeros == 2 {
        let zero_index = (0..3).position(|i| array[i][i] == 0).unwrap();
        array[zero_index][zero_index] = 2;
        return;
    }
    // Check secondary diagonal (from top-right to bottom-left)
    let twos = (0..3).filter(|&i| array[i][2 - i] == 2).count();
    let zeros = (0..3).filter(|&i| array[i][2 - i] == 0).count();

    if twos == 1 && zeros == 2 {
        let zero_index = (0..3).position(|i| array[i][2 - i] == 0).unwrap();
        array[zero_index][2 - zero_index] = 2;
        return;
    }
    // If no good move is found, play randomly
    let mut rng = rand::thread_rng();
    let mut row;
    let mut col;
    
    loop {
        row = rng.gen_range(0..3);
        col = rng.gen_range(0..3);
    
        if array[row][col] == 0 {
            break;
        }
    }
    array[row][col] = 2;
}

pub fn check_winner(array: [[i32; 3]; 3]) {
    let mut winner = 0;

    for i in 0..3 {
        if array[i][0] == array[i][1] && array[i][1] == array[i][2] && array[i][0] != 0 {
            winner = array[i][0];
        }
    }

    for i in 0..3 {
        if array[0][i] == array[1][i] && array[1][i] == array[2][i] && array[0][i] != 0 {
            winner = array[0][i];
        }
    }

    if array[0][0] == array[1][1] && array[1][1] == array[2][2] && array[0][0] != 0 {
        winner = array[0][0];
    }

    if array[0][2] == array[1][1] && array[1][1] == array[2][0] && array[0][2] != 0 {
        winner = array[0][2];
    }

    if winner == 1 {
        println!("You win!");
        std::process::exit(0);
    } else if winner == 2 {
        println!("You lose!");
        std::process::exit(0);
    }

    // check for draw 
    for row in array.iter() {
        for &cell in row.iter() {
            if cell == 0 {
                return; // still room to play
            }
        }
    }
    println!("It's a draw!");
    std::process::exit(0);
}