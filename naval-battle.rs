

use itertools::Itertools;
use rand::distributions::{Distribution, Standard, Uniform};
use rand::rngs::{StdRng, ThreadRng};
use rand::thread_rng;
use rand::{Rng, SeedableRng};

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
    field: Vec<Vec<CellType>>,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for row in &self.field {
            for element in row {
                #[rustfmt::skip]
                let char_repr = match element {
                    CellType::EMPTY       => ' ', 
                    CellType::UNAVAILABLE => '.',
                    CellType::OCCUPIED    => 'X',
                };
                write!(f, "{}", char_repr)?;
            }
            writeln!(f)?;
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
            let out_of_bounds: bool =
                !bounds.contains(&(x + direction.0)) || !bounds.contains(&(y + direction.1));
            // Indices cannot be negative.
            if out_of_bounds {
                continue;
            }
            let bounding_box_cell =
                field.field[(x + direction.0) as usize][(y + direction.1) as usize];
            if bounding_box_cell == CellType::OCCUPIED {
                return false;
            }
        }
    }
    /* II. Check whether the cells that are being used to place the ship are occupied. */
    for _ in 0..ship_size {
        let out_of_bounds = !bounds.contains(&x) || !bounds.contains(&y);
        if out_of_bounds {
            return false;
        }
        let current_cell = field.field[x as usize][y as usize];
        if let CellType::OCCUPIED | CellType::UNAVAILABLE = current_cell {
            return false;
        }
        x += dx;
        y += dy;
    }
    return true;
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
    let coin_toss: bool = rng.gen_bool(0.5);
    // Decide the ship's alignment (horizontal / vertical, with equal probability).
    let (mut dx, mut dy): (isize, isize) = (0, 0);
    if coin_toss {
        dx = 1;
    } else {
        dy = 1;
    }
    let cell_coordinates = get_available_cells(&field, dx, dy, ship_size);
    let chosen_cell = cell_coordinates[rng.gen_range(0, cell_coordinates.len())];
    let mut x = chosen_cell.0;
    let mut y = chosen_cell.1;
    for _ in 0..ship_size {
        field.field[x as usize][y as usize] = CellType::OCCUPIED;
        x += dx;
        y += dy;
    }
}

impl Field {
    fn generate() -> Self {
        /* Generating the field. */
        let mut f =
            Field { field: vec![vec![CellType::EMPTY; FIELD_SIZE as usize]; FIELD_SIZE as usize] };
        let mut rng: ThreadRng = rand::thread_rng();
        for ship_size in (1..=4).rev().flat_map(|c| std::iter::repeat(c).take(5 - c)) {
            emplace_ships(&mut f, ship_size, &mut rng);
        }
        println!("{}", f);
        todo!()
    }
}

fn main() {
    let field = Field::generate();
    println!("{}", field)
}
