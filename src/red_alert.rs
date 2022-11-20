pub mod hittable;
pub mod cell;
pub mod board;
pub mod boat;
pub mod player;

use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;
use std::ops::RangeFrom;
use uuid::Uuid;

use crate::red_alert::board::Board;
use crate::red_alert::boat::Boat;
use crate::red_alert::player::Player;

/* -------------- Contants -------------- */

const VALID_X_LEN_RANGE : RangeFrom<u32> = 5..;
const VALID_Y_LEN_RANGE : RangeFrom<u32> = 5..;

/* -------------- Errors -------------- */

#[derive(Debug)]
pub struct InvalidGame {
    message : String,
}

impl fmt::Display for InvalidGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/* -------------- RedAlert -------------- */

#[derive(Debug)]
pub struct RedAlert {
    board_x_len : u32,
    board_y_len : u32,
    players : HashMap<String, Player>,
}

impl RedAlert {
    pub fn new(board_x_len : u32, board_y_len : u32) -> Result<RedAlert, InvalidGame> {
        if ! VALID_X_LEN_RANGE.contains(&board_x_len) {
            return Err( InvalidGame {message : format!("Expected x_len shall be in the range {}.. Provided: {}", VALID_X_LEN_RANGE.start, board_x_len)} );
        }
        if ! VALID_Y_LEN_RANGE.contains(&board_y_len) {
            return Err( InvalidGame {message : format!("Expected y_len shall be in the range {}.. Provided: {}", VALID_Y_LEN_RANGE.start, board_y_len)} );
        }

        Ok(Self {
            board_x_len : board_x_len,
            board_y_len : board_y_len,
            players : HashMap::default(),
        })
    }

    pub fn board_x_len(&self) -> u32 {
        self.board_x_len
    }

    pub fn board_y_len(&self) -> u32 {
        self.board_y_len
    }

    pub fn add_player(&mut self, name : &str) -> Result<(),()> {
        let name = String::from(name);
        if self.players.contains_key(&name) {
            return Err(());
        }

        let player = Player::new(
            &name,
            Board::new(
                self.board_x_len,
                self.board_y_len
            ),
            boats_vec_to_map(
                create_boats(
                    get_n_boat_pieces_per_player(
                        self.board_x_len,
                        self.board_y_len
                    )
                )
            ),
        );

        self.players.insert(player.name().clone(), player);

        Ok(())
    }

    pub fn place_boat(&mut self, player_name : &str, boat_id : &Uuid, x : u32, y : u32) -> Result<(), String> {
        let player_name = String::from(player_name);
        if let Some(player) = self.players.get_mut(&player_name) {
            return player.place_boat(boat_id, x, y);
        }
        Err(String::from("Unkown player"))
    }

    pub fn get_player_board(&self, player_name : &str) -> Option<&Board> {
        let player_name = String::from(player_name);
        if let Some(player) = self.players.get(&player_name) {
            return Some(player.board())
        }
        None
    }

    pub fn get_player_mut_board(&mut self, player_name : &str) -> Option<&mut Board> {
        let player_name = String::from(player_name);
        if let Some(player) = self.players.get_mut(&player_name) {
            return Some(player.mut_board())
        }
        None
    }

    pub fn get_player_boats(&self, player_name : &str) -> Option<&HashMap<Uuid, Boat>> {
        let player_name = String::from(player_name);
        if let Some(player) = self.players.get(&player_name) {
            return Some(player.boats())
        }
        None
    }

    pub fn get_winner(&self) -> Option< &Player > {
        let mut result : Option< &Player > = None;
        for (_, player) in &self.players {
            if player.is_alive() {
                if result.is_some() {
                    return None;
                }
                result = Some(player);
            }
        }
        result
    }
}

fn boats_vec_to_map(mut boats : Vec<Boat>) -> HashMap<Uuid, Boat> {
    let mut map : HashMap<Uuid, Boat> = HashMap::new();
    while ! boats.is_empty() {
        let boat = boats.pop().unwrap();
        map.insert(boat.id(), boat);
    }
    map
}

fn get_n_boat_pieces_per_player(board_x_len : u32, board_y_len : u32) -> u32 {
    (board_x_len * board_y_len) / 5
}

fn create_boats(n_pieces : u32) -> Vec<Boat> {
    // Possible boats are :
    // - 0,0 --> 0 piece --> used to make the index match the boat size
    // - 1,1 --> 1 piece
    // - 2,1 --> 2 pieces
    // - 1,3 --> 3 pieces
    // - 4,1 --> 4 pieces
    // - 1,5 --> 5 pieces
    // A complete set of boats has 0+1+2+3+4+5 = 15 pieces
    let complete_set : Vec<Boat> = vec![
        Boat::new(0, 0),
        Boat::new(1, 1),
        Boat::new(2, 1),
        Boat::new(1, 3),
        Boat::new(4, 1),
        Boat::new(1, 5),
    ];
    let complete_set_size : u32 = 15;

    let mut boats : Vec<Boat> = vec![];

    // First, let's give as much completes sets as possible
    for _ in 0..(n_pieces / complete_set_size) {
        boats.append(&mut complete_set[1..].to_vec());
    }

    // Remaining pieces
    let n_pieces = n_pieces % complete_set_size;

    match n_pieces {
        0 => (), // do nothing
        1 => boats.push(complete_set[1].clone()),
        2 => boats.push(complete_set[2].clone()),
        3 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[2].clone());
        }
        4 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[3].clone());
        }
        5 => {
            boats.push(complete_set[2].clone());
            boats.push(complete_set[3].clone());
        }
        6 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[2].clone());
            boats.push(complete_set[3].clone());
        }
        7 => {
            boats.push(complete_set[3].clone());
            boats.push(complete_set[4].clone());
        }
        8 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[3].clone());
            boats.push(complete_set[4].clone());
        }
        9 => {
            boats.push(complete_set[2].clone());
            boats.push(complete_set[3].clone());
            boats.push(complete_set[4].clone());
        }
        10 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[2].clone());
            boats.push(complete_set[3].clone());
            boats.push(complete_set[4].clone());
        }
        11 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[2].clone());
            boats.push(complete_set[3].clone());
            boats.push(complete_set[5].clone());
        }
        12 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[2].clone());
            boats.push(complete_set[4].clone());
            boats.push(complete_set[5].clone());
        }
        13 => {
            boats.push(complete_set[1].clone());
            boats.push(complete_set[3].clone());
            boats.push(complete_set[4].clone());
            boats.push(complete_set[5].clone());
        }
        14 => {
            boats.push(complete_set[2].clone());
            boats.push(complete_set[3].clone());
            boats.push(complete_set[4].clone());
            boats.push(complete_set[5].clone());
        }
        _ => (), // shall not happen
    }

    boats
}
