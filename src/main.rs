use proceed;

mod board;
mod utils;

fn main() {
    let mut board = board::Board::new(3);
    let mut step = 1;

    loop {
        std::process::Command::new("clear").status().unwrap();
        println!("{}", board);

        let move_point: utils::point::Point =
            match board.read_point(&mut std::io::stdin().lock(), step) {
                Ok(point) => point,
                Err(error) => {
                    println!("Error: \"{}\"", error);
                    println!("Press any key to continue...");
                    proceed::proceed();
                    continue;
                }
            };

        match board.do_move(move_point, step) {
            Ok(response) => match response {
                board::MoveResponse::Win(player_symbol) => {
                    std::process::Command::new("clear").status().unwrap();
                    println!("{}", board);
                    println!("Congratulations! Player '{}' won!", player_symbol);
                    println!("Press any key to continue...");
                    proceed::proceed();
                    break;
                }
                board::MoveResponse::Draw => {
                    std::process::Command::new("clear").status().unwrap();
                    println!("{}", board);
                    println!("Draw!");
                    println!("Press any key to continue...");
                    proceed::proceed();
                    break;
                }
                _ => {
                    step += 1;
                }
            },
            Err(error) => {
                println!("Error: \"{}\"", error);
                println!("Press any key to continue...");
                proceed::proceed();
                continue;
            }
        }
    }

    // loop {
    //     let current_player = if step_counter % 2 == 1 {
    //         CellState::Player1
    //     } else {
    //         CellState::Player2
    //     };
    //     std::process::Command::new("clear").status().unwrap();
    //     println!("{}", render_board(&board));
    //     println!(
    //         "Player '{}' your turn!",
    //         convert_cell_state_to_char(&current_player)
    //     );

    //     let p: Point = loop {
    //         let p = read_input_point();
    //         let input_result = point_is_suitable(&board, &p);
    //         match input_result.0 {
    //             true => break p,
    //             false => println!("{}", input_result.1),
    //         };
    //     };

    //     do_move(&mut board, current_player, &p);

    //     if check_player_win(&board, COUNT_IN_A_ROW_FOR_WIN, &p) {
    //         std::process::Command::new("clear").status().unwrap();
    //         println!("{}", render_board(&board));
    //         println!(
    //             "Congratulations! Player '{}' won!",
    //             convert_cell_state_to_char(&current_player)
    //         );
    //         break;
    //     }
    //     if step_counter == 9 {
    //         std::process::Command::new("clear").status().unwrap();
    //         println!("{}", render_board(&board));
    //         println!("Draw!");
    //         break;
    //     }
    //     step_counter += 1;
    // }
}

// #[allow(unused_macros)]
// macro_rules! read {
//     ($out:ident as $type:ty) => {
//         let mut inner = String::new();
//         std::io::stdin().read_line(&mut inner).expect("A String");
//         let $out = inner.trim().parse::<$type>().expect("Parsable");
//     };
// }

// fn read_input_point() -> Point {
//     println!("Choose you cell({}-{}):", 0, BOARD_SIDE_LEN - 1);
//     print!("x - ");
//     io::stdout().flush().unwrap();
//     read!(x as usize);
//     print!("y - ");
//     io::stdout().flush().unwrap();
//     read!(y as usize);
//     println!();
//     Point {
//         x,
//         y: (BOARD_SIDE_LEN - y - 1),
//     }
// }
