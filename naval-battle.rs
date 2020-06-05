// RUSTFLAGS="-Ctarget-cpu=native"
use rand::{Rng, SeedableRng};

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

fn show(field: &Field) {
    for row in &field.field {
        for element in row {
            let char_repr = match element {
                CellType::EMPTY => '◻',
                CellType::UNAVAILABLE => '▨',
                CellType::OCCUPIED => '◼',
            };
            print!("{} ", char_repr);
        }
        println!();
    }
}

fn main() {
    // let mut f = Field{field: vec![vec![CellType::EMPTY; 10]; 10] };
    // show(&mut f);
}
