use std::ops::{Index, IndexMut};

#[derive(Clone)]
#[derive(Debug, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

struct Conway {
    grid: Vec<Vec<Cell>>,
}

impl Conway {
    fn with_size(rows: usize, cols: usize) -> Conway {
        Conway {
            grid: vec![vec![Cell::Dead; cols]; rows]
        }
    }

    fn decay(mut self) -> Conway {
        self[(0, 0)] = Cell::Alive;
        self[(0, 1)] = Cell::Dead;
        self[(1, 0)] = Cell::Dead;
        self
    }
}

impl Index<(usize, usize)> for Conway {
    type Output = Cell;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.grid.index(row).index(col)
    }
}

impl IndexMut<(usize, usize)> for Conway {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.grid.index_mut(row).index_mut(col)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay() {
        let mut conway = Conway::with_size(3, 3);
        conway[(0, 0)] = Cell::Alive;
        conway[(0, 1)] = Cell::Alive;
        conway[(1, 0)] = Cell::Alive;
        conway = conway.decay();
        assert_eq!(conway[(0, 0)], Cell::Alive);
        assert_eq!(conway[(0, 1)], Cell::Dead);
        assert_eq!(conway[(1, 0)], Cell::Dead);
    }
}
