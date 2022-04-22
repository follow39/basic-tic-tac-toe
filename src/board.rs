use std::collections::HashMap;
use std::fmt;

use crate::utils::*;

pub struct Board {
    board: Vec<Vec<cell_state::CellState>>,
    cell_state_symbols: HashMap<cell_state::CellState, char>,
    count_to_win: usize,
}

pub enum MoveResponse {
    Empty,
    Win(char),
    Draw,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            board: vec![vec![cell_state::CellState::Empty; size]; size],
            cell_state_symbols: HashMap::from([
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
            result.push_str(&(self.board.len() - i).to_string());
            for item in row.into_iter() {
                result.push('┃');
                result.push_str(&format!(
                    "{: ^3}",
                    self.cell_state_symbols.get(&item).unwrap()
                ));
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

    pub fn read_point(
        &self,
        input: &mut impl std::io::BufRead,
        step: usize,
    ) -> Result<point::Point, String> {
        println!(
            "Player \'{}\' choose you cell:",
            *(self
                .cell_state_symbols
                .get(&Board::get_player(step))
                .unwrap())
        );
        let p = match point::Point::from(input) {
            Ok(value) => value,
            Err(error) => return Err(format!("Error: \"{}\"", error)),
        };
        Ok(p)
    }

    fn check_point_in_range(&self, p: point::Point) -> Result<point::Point, String> {
        if p.x() == 0 || p.x() > self.board.len() || p.y() == 0 || p.y() > self.board.len() {
            return Err(String::from("Select a value from the suggested range."));
        }
        Ok(p)
    }

    fn check_cell_empty(&self, p: point::Point) -> Result<point::Point, String> {
        if self.board[p.y()][p.x()] != cell_state::CellState::Empty {
            return Err(String::from("This cell is occupied! Choose another one."));
        }
        Ok(p)
    }

    fn get_player(step: usize) -> cell_state::CellState {
        if step % 2 == 0 {
            cell_state::CellState::Player2
        } else {
            cell_state::CellState::Player1
        }
    }

    pub fn do_move(&mut self, p: point::Point, step: usize) -> Result<MoveResponse, String> {
        let move_point = match self.check_point_in_range(p) {
            Ok(p) => point::Point::new(p.x() - 1, self.board.len() - p.y()),
            Err(error) => return Err(error),
        };
        match self.check_cell_empty(move_point) {
            Ok(move_point) => self.board[move_point.y()][move_point.x()] = Board::get_player(step),
            Err(error) => return Err(error),
        }

        if self.chech_win(move_point, self.count_to_win) {
            return Ok(MoveResponse::Win(
                *(self
                    .cell_state_symbols
                    .get(&Board::get_player(step))
                    .unwrap()),
            ));
        }
        if step == self.board.len().pow(2) {
            return Ok(MoveResponse::Draw);
        }
        Ok(MoveResponse::Empty)
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

        result_hor || result_ver || result_diag_p || result_diag_n
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_board())
    }
}
