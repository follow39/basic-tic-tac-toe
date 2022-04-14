use std::cmp;
use std::io::{self, Write};

const BOARD_SIDE_LEN: usize = 3;
const COUNT_IN_A_ROW_FOR_WIN: u32 = 3;
const SYMBOL_PLAYER_1: char = 'x';
const SYMBOL_PLAYER_2: char = 'o';

#[derive(Debug, Copy, Clone)]
enum CellState {
    Empty,
    Player1,
    Player2,
}

struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let mut board = [[CellState::Empty; BOARD_SIDE_LEN]; BOARD_SIDE_LEN];
    let mut step_counter = 1;

    loop {
        let current_player_symbol = if step_counter % 2 == 1 {
            SYMBOL_PLAYER_1
        } else {
            SYMBOL_PLAYER_2
        };
        std::process::Command::new("clear").status().unwrap();
        println!("{}", render_board(&board));
        println!("Player '{}' your turn!", current_player_symbol);

        while !do_move(&mut board, current_player_symbol, read_input_point()) {
            println!("This cell is occupied! Choose another one.");
        }

        step_counter += 1;

        if check_player_win(&board, current_player_symbol) {
            println!("Congratulations! Player '{}' won!", current_player_symbol);
        }
    }
}

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

fn do_move(
    board: &mut [[CellState; BOARD_SIDE_LEN]; BOARD_SIDE_LEN],
    player_symbol: char,
    p: Point,
) -> bool {
    match board[p.y][p.x] {
        CellState::Empty => {
            board[p.y][p.x] = if player_symbol == SYMBOL_PLAYER_1 {
                CellState::Player1
            } else {
                CellState::Player2
            };
        }
        _ => return false,
    }
    true
}

fn read_input_point() -> Point {
    loop {
        println!("Choose you cell({}-{}):", 1, BOARD_SIDE_LEN);
        print!("x - ");
        io::stdout().flush().unwrap();
        read!(x as usize);
        print!("y - ");
        io::stdout().flush().unwrap();
        read!(y as usize);
        println!();
        if x > 0 && x <= BOARD_SIDE_LEN && y > 0 && y <= BOARD_SIDE_LEN {
            return Point {
                x: x - 1,
                y: (BOARD_SIDE_LEN - y),
            };
        } else {
            print!("Select a value from the suggested range.");
        }
    }
}

fn check_player_win(
    board: &[[CellState; BOARD_SIDE_LEN]; BOARD_SIDE_LEN],
    player_symbol: char,
) -> bool {
    false
}

fn render_row_line(
    fill: char,
    delimiter: char,
    left_side: char,
    right_side: char,
    board_side_len: usize,
) -> String {
    let mut line = String::new();
    line.push(' ');
    line.push(left_side);
    for j in 1..(board_side_len * 3 + BOARD_SIDE_LEN) {
        if j % 4 == 0 {
            line.push(delimiter);
        } else {
            line.push(fill);
        }
    }
    line.push(right_side);
    line
}

fn render_board(board: &[[CellState; BOARD_SIDE_LEN]; BOARD_SIDE_LEN]) -> String {
    if !board.is_empty() {
        println!("{}", render_row_line('━', '┳', '╭', '╮', BOARD_SIDE_LEN));
    }
    for (i, row) in board.into_iter().enumerate() {
        if i > 0 {
            println!("{}", render_row_line('━', '╋', '┣', '┫', BOARD_SIDE_LEN));
        }
        print!("{}", (BOARD_SIDE_LEN - i));
        for item in row.into_iter() {
            print!("┃");
            print!(
                "{: ^3}",
                match item {
                    CellState::Empty => ' ',
                    CellState::Player1 => SYMBOL_PLAYER_1,
                    CellState::Player2 => SYMBOL_PLAYER_2,
                }
            );
        }
        println!("┃");
    }
    if !board.is_empty() {
        println!("{}", render_row_line('━', '┻', '╰', '╯', BOARD_SIDE_LEN));
    }
    print!("  ");
    for (i, _) in board.into_iter().enumerate() {
        print!("{: ^3} ", i + 1)
    }
    String::new()
}
