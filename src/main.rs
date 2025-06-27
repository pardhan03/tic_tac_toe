use std::io;

const PLAYER_X:char = 'X';
const PLAYER_O:char = 'O';

const BOARD_SIZE:usize = 3;

type Board = [[char;BOARD_SIZE];BOARD_SIZE];

fn initialize_board() ->Board{
    return [[' ';BOARD_SIZE];BOARD_SIZE]
}

fn print_board(board: &Board){
    for row in board{
        for cell in row{
            println!("{}", cell);
        }
        println!();
    }
}

fn play_game(){

}

fn main() {
    println!("Welcome to the Tic Tac Toe");
    play_game();
}
