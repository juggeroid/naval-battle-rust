

use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, Instant};

const FIELD_SIZE: isize = 10;
#[rustfmt::skip]
const DIRECTIONS: [(isize, isize); 9 as usize] = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0), (-1, 1), (1, -1), (-1, -1), (1, 1)];

#[derive(Clone, PartialEq, Copy)]
enum CellType {
    EMPTY,
    UNAVAILABLE,
    OCCUPIED,
}

struct Field {
    field: [CellType; 100],
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for (index, element) in self.field.iter().enumerate() {
            #[rustfmt::skip]
            let char_repr = match element {
                CellType::EMPTY       => ' ',
                CellType::UNAVAILABLE => '.',
                CellType::OCCUPIED    => 'X',
            };
            if index % FIELD_SIZE as usize == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", char_repr)?;
        })
    }
}

fn is_valid_formation(
    field: &Field,
    mut x: isize,
    mut y: isize,
    dx: isize,
    dy: isize,
    ship_size: usize,
) -> bool {
    /* I. Construct a bounding box for the placed ship. */
    let bounds = 0..FIELD_SIZE;
    for ship_size in 0..ship_size {
        let x = x + (dx * ship_size as isize);
        let y = y + (dy * ship_size as isize);
        for direction in DIRECTIONS.iter() {
            // Indices cannot be negative.
            if !bounds.contains(&(x + direction.0)) || !bounds.contains(&(y + direction.1)) {
                continue;
            }
            let bounding_box_cell =
                field.field[((x + direction.0) + ((y + direction.1) * FIELD_SIZE)) as usize];
            if bounding_box_cell == CellType::OCCUPIED {
                return false;
            }
        }
    }
    /* II. Check whether the cells that are being used to place the ship are occupied. */
    for _ in 0..ship_size {
        if !bounds.contains(&x) || !bounds.contains(&y) {
            return false;
        }
        let current_cell = field.field[(y * FIELD_SIZE + x) as usize];
        if let CellType::OCCUPIED | CellType::UNAVAILABLE = current_cell {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn get_available_cells(
    field: &Field,
    dx: isize,
    dy: isize,
    ship_size: usize,
) -> Vec<(isize, isize)> {
    (0..FIELD_SIZE)
        .cartesian_product(0..FIELD_SIZE)
        .filter(|(x, y)| is_valid_formation(&field, *x, *y, dx, dy, ship_size))
        .collect()
}

fn emplace_ships(field: &mut Field, ship_size: usize, rng: &mut ThreadRng) {
    let (dx, dy) = if rng.gen() { (1, 0) } else { (0, 1) };
    let cell_coordinates = get_available_cells(&field, dx, dy, ship_size);
    let chosen_cell = cell_coordinates[rng.gen_range(0, cell_coordinates.len())];
    let (mut x, mut y) = (chosen_cell.0, chosen_cell.1);
    for _ in 0..ship_size {
        field.field[(x + y * FIELD_SIZE) as usize] = CellType::OCCUPIED;
        x += dx;
        y += dy;
    }
}

impl Field {
    fn generate() -> Self {
        /* Generating the field. */
        let mut f = Field { field: [CellType::EMPTY; 100] };
        let mut rng: ThreadRng = rand::thread_rng();
        for ship_size in (1..=4).rev().flat_map(|c| std::iter::repeat(c).take(5 - c)) {
            emplace_ships(&mut f, ship_size, &mut rng);
        }
        f
    }
}
fn main() {
    let now = Instant::now();
    let field = Field::generate();
    let new = Instant::now();
    println!("{:?}", new.duration_since(now));
}
