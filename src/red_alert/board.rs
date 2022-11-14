use super::cell::Cell;
use super::boat::Boat;
use super::hittable::Hittable;

use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct Board {
    pub cells : Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width : u32, height : u32) -> Self {
        let mut res = Self {
            cells : Vec::new(),
        };

        for _/*cell_x*/ in 0..width {
            let mut local_vec : Vec<Cell> = Vec::new();
            for _/*cell_y*/ in 0..height {
                local_vec.push(Cell::new(/*cell_x, cell_y*/));
            }
            res.cells.push(local_vec);
        }
        res
    }

    pub fn x_len(&self) -> u32{
        self.cells.len() as u32
    }

    pub fn y_len(&self) -> u32 {
        match self.cells.get(0) {
            Some(row) => row.len() as u32,
            _ => 0,
        }
    }

    pub fn place_boat(&mut self, boat : &mut Boat) -> Result<(), ()> {
        let x1 = boat.x();
        let x2 = x1 + boat.x_len();
        let y1 = boat.y();
        let y2 = y1 + boat.y_len();

        if x2 > self.x_len() || y2 > self.y_len() {
            return Err(());
        }

        let x1_extended = if x1 > 0 {x1-1} else {x1};
        let x2_extended = if x2 < (self.x_len() - 1) {x2+1} else {x2};
        let y1_extended = if y1 > 0 {y1-1} else {y1};
        let y2_extended = if y2 < (self.y_len() - 1) {y2+1} else {y2};
        for x in x1_extended..x2_extended {
            for y in y1_extended..y2_extended {
                if self.cells[x as usize][y as usize].get_boat_piece().is_some() {
                    return Err(());
                }
            }
        }

        for x in x1..x2 {
            for y in y1..y2 {
                self.cells[x as usize][y as usize].set_boat_piece(boat.get_boat_piece_rc(x, y)?);
            }
        }
        boat.place();
        Ok(())
    }

    pub fn hit(&mut self, x : u32, y : u32) -> Result<bool,()> {
        if x < self.x_len() && y < self.y_len() {
            self.cells[x as usize][y as usize].hit();
            return Ok(self.cells[x as usize][y as usize].get_boat_piece().is_some());
        } else {
            return Err(());
        }
    }

    pub fn is_hit(&mut self, x : u32, y : u32) -> Result<bool,()> {
        if x < self.x_len() && y < self.y_len() {
            return Ok(self.cells[x as usize][y as usize].get_boat_piece().is_some());
        } else {
            return Err(());
        }
    }

    pub fn to_string(&self, show_boats : bool) -> String {
        let mut board_str = String::new();
        board_str.reserve((2 * self.x_len() as usize + "\n││ 0".len()) * (self.y_len() as usize + "0──".len()));

        board_str.push(' ');
        for x in 0..self.x_len() {
            board_str.push_str(format!(" {}", x%10).as_str());
        }
        board_str.push_str(format!("\n┌{}┐", "─".repeat(2 * self.x_len() as usize)).as_str());

        for y in 0..self.y_len() {
            board_str.push_str("\n│");
            for x in 0..self.x_len() {
                board_str.push_str(self.cells[x as usize][y as usize].to_string(show_boats).as_str());
            }
            board_str.push_str(format!("│ {}", y%10).as_str());
        }
        
        board_str.push_str(format!("\n└{}┘", "─".repeat(2 * self.x_len() as usize)).as_str());

        board_str
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string(true))
    }
}
