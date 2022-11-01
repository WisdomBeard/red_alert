use std::io;

use crate::red_alert::RedAlert;
use crate::red_alert::board::Board;
use crate::red_alert::boat::Boat;

pub mod red_alert;

fn main() {
    println!("!!! RED ALERT !!!");

    let board_x_len = 10u32;
    let board_y_len = 10u32;

    let mut game = RedAlert::new(board_x_len, board_y_len).unwrap();

    let izuku   = game.add_player("Izuku");
    let katsuki = game.add_player("Katsuki");

    dbg!(game.get_player_board(izuku));
    dbg!(game.get_player_boats(katsuki));
    // dbg!(game);

    let (x, y) = user_get_coordinates(board_x_len, board_y_len);
    let &boat_id = game.get_player_boats(izuku).unwrap().keys().last().unwrap();
    game.place_boat(izuku, boat_id, x, y).unwrap();
    dbg!(game.get_player_boats(izuku));

    // board.hit(boat.x(), boat.y());

    // dbg!(board);
    // dbg!(boat);
}

fn user_get_coordinates(board_x_len : u32, board_y_len : u32) -> (u32, u32) {
    let mut x : u32 = 0;
    let mut y : u32 = 0;
    
    for (pos, pos_name, max_pos) in [
        (&mut x, "X", (board_x_len - 1)),
        (&mut y, "Y", (board_y_len - 1)),
    ] {
        println!("Please, provide a {pos_name} position in [0, {}]:", max_pos);
        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            *pos = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("  (provide a valid u32 value)");
                    continue;
                }
            };

            if *pos > max_pos {
                println!("  (provide a value < {max_pos})");
                continue;
            }

            break;
        }
    }

    (x, y)
}
