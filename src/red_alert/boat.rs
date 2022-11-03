use super::hittable::Hittable;

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use uuid::Uuid;

/* -------------- BoatPiece -------------- */

#[derive(Debug)]
pub struct BoatPiece {
    x : u32,
    y : u32,
    is_hit : bool,
}

impl BoatPiece {
    fn new(x : u32, y : u32) -> Self {
        Self {
            x : x,
            y : y,
            is_hit : false,
        }
    }

    fn x(&self) -> u32 {
        self.x
    }

    fn y(&self) -> u32 {
        self.y
    }
}

impl Hittable for BoatPiece {
    fn hit(&mut self) -> () {
        self.is_hit = true;
    }

    fn repair(&mut self) -> () {
        self.is_hit = false;
    }

    fn is_hit(&self) -> bool {
        self.is_hit
    }
}

/* -------------- Boat -------------- */

#[derive(Debug)]
pub struct Boat {
    id : Uuid,
    x : u32,
    y : u32,
    x_len : u32,
    y_len : u32,
    pieces : Vec<Rc<RefCell<BoatPiece>>>,
    is_placed : bool,
}

impl Boat {
    pub fn new(x_len : u32, y_len : u32) -> Self {
        let mut res = Self {
            id : Uuid::new_v4(),
            x : 0,
            y : 0,
            x_len : x_len,
            y_len : y_len,
            pieces : Vec::new(),
            is_placed : false,
        };

        for piece_y in 0..y_len {
            for piece_x in 0..x_len {
                res.pieces.push(Rc::new(RefCell::new(BoatPiece::new(piece_x, piece_y))));
            }
        }

        res
    }

    pub fn target(&self, x : u32, y : u32) -> bool {
        x >= self.x && x < (self.x + self.x_len) && y >= self.y && y < (self.y + self.y_len)
    }

    pub fn hit(&mut self, x : u32, y : u32) -> bool {
        if let Ok(index) = self.coordinates_to_index(x, y) {
            self.pieces[index].borrow_mut().hit();
            return true;
        } else {
            return false;
        }
    }

    pub fn remaining_intact_pieces(&self) -> u32 {
        let mut remaining : u32 = 0;
        for piece in &self.pieces {
            if piece.borrow().is_hit() {
                remaining += 1;
            }
        }
        remaining
    }

    pub fn get_boat_piece_rc(&self, x : u32, y : u32) -> Result<Rc<RefCell<BoatPiece>>, ()> {
        let index = self.coordinates_to_index(x, y)?;
        Ok( Rc::clone(&(self.pieces[index])) )
    }

    pub fn get_n_pieces(&self) -> u32 {
        self.x_len * self.y_len
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn x(&self) -> u32 {
        self.x
    }
    
    pub fn set_x(&mut self, new_x : u32) {
        self.x = new_x;
    }

    
    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_y(&mut self, new_y : u32) {
        self.y = new_y;
    }


    pub fn x_len(&self) -> u32 {
        self.x_len
    }

    pub fn y_len(&self) -> u32 {
        self.y_len
    }

    pub fn place(&mut self) {
        self.is_placed = true;
    }

    fn coordinates_to_index(&self, x : u32, y : u32) -> Result<usize, ()> {
        if ! self.target(x, y) {
            return Err(());
        }
        Ok( ((y - self.y) * self.x_len + (x - self.x)) as usize )
    }
}

impl Clone for Boat {
    fn clone(&self) -> Self {
        Self::new(self.x_len, self.y_len)
    }

    fn clone_from(&mut self, source: &Self) {
        self.x_len = source.x_len;
        self.y_len = source.y_len;
    }
}

impl fmt::Display for Boat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut boat_str = "X".repeat(self.x_len as usize);
        boat_str.push('\n');
        write!(f, "{}", boat_str.repeat(self.y_len as usize))
    }
}
