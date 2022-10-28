use super::hittable::Hittable;
use super::hidable::Hidable;
// use super::boat::Boat;

#[derive(Debug)]
pub struct Cell {
    x : u32,
    y : u32,
    is_hidden : bool,
    is_hit : bool,
    // boat : Option<&'a mut Boat>,
}

impl Cell {
    pub fn new(x : u32, y : u32) -> Self {
        Self {
            x : x,
            y : y,
            is_hidden : true,
            is_hit : false,
            // boat : Option::None,
        }
    }

    // pub fn set_boat(&mut self, boat: &'a mut Boat) -> () {
    //     self.boat = Option::Some(boat);
    // }
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
