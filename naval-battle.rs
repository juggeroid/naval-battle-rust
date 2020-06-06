// RUSTFLAGS="-Ctarget-cpu=native"
use rand::rngs::ThreadRng;
use rand::{Rng, SeedableRng};

const MINIMUM_FIELD_SIZE: usize = 0;
const MAXIMUM_FIELD_SIZE: usize = 10;

#[derive(Clone)]
enum CellType {
    EMPTY,
    UNAVAILABLE,
    OCCUPIED,
}

struct Cell {
    coordinates: (isize, isize),
    cell_type:   CellType,
}

struct Ship {
    coordinates: Vec<Cell>, /* e.g. (1, 2), (1, 3), ... */
}

struct Field {
    field: Vec<Vec<CellType>>,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for row in &self.field {
            for element in row {
                let char_repr = match element {
                    CellType::EMPTY => '◻',
                    CellType::UNAVAILABLE => '▨',
                    CellType::OCCUPIED => '◼',
                };
                write!(f, "{}", char_repr)?;
            }
            writeln!(f)?;
        })
    }
}

impl Field {
    fn generate() -> Self {
        let mut f =
            Field { field: vec![vec![CellType::EMPTY; MAXIMUM_FIELD_SIZE]; MAXIMUM_FIELD_SIZE] };
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let coin_tossed: bool = rng.gen_bool(1.0 / 2.0);
        let mut generate_coordinate = || rng.gen_range(MINIMUM_FIELD_SIZE, MAXIMUM_FIELD_SIZE);
        for ship_size in (1..=4).flat_map(|c| std::iter::repeat(c).take(5 - c)) {
            let (mut x, mut y) = (generate_coordinate(), generate_coordinate());
            let (mut dx, mut dy) = (0, 0);
            if coin_tossed {
                dx = 1
            } else {
                dy = 1
            };
            for placement_iteration in 0..=ship_size {
                x += dx;
                y += dy;
                f.field[x + dx][y + dy] = CellType::OCCUPIED;
            }
        }
        f
    }
}

fn main() {
    let mut field = Field::generate();
    println!("{}", field)
}
