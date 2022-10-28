use super::hittable::Hittable;

#[derive(Debug)]
struct BoatPiece {
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

#[derive(Debug)]
pub struct Boat {
    x : u32,
    y : u32,
    x_len : u32,
    y_len : u32,
    pieces : Vec<BoatPiece>,
}

impl Boat {
    pub fn new(x : u32, y : u32, x_len : u32, y_len : u32) -> Self {
        let mut res = Self {
            x : x,
            y : y,
            x_len : x_len,
            y_len : y_len,
            pieces : Vec::new(),
        };

        for piece_x in 0..x_len {
            for piece_y in 0..y_len {
                res.pieces.push(BoatPiece::new(piece_x, piece_y));
            }
        }

        res
    }

    pub fn target(&self, x : u32, y : u32) -> bool {
        x >= self.x && x < (self.x + self.x_len) && y >= self.y && y < (self.y + self.y_len)
    }

    pub fn hit(&mut self, x : u32, y : u32) -> bool {
        if self.target(x, y) {
            let index = self.coordinates_to_index(x, y).unwrap();
            self.pieces[index].hit();
            true
        } else {
            false
        }
    }

    pub fn remaining_intact_pieces(&self) -> u32 {
        let mut remaining : u32 = 0;
        for piece in &self.pieces {
            if piece.is_hit() {
                remaining += 1;
            }
        }
        remaining
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

    fn coordinates_to_index(&self, x : u32, y : u32) -> Result<usize, ()> {
        if self.target(x, y) {
            Ok( (((x - self.x) * self.x_len) + (y - self.y)) as usize )
        } else {
            Err(())
        }
    }
}
