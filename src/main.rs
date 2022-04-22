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
}