use std::io;

use crate::red_alert::board::Board;
use crate::red_alert::boat::Boat;

pub mod red_alert;

fn main() {
    println!("!!! RED ALERT !!!");

    let mut board = Board::new(10, 10);
    let mut boat   = Boat::new(0, 0, 1, 3);

    user_place_boat(&mut board, &mut boat);

    board.hit(boat.x(), boat.y());

    dbg!(board);
    dbg!(boat);
}

fn user_place_boat(board : &mut Board, boat : &mut Boat) {
    let mut x : u32 = 0;
    let mut y : u32 = 0;
    
    for (pos, pos_name, max_pos) in [
        (&mut x, "X", (board.x_len() - boat.x_len())),
        (&mut y, "Y", (board.y_len() - boat.y_len())),
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

    boat.set_x(x);
    boat.set_y(y);
    board.place_boat(boat).unwrap_or_default();
}
