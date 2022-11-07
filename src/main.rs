use std::io;

use crate::red_alert::RedAlert;
// use crate::red_alert::board::Board;
// use crate::red_alert::boat::Boat;
use uuid::Uuid;

pub mod red_alert;

fn main() {
    println!("!!! RED ALERT !!!");

    let (board_x_len, board_y_len) = user_get_board_size();

    let mut game = RedAlert::new(board_x_len, board_y_len).unwrap();

    let player_names = user_create_players();
    for player_name in player_names.iter() {
        game.add_player(&player_name).unwrap();
    }

    for player_name in player_names.iter() {
        let player_boats = game.get_player_boats(&player_name).unwrap();

        let mut boat_ids : Vec<(Uuid,String)> = vec![];
        let mut horizontal_boat_strs : Vec<(u32,String)> = vec![];
        let mut vertical_boat_strs : Vec<(u32,String)> = vec![];
        for boat in player_boats.values() {
            boat_ids.push((boat.id().clone(), format!("{}", boat)));
            if boat.x_len() > 1 {
                horizontal_boat_strs.push((boat.x_len(), format!("{}", boat)))
            } else {
                vertical_boat_strs.push((boat.y_len(), format!("{}", boat)))
            }
        }

        // Make space

        println!("{}", "\n".repeat(50));

        // Show all boats

        horizontal_boat_strs.sort_unstable_by(|a,b|{
            a.0.cmp(&b.0)
        });
        vertical_boat_strs.sort_unstable_by(|a,b|{
            a.0.cmp(&b.0)
        });

        println!("Your fleet:");
        for (_, boat_str) in &horizontal_boat_strs {
            println!("{}", &boat_str);
        }

        for index in 0..5 {
            let mut line = String::new();
            line.reserve(vertical_boat_strs.len() * 2);
            
            for (boat_y_len, boat_str) in &vertical_boat_strs {
                line.push_str(format!(" {}", boat_str.chars().nth(index).unwrap_or(' ')).as_str());
            }
            if line.is_empty() {
                break;
            }

            println!("{}", &line);
        }

        // Place boats

        for (boat_id, boat_str) in &boat_ids {
            // Show board and the boat to place
            println!("\nYour map:\n{}\nPlease, {}, place the following boat:\n{}",
                game.get_player_board(&player_name).unwrap(),
                player_name,
                &boat_str
            );
            loop {
                let (x, y) = user_get_coordinates(board_x_len, board_y_len);
                match game.place_boat(&player_name, &boat_id, x, y) {
                    Ok(_) => break,
                    Err(message) => println!("{}", message),
                }
            }
        }
    }

    /* PSEUDO CODE
        foreach user
            print his board
            show other user boards on demand
            hit cell
            show impacted board
            check if end of game
    */

    // let izuku   = String::from("Izuku");
    // let katsuki = String::from("Katsuki");
    // game.add_player(&izuku);
    // game.add_player(&katsuki);

    // dbg!(game.get_player_board(&izuku));
    // dbg!(game.get_player_boats(&katsuki));
    // dbg!(game);
/*
    let (x, y) = user_get_coordinates(board_x_len, board_y_len);
    let &boat_id = game.get_player_boats(&izuku).unwrap().keys().last().unwrap();
    game.place_boat(&izuku, boat_id, x, y).unwrap();
    dbg!(game.get_player_boats(&izuku));
*/
    // board.hit(boat.x(), boat.y());

    // dbg!(board);
    // dbg!(boat);
}

fn user_get_board_size() -> (u32, u32) {
    let mut x_len : u32 = 0;
    let mut y_len : u32 = 0;
    
    for (size, pos_name, min_size) in [
        (&mut x_len, "X", 5),
        (&mut y_len, "Y", 5),
    ] {
        println!("Please, provide a board {pos_name} size in {}.. :", min_size);
        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            *size = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("  (provide a valid u32 value)");
                    continue;
                }
            };

            if *size < min_size {
                println!("  (provide a value >= {min_size})");
                continue;
            }

            break;
        }
    }

    (x_len, y_len)
}

fn user_create_players() -> Vec<String> {
    let min_n_players = 2usize;
    let mut players : Vec<String> = vec![];
    
    while players.len() < min_n_players {
        user_new_player(&mut players);
    }

    while user_wants_new_player() {
        user_new_player(&mut players);
    }

    players
}

fn user_wants_new_player() -> bool {
    println!("Do you want to add a new player? (y/n)");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "y" => return true,
            "n" => return false,
            _ => {
                println!("  (y/n)");
                continue;
            }
        }
    }
}

fn user_new_player(players : &mut Vec<String>) {
    println!("Please, provide a unique player name:");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim().to_string();

        if input.is_empty() {
            println!("  (provide a non-empty name)");
            continue;
        }

        if players.contains(&input) {
            println!("  (provide a unique name. Not in: {})", players.join(", "));
            continue;
        }

        players.push(input);

        break;
    }
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
