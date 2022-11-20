use std::io;

use crate::red_alert::RedAlert;
// use crate::red_alert::board::Board;
// use crate::red_alert::boat::Boat;
use uuid::Uuid;

pub mod red_alert;

fn main() {
    println!("!!! RED ALERT !!!");

    // Ask for the desired board size
    let (board_x_len, board_y_len) = user_get_board_size();

    // Create a game accordingly
    let mut game = RedAlert::new(board_x_len, board_y_len).unwrap();

    // Create players
    let player_names = user_create_players();
    for player_name in player_names.iter() {
        game.add_player(&player_name).unwrap();
    }

    // For each player : deploy its fleet
    for player_name in player_names.iter() {
        let player_boats = game.get_player_boats(&player_name).unwrap();

        let mut boat_ids : Vec<(Uuid,String)> = vec![];
        for boat in player_boats.values() {
            boat_ids.push((boat.id().clone(), format!("{}", boat)));
        }

        // Place boats
        
        for (boat_id, boat_str) in &boat_ids {
            // Make space
            clear();

            // Show all boats
            print_fleet(&game, player_name.as_str());

            // Show board
            println!("\nYour map:\n{}", game.get_player_board(&player_name).unwrap());

            // Show the boat to place
            println!("\nPlease, {}, place the following boat:\n{}",
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

        // Make space
        clear();

        // Show board
        println!("\n{}", game.get_player_board(&player_name).unwrap());

        user_confirm(player_name, "your fleet is now deployed !");
    }

    // For each player : ask for action
    'main_loop: loop {
        for player_name in player_names.iter() {
            clear();
            user_confirm(player_name, "are you ready to take actions ?!");

            clear();
            // Show opponents
            for opponent_name in player_names.iter() {
                if opponent_name == player_name {
                    continue
                }

                let board = &*(game.get_player_board(&opponent_name).unwrap());
                println!("\nOur knowledge of {}'s map:\n{}", &opponent_name, board.to_string(false));
            }

            // Show board
            println!("\nYour map:\n{}", game.get_player_board(&player_name).unwrap());
            
            user_hit(&mut game, &player_name);
            
            if game.get_winner().is_some() {
                user_confirm(player_name, "You did it, they all eat fish at the bottom of the ocean!");
                break 'main_loop;
            } else {
                user_confirm("", "");
            }
        }
    }
}

fn print_fleet(game : &RedAlert, player_name : &str) {
    let player_boats = game.get_player_boats(player_name).unwrap();

    let mut horizontal_boat_strs : Vec<(u32,String)> = vec![];
    let mut vertical_boat_strs : Vec<(u32,String)> = vec![];
    for boat in player_boats.values() {
        if boat.x_len() > 1 {
            horizontal_boat_strs.push((boat.x_len(), format!("{}", boat)))
        } else {
            vertical_boat_strs.push((boat.y_len(), format!("{}", boat)))
        }
    }

    horizontal_boat_strs.sort_unstable_by(|a,b|{
        b.0.cmp(&a.0)
    });
    vertical_boat_strs.sort_unstable_by(|a,b|{
        b.0.cmp(&a.0)
    });

    println!("Your fleet:");
    for (_, boat_str) in &horizontal_boat_strs {
        println!("{}\n", &boat_str);
    }

    for index in 0..5 {
        let mut line = String::new();
        line.reserve(vertical_boat_strs.len() * 2);

        for (boat_y_len, boat_str) in &vertical_boat_strs {
            if index < *boat_y_len as usize {
                line.push_str(format!("{}  ", boat_str.chars().nth(2 * index).unwrap()).as_str());
            } else {
                line.push_str("    ");
            }
        }
        if line.is_empty() {
            break;
        }
        
        println!("{}", &line);
    }
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

fn user_confirm(player_name : &str, message : &str) -> String {
    let sep : &str;
    if player_name.is_empty() || message.is_empty() {
        sep = "";
    } else {
        sep = ", ";
    }
    println!("\n{}{}{}", player_name, sep, message);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    input.trim().to_string()
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

fn user_hit(game : &mut RedAlert, player_name : &str) {
    let (board_x_len, board_y_len) = (game.board_x_len(), game.board_y_len());
    loop {
        let ennemy_name = user_confirm(&player_name, "Who is the ennemy?");
        if let Some(ennemy_board) = game.get_player_mut_board(&ennemy_name) {
            println!("Captain, we need coordinates to hit!");

            loop {
                let (hit_x, hit_y) = user_get_coordinates(board_x_len, board_y_len);

                let is_already_hit = ennemy_board.is_hit(hit_x, hit_y).unwrap();
                if ! is_already_hit {
                    if ennemy_board.hit(hit_x, hit_y).unwrap() {
                        println!("BOOM!");
                    } else {
                        println!("Shit... it's a miss");
                    }
                    break;
                } else {
                    println!("  Huh, this spot was already hit...");
                    continue;
                }
            }
            break;
        } else {
            println!("  Sorry, didn't get it.");
            continue;
        }
    }
}

fn clear() {
    println!("{}", "\n".repeat(100));
}