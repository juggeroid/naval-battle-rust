use itertools::Itertools;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use staticvec::StaticVec;

const FIELD_SIZE: isize = 10;
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

impl std::fmt::Display for Field {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for (index, element) in self.field.iter().enumerate() {
            #[rustfmt::skip]
            let char_repr = match element {
                CellType::EMPTY       => '.',
                CellType::UNAVAILABLE => 'o',
                CellType::OCCUPIED    => 'X',
            };
            if index % FIELD_SIZE as usize == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", char_repr)?;
        })
    }
}

#[inline(always)]
fn is_valid_formation(
    field: &Field,
    mut x: isize,
    mut y: isize,
    dx: isize,
    dy: isize,
    ship_size: usize,
) -> bool {
    // I. Construct a bounding box for the placed ship.
    let bounds = 0..FIELD_SIZE;
    for ship_size in 0..ship_size {
        let x = x + (dx * ship_size as isize);
        let y = y + (dy * ship_size as isize);
        // Move in every box direction.
        for direction in DIRECTIONS.iter() {
            // Indices cannot be negative or >= FIELD_SIZE.
            if !bounds.contains(&(x + direction.0)) || !bounds.contains(&(y + direction.1)) {
                continue;
            }
            let bounding_box_cell = unsafe {
                field
                    .field
                    .get_unchecked(((x + direction.0) + ((y + direction.1) * FIELD_SIZE)) as usize)
            };
            // If there's a ship within a bounding box, halt the loop -- we cannot place the ship here.
            if bounding_box_cell == &CellType::OCCUPIED {
                return false;
            }
        }
    }
    // II. Check whether the cells that are being used to place the ship onto are occupied.
    for _ in 0..ship_size {
        if !bounds.contains(&x) || !bounds.contains(&y) {
            return false;
        }
        let current_cell = unsafe { field.field.get_unchecked((y * FIELD_SIZE + x) as usize) };
        if let CellType::OCCUPIED | CellType::UNAVAILABLE = current_cell {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

#[inline(always)]
fn get_available_cells(
    field: &Field,
    dx: isize,
    dy: isize,
    ship_size: usize,
    buffer: &mut StaticVec<(isize, isize), SQUARED_SIZE>,
) {
    buffer.clear();
    (0..FIELD_SIZE)
        .cartesian_product(0..FIELD_SIZE)
        .filter(|(x, y)| is_valid_formation(&field, *x, *y, dx, dy, ship_size))
        .for_each(|p| buffer.push(p));
}

#[inline(always)]
fn emplace_ships(
    field: &mut Field,
    ship_size: usize,
    rng: &mut SmallRng,
    cell_coordinates: &mut StaticVec<(isize, isize), SQUARED_SIZE>,
) {
    // Flip a coin to determine an alignment (horizontal / vertical).
    let (dx, dy) = if rng.gen() { (1, 0) } else { (0, 1) };
    // Get the vector of appropriate cells.
    get_available_cells(&field, dx, dy, ship_size, cell_coordinates);
    let (mut x, mut y) = cell_coordinates
        .choose(rng)
        .unwrap_or_else(|| unsafe { std::hint::unreachable_unchecked() });
    // Place a ship!
    for _ in 0..ship_size {
        unsafe {
            *field.field.get_unchecked_mut((x + y * FIELD_SIZE) as usize) = CellType::OCCUPIED
        };
        x += dx;
        y += dy;
    }
}

impl Field {
    #[inline(always)]
    fn generate() -> Self {
        /* Generating the field. */
        let mut f = Field { field: [CellType::EMPTY; (FIELD_SIZE * FIELD_SIZE) as usize] };
        let mut rng: SmallRng = SmallRng::from_entropy();
        let mut buffer = StaticVec::<(isize, isize), SQUARED_SIZE>::new();
        for ship_size in [4, 3, 3, 2, 2, 2, 1, 1, 1, 1].iter() {
            emplace_ships(&mut f, *ship_size, &mut rng, &mut buffer);
        }
        f
    }
}
fn main() {
    let field = Field::generate();
}
