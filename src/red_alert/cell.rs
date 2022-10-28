use super::hittable::Hittable;
use super::hidable::Hidable;
use super::boat::BoatPiece;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Cell {
    // x : u32,
    // y : u32,
    is_hidden : bool,
    is_hit : bool,
    boat_piece : Option<Rc<RefCell<BoatPiece>>>,
}

impl Cell {
    pub fn new(/*x : u32, y : u32*/) -> Self {
        Self {
            // x : x,
            // y : y,
            is_hidden : true,
            is_hit : false,
            boat_piece : Option::None,
        }
    }

    pub fn set_boat_piece(&mut self, boat_piece: Rc<RefCell<BoatPiece>>) -> () {
        self.boat_piece = Option::Some(boat_piece);
    }
}

impl Hidable for Cell {
    fn hide(&mut self) -> () {
        self.is_hidden = true;
    }

    fn reveal(&mut self) -> () {
        self.is_hidden = false;
    }

    fn is_hidden(&self) -> bool {
        self.is_hidden
    }
}

impl Hittable for Cell {
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
