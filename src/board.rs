use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

use crate::utils::*;

pub struct Board {
    board: Vec<Vec<cell_state::CellState>>,
    cell_state: HashMap<cell_state::CellState, char>,
    count_to_win: usize,
}

pub enum MoveResponse {
    Empty,
    Win(char),
}

impl Board {
    // pub fn board(self) -> &Vec<Vec<cell_state::CellState>> {
    //     &self.board
    // }

    pub fn new(size: usize) -> Board {
        Board {
            board: vec![vec![cell_state::CellState::Empty; size]; size],
            cell_state: HashMap::from([
                (cell_state::CellState::Empty, ' '),
                (cell_state::CellState::Player1, 'x'),
                (cell_state::CellState::Player2, 'o'),
            ]),
            count_to_win: 3,
        }
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
        for j in 1..(board_side_len * 3 + board_side_len) {
            if j % 4 == 0 {
                line.push(delimiter);
            } else {
                line.push(fill);
            }
        }
        line.push(right_side);
        line.push('\n');
        line
    }

    pub fn render_board(&self) -> String {
        let mut result = String::new();
        if self.board.is_empty() {}
        result.push_str(&Board::render_row_line(
            '━',
            '┳',
            '╭',
            '╮',
            self.board.len(),
        ));
        for (i, row) in (&self.board).into_iter().enumerate() {
            if i > 0 {
                result.push_str(&Board::render_row_line(
                    '━',
                    '╋',
                    '┣',
                    '┫',
                    self.board.len(),
                ));
            }
            result.push_str(&(self.board.len() - i - 1).to_string());
            for item in row.into_iter() {
                result.push('┃');
                result.push_str(&format!("{: ^3}", self.cell_state.get(&item).unwrap()));
            }
            result.push('┃');
            result.push('\n');
        }
        result.push_str(&Board::render_row_line(
            '━',
            '┻',
            '╰',
            '╯',
            self.board.len(),
        ));
        result.push_str("  ");
        for i in 1..(self.board.len() + 1) {
            result.push_str(&format!("{: ^3} ", i));
        }
        result
    }

    pub fn read_point(input: &impl io::Read) -> Result<point::Point, String> {
        Ok(point::Point::new(0, 0))
    }

    fn check_cell(&self, p: point::Point) -> Result<point::Point, String> {
        if p.x() >= self.board.len() || p.y() >= self.board.len() {
            return Err(String::from("Select a value from the suggested range."));
        }
        if self.board[p.y()][p.x()] != cell_state::CellState::Empty {
            return Err(String::from("This cell is occupied! Choose another one."));
        }
        Ok(p)
    }

    pub fn do_move(&self, p: point::Point, step: usize) -> Result<MoveResponse, String> {
        match self.check_cell(p) {
            Err(error) => return Err(error),
            _ => {}
        }
        if self.chech_win(p, self.count_to_win) {
            Ok(MoveResponse::Win('S'))
        } else {
            Ok(MoveResponse::Empty)
        }
    }

    fn chech_win(&self, last_point: point::Point, count_to_win: usize) -> bool {
        let mut cnt = 1usize;
        for i in 1..count_to_win {
            if last_point.x() >= i
                && self.board[last_point.y()][last_point.x() - i]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        for i in 1..count_to_win {
            if last_point.x() + i < self.board.len()
                && self.board[last_point.y()][last_point.x() + i]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        let result_hor = cnt >= count_to_win;

        cnt = 1;
        for i in 1..count_to_win {
            if last_point.y() >= i
                && self.board[last_point.y() - i][last_point.x()]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        for i in 1..count_to_win {
            if last_point.y() + i < self.board.len()
                && self.board[last_point.y() + i][last_point.x()]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        let result_ver = cnt >= count_to_win;

        cnt = 1;
        for i in 1..count_to_win {
            if last_point.x() >= i
                && last_point.y() >= i
                && self.board[last_point.y() - i][last_point.x() - i]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        for i in 1..count_to_win {
            if last_point.x() + i < self.board.len()
                && last_point.y() + i < self.board.len()
                && self.board[last_point.y() + i][last_point.x() + i]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        let result_diag_p = cnt >= count_to_win;
        cnt = 1;

        cnt = 1;
        for i in 1..count_to_win {
            if last_point.x() >= i
                && last_point.y() + i < self.board.len()
                && self.board[last_point.y() + i][last_point.x() - i]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        for i in 1..count_to_win {
            if last_point.x() + i < self.board.len()
                && last_point.y() >= i
                && self.board[last_point.y() - i][last_point.x() + i]
                    == self.board[last_point.y()][last_point.x()]
            {
                cnt += 1;
            } else {
                break;
            }
        }
        let result_diag_n = cnt >= count_to_win;
        cnt = 1;

        result_hor || result_ver || result_diag_p || result_diag_n
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_board())
    }
}
