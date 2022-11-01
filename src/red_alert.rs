pub mod hidable;
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
    players : HashMap<Uuid, Player>,
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

    pub fn add_player(&mut self, name : &str) -> Uuid {
        let player = Player::new(
            name,
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
        let player_id = player.id();
        self.players.insert(player.id(), player);
        player_id
    }

    pub fn place_boat(&mut self, player_id : Uuid, boat_id : Uuid, x : u32, y : u32) -> Result<(), String> {
        if let Some(player) = self.players.get_mut(&player_id) {
            return player.place_boat(boat_id, x, y);
        }
        Err(String::from("Unkown player"))
    }

    pub fn get_player_board(&self, player_id : Uuid) -> Option<&Board> {
        if let Some(player) = self.players.get(&player_id) {
            return Some(player.board())
        }
        None
    }

    pub fn get_player_boats(&self, player_id : Uuid) -> Option<&HashMap<Uuid, Boat>> {
        if let Some(player) = self.players.get(&player_id) {
            return Some(player.boats())
        }
        None
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

/*
fn main() {

    let mut board = Board::new(10, 10);
    let mut boat   = Boat::new(1, 3);

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

    place_boat(board, boat, x, y);
}
*/