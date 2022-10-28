use super::cell::Cell;
use super::boat::Boat;

// use std::rc::Rc;
// use std::cell::RefCell;

#[derive(Debug)]
pub struct Board {
    cells : Vec<Vec<Cell>>,
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
        self.cells.len().try_into().unwrap()
    }

    pub fn y_len(&self) -> u32 {
        match self.cells.get(0) {
            Some(row) => row.len().try_into().unwrap(),
            _ => 0,
        }
    }

    pub fn place_boat(&mut self, boat : &mut Boat) -> Result<(), ()> {
        let x1 = boat.x();
        let x2 = x1 + boat.x_len();
        let y1 = boat.y();
        let y2 = y1 + boat.y_len();
        for x in x1..x2 {
            for y in y1..y2 {
                self.cells[x as usize][y as usize].set_boat_piece(boat.get_boat_piece_rc(x, y)?);
            }
        }
        Ok(())
    }
}
