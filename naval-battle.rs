

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use itertools::Itertools;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use staticvec::StaticVec;
use std::fmt;
use std::ops::{Index, IndexMut};

const FIELD_SIZE: usize = 10;
const SQUARED_SIZE: usize = (FIELD_SIZE * FIELD_SIZE) as usize;

#[rustfmt::skip]
const DIRECTIONS: [(isize, isize); 9 as usize] = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0), (-1, 1), (1, -1), (-1, -1), (1, 1)];

#[derive(Clone, PartialEq, Copy)]
enum CellType {
    EMPTY,
    UNAVAILABLE,
    OCCUPIED,
}

struct Field {
    field: [CellType; (FIELD_SIZE * FIELD_SIZE) as usize],
}

impl Index<(usize, usize)> for Field {
    type Output = CellType;
    fn index(&self, (x, y): (usize, usize)) -> &CellType {
        &self.field[x + y * FIELD_SIZE]
    }
}

impl IndexMut<(usize, usize)> for Field {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut CellType {
        &mut self.field[x + y * FIELD_SIZE]
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EMPTY => '.',
                Self::UNAVAILABLE => 'o',
                Self::OCCUPIED => 'X',
            }
        )
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, element) in self.field.iter().enumerate() {
            // Start of the new line.
            if index % FIELD_SIZE as usize == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct ShipShape {
    dx:   usize,
    dy:   usize,
    size: usize,
}

#[derive(Copy, Clone)]
struct Ship {
    x:     usize,
    y:     usize,
    shape: ShipShape,
}

#[allow(clippy::copy_iterator)]
impl Iterator for Ship {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.shape.size > 0 {
            let result = (self.x, self.y);
            self.x += self.shape.dx;
            self.y += self.shape.dy;
            self.shape.size -= 1;
            Some(result)
        } else {
            None
        }
    }
}

impl Field {
    fn can_place_ship(&self, ship: Ship) -> bool {
        // I. Construct a bounding box for the placed ship.
        let bounds = 0..(FIELD_SIZE as isize);
        for (x, y) in ship {
            // Move in every box direction.
            for direction in &DIRECTIONS {
                // Indices cannot be negative or >= FIELD_SIZE.
                if !bounds.contains(&(x as isize + direction.0))
                    || !bounds.contains(&(y as isize + direction.1))
                {
                    continue;
                }
                let bounding_box_cell = self
                    [((x as isize + direction.0) as usize, (y as isize + direction.1) as usize)];
                // If there's a ship within a bounding box, halt the loop -- we cannot place the ship here.
                if bounding_box_cell == CellType::OCCUPIED {
                    return false;
                }
            }
        }
        // II. Check whether the cells that are being used to place the ship onto are occupied.
        let bounds = 0..FIELD_SIZE;
        for (x, y) in ship {
            if !bounds.contains(&x) || !bounds.contains(&y) {
                return false;
            }
            let current_cell = self[(x, y)];
            if let CellType::OCCUPIED | CellType::UNAVAILABLE = current_cell {
                return false;
            }
        }
        true
    }

    fn get_available_cells(&self, shape: ShipShape) -> Vec<(usize, usize)> {
        (0..FIELD_SIZE)
            .cartesian_product(0..FIELD_SIZE)
            .filter(|&(x, y)| self.can_place_ship(Ship { x, y, shape }))
            .collect()
    }

    fn emplace_ships(
        &mut self,
        size: usize,
        rng: &mut impl Rng,
        cell_coordinates: &mut StaticVec<(isize, isize), SQUARED_SIZE>,
    ) {
        // Flip a coin to determine an alignment (horizontal / vertical).
        let (dx, dy) = if rng.gen() { (1, 0) } else { (0, 1) };
        let shape = ShipShape { dx, dy, size };
        // Get the vector of appropriate cells.
        let cell_coordinates = self.get_available_cells(shape);
        let (x, y) = *cell_coordinates.choose(rng).unwrap();
        let ship = Ship { x, y, shape };
        // Place a ship!
        for (x, y) in ship {
            self[(x, y)] = CellType::OCCUPIED;
        }
    }
}

impl Field {
    fn generate() -> Self {
        /* Generating the field. */
        let mut result = Self { field: [CellType::EMPTY; FIELD_SIZE * FIELD_SIZE] };
        let mut rng: SmallRng = SmallRng::from_entropy();
        let mut buffer = StaticVec::<(isize, isize), SQUARED_SIZE>::new();
        for ship_size in &[4, 3, 3, 2, 2, 2, 1, 1, 1, 1] {
            result.emplace_ships(*ship_size, &mut rng, &mut buffer);
        }
        result
    }
}
fn main() {
    let now = std::time::Instant::now();
    let field = Field::generate();
    println!("{:?}", now.elapsed());
    println!("{}", field)
}

