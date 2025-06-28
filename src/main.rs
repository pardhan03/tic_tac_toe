use std::io::{self, Read};

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> Board {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}

fn print_board(board: &Board) {
    println!("\n  0   1   2"); // Column headers
    for (i, row) in board.iter().enumerate() {
        print!("{}", i); // Row header
        for (j, cell) in row.iter().enumerate() {
            print!(" {} ", cell);
            if j < BOARD_SIZE - 1 {
                print!("|");
            }
        }
        println!(); // Newline after each row
        if i < BOARD_SIZE - 1 {
            println!(" ---+---+---");
        }
    }
    println!();
}

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!("Player {} input (row col):", current_player);

        let mut input = String::new(); // Make it mutable
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }

        let coordinates: Vec<usize> = input
            .trim() // Remove newline and extra spaces
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok()) // safely parse
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }

        println!(
            "Invalid input. Please enter two numbers (0-{}) separated by a space, for an empty cell.",
            BOARD_SIZE - 1
        );
    }
}

fn check_winner(current_player: char, board: &Board) -> bool {
    // check along the row
    for row in 0..BOARD_SIZE {
        if board[row][0] == current_player
            && board[row][1] == current_player
            && board[row][2] == current_player
        {
            return true;
        }
    }

    // check along the col
    for col in 0..BOARD_SIZE {
        if board[0][col] == current_player
            && board[1][col] == current_player
            && board[2][col] == current_player
        {
            return true;
        }
    }

    // check diagonally
    if (board[0][0] == current_player
        && board[1][1] == current_player
        && board[2][2] == current_player)
        || (board[0][2] == current_player
            && board[1][1] == current_player
            && board[2][0] == current_player)
    {
        return true;
    }

    return false;
}

fn check_drwa(board: &Board) -> bool{
    for row in board{
        for cell in row{
            if *cell == ' '{
                return false;
        }
    }
}
    return true;
}

fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current Board:");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        if check_winner(current_player, &board){
            println!("Player {} is winnder", current_player);
            break;
        }

        if check_drwa(&board){
            println!("Game is Draw");
        }

        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        }
    }
}

fn main() {
    println!("Welcome to the Tic Tac Toe");
    play_game();
}
