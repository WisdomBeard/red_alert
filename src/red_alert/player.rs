use crate::red_alert::board::Board;
use crate::red_alert::boat::Boat;

use uuid::Uuid;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Player {
    name : String,
    board : Board,
    boats : HashMap<Uuid, Boat>,
}

impl Player {
    pub fn new(name : &str, board : Board, boats : HashMap<Uuid, Boat>) -> Self {
        Self {
            name  : name.to_string(),
            board : board,
            boats : boats,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn mut_board(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn boat(&self, boat_id : &Uuid) -> Option<&Boat> {
        self.boats.get(boat_id)
    }

    pub fn mut_boat(&mut self, boat_id : &Uuid) -> Option<&mut Boat> {
        self.boats.get_mut(boat_id)
    }

    pub fn boats(&self) -> &HashMap<Uuid, Boat> {
        &self.boats
    }

    pub fn mut_boats(&mut self) -> &mut HashMap<Uuid, Boat> {
        &mut self.boats
    }

    pub fn place_boat(&mut self, boat_id : Uuid, x : u32, y : u32) -> Result<(), String> {
        let boat = self.boats.get_mut(&boat_id).ok_or_else(||"Unknown boat".to_string())?;

        boat.set_x(x);
        boat.set_y(y);

        self.board.place_boat(boat)
            .or(Err(format!("Failed to place the boat on {}x{}", x, y)))?;

        Ok(())
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}