use super::hittable::Hittable;
use super::boat::BoatPiece;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Cell {
    // x : u32,
    // y : u32,
    is_hit : bool,
    boat_piece : Option<Rc<RefCell<BoatPiece>>>,
}

impl Cell {
    pub fn new(/*x : u32, y : u32*/) -> Self {
        Self {
            // x : x,
            // y : y,
            is_hit : false,
            boat_piece : Option::None,
        }
    }

    pub fn get_boat_piece(&self) -> Option<Rc<RefCell<BoatPiece>>> {
        self.boat_piece.clone()
    }

    pub fn set_boat_piece(&mut self, boat_piece: Rc<RefCell<BoatPiece>>) -> () {
        self.boat_piece = Option::Some(boat_piece);
    }

    pub fn to_string(&self, show_boats : bool) -> String {
        match (&self.is_hit, show_boats, &self.boat_piece) {
            (false, true, Some(_)) => format!("{}", "⚓"),
            (true, _, Some(_)) => format!("{}", "🔥"),
            (true, _, None) => format!("{}", "🌊"),
            (_, _, _) => format!("{}", "🟦"),
        }
    }
}

impl Hittable for Cell {
    fn hit(&mut self) -> () {
        self.is_hit = true;
        if let Some(boat_piece_rc) = &self.boat_piece {
            boat_piece_rc.borrow_mut().hit();
        }
    }

    fn repair(&mut self) -> () {
        self.is_hit = false;
    }

    fn is_hit(&self) -> bool {
        self.is_hit
    }
}
