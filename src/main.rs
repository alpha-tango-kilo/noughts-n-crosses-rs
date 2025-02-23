use std::io;
use std::char;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Empty,
    Nought,
    Cross,
}

fn main() {
    println!("Welcome to Rust Noughts & Crosses!");

    let mut board: [CellState; 9] = [CellState::Empty; 9];
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
            if !(choice >= 1 && choice <= 9 && board[choice - 1] == CellState::Empty) {
                println!("Invalid number given, please choose a number available on the board");
            } else {
                board[choice - 1] = if turn { CellState::Nought } else { CellState::Cross };
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

fn check_win(arr: &[CellState; 9]) -> bool {
    // Diagonals
    if (arr[0] != CellState::Empty && arr[0] == arr[4] && arr[0] == arr[8]) || (arr[2] != CellState::Empty && arr[2] == arr[4] && arr[2] == arr[6]) {
        return true;
    } else {
        for n in 0..=2 {
            let offset = n * 3;
            // First clause rows, second clause columns
            if (arr[offset] != CellState::Empty && arr[offset] == arr[offset + 1] && arr[offset] == arr[offset + 2]) || (arr[n] != CellState::Empty && arr[n] == arr[n + 3] && arr[n] == arr[n + 6]) {
                return true;
            }
        }
    }
    false
}

fn draw_board(arr: &[CellState; 9]) {
    for (n, &cell) in arr.iter().enumerate() {
        let cell_char = match cell {
            CellState::Empty  => char::from_digit((n + 1) as u32, 10).expect("Enumeration index was not an int"),
            CellState::Cross  => 'X',
            CellState::Nought => 'O',
        };
        print!(" {} ", cell_char);

        if n % 3 != 2 {
            print!("|");
        } else if n != 8 {
            println!("\n-----------");
        }
    }
    println!();
}
