use std::io;
use std::char;

fn main() {
    println!("Welcome to Rust Noughts & Crosses!");

    let mut board: [i8; 9] = [0; 9]; // 0 for untaken, 1 for knought, -1 for cross
    let mut turn = true;

    loop {
        let player = if turn { "Noughts" } else { "Crosses" };

        println!("{}' turn", player);
        draw_board(&board);

        loop {
            println!("Where would you like to play? (Numbers in cells indicate available positions)");

            // Take input
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            // Parse
            let choice: usize = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That wasn't a number");
                    continue;
                },
            };

            // Validate
            if !(choice >= 1 && choice <= 9 && board[choice - 1] == 0) {
                println!("Invalid number given, please choose a number available on the board");
            } else {
                board[choice - 1] = if turn { 1 } else { -1 };
                break;
            }
        }

        // Check for a win
        if check_win(&board) {
            println!("{} won! Congratulations", player);
            draw_board(&board);
            break;
        }
        turn = !turn;
    }
}

fn check_win(&arr: &[i8; 9]) -> bool {
    // Diagonals
    if (arr[0] != 0 && arr[0] == arr[4] && arr[0] == arr[8]) || (arr[2] != 0 && arr[2] == arr[4] && arr[2] == arr[6]) {
        return true;
    } else {
        for n in 0..2 {
            let offset = n * 3;
            // First clause rows, second clause columns
            if (arr[offset] != 0 && arr[offset] == arr[offset + 1] && arr[offset] == arr[offset + 2]) || (arr[n] != 0 && arr[n] == arr[n + 3] && arr[n] == arr[n + 6]) {
                return true;
            }
        }
    }
    false
}

fn draw_board(&arr: &[i8; 9]) {
    for (n, &cell) in arr.iter().enumerate() {
        print!(" {} ", if cell == 0 { char::from_digit((n + 1) as u32, 10).expect("Enumeration index was not an int") } else if cell == 1 { 'O' } else { 'X' });
        if n % 3 != 2 {
            print!("|");
        } else if n != 8 {
            println!("\n-----------");
        }
    }
    println!();
}
