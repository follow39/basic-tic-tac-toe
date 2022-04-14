use std::{
    cell::Cell,
    io::{self, Write},
};

const BOARD_SIDE_LEN: usize = 3;
const COUNT_IN_A_ROW_FOR_WIN: u32 = 3;
const SYMBOL_PLAYER_1: char = 'x';
const SYMBOL_PLAYER_2: char = 'o';
const MESSAGE_WRONG_INPUT: &str = "Select a value from the suggested range.";
const MESSAGE_CELL_NOT_EMPTY: &str = "This cell is occupied! Choose another one.";

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
        let current_player = if step_counter % 2 == 1 {
            CellState::Player1
        } else {
            CellState::Player2
        };
        std::process::Command::new("clear").status().unwrap();
        println!("{}", render_board(&board));
        println!(
            "Player '{}' your turn!",
            convert_cell_state_to_char(&current_player)
        );

        let p: Point = loop {
            let p = read_input_point();
            let input_result = point_is_suitable(&board, &p);
            match input_result.0 {
                true => break p,
                false => println!("{}", input_result.1),
            };
        };

        do_move(&mut board, current_player, &p);

        if check_player_win(&board, COUNT_IN_A_ROW_FOR_WIN, &p) {
            println!(
                "Congratulations! Player '{}' won!",
                convert_cell_state_to_char(&current_player)
            );
            break;
        }
        if step_counter == 9 {
            println!("Draw!");
            break;
        }
        step_counter += 1;
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

fn convert_cell_state_to_char(state: &CellState) -> char {
    match state {
        CellState::Empty => ' ',
        CellState::Player1 => SYMBOL_PLAYER_1,
        CellState::Player2 => SYMBOL_PLAYER_2,
    }
}

fn do_move(
    board: &mut [[CellState; BOARD_SIDE_LEN]; BOARD_SIDE_LEN],
    player: CellState,
    p: &Point,
) {
    board[p.y][p.x] = player;
}

fn point_is_suitable(
    board: &[[CellState; BOARD_SIDE_LEN]; BOARD_SIDE_LEN],
    p: &Point,
) -> (bool, String) {
    if p.x >= BOARD_SIDE_LEN || p.y >= BOARD_SIDE_LEN {
        return (false, String::from(MESSAGE_WRONG_INPUT));
    }
    match board[p.y][p.x] {
        CellState::Empty => (true, String::new()),
        _ => (false, String::from(MESSAGE_CELL_NOT_EMPTY)),
    }
}

fn read_input_point() -> Point {
    println!("Choose you cell({}-{}):", 0, BOARD_SIDE_LEN - 1);
    print!("x - ");
    io::stdout().flush().unwrap();
    read!(x as usize);
    print!("y - ");
    io::stdout().flush().unwrap();
    read!(y as usize);
    println!();
    Point {
        x,
        y: (BOARD_SIDE_LEN - y - 1),
    }
}

fn check_player_win(
    board: &[[CellState; BOARD_SIDE_LEN]; BOARD_SIDE_LEN],
    count_for_win: u32,
    last_point: &Point,
) -> bool {
    let mut cnt: u32;
    
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
        print!("{}", (BOARD_SIDE_LEN - i - 1));
        for item in row.into_iter() {
            print!("┃");
            print!("{: ^3}", convert_cell_state_to_char(&item));
        }
        println!("┃");
    }
    if !board.is_empty() {
        println!("{}", render_row_line('━', '┻', '╰', '╯', BOARD_SIDE_LEN));
    }
    print!("  ");
    for (i, _) in board.into_iter().enumerate() {
        print!("{: ^3} ", i)
    }
    String::new()
}
