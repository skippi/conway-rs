use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

struct Conway {
    grid: HashSet<(i32, i32)>,
}

impl Conway {
    pub fn new() -> Conway {
        Conway {
            grid: HashSet::new(),
        }
    }

    pub fn decay(mut self) -> Conway {
        self.grid = self
            .grid
            .iter()
            .filter(|&&pos| self.neighbors(pos).len() >= 2)
            .cloned()
            .collect();
        self
    }

    pub fn neighbors(&self, (row, col): (i32, i32)) -> Vec<(i32, i32)> {
        let relative_coords = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        relative_coords
            .iter()
            .map(|(x, y)| (row + x, col + y))
            .filter(|pos| self.grid.contains(pos))
            .collect()
    }

    #[cfg(test)]
    fn spawn(&mut self, pos: (i32, i32)) {
        self.grid.insert(pos);
    }

    #[cfg(test)]
    fn status(&self, pos: (i32, i32)) -> Cell {
        match self.grid.get(&pos) {
            Some(_) => Cell::Alive,
            None => Cell::Dead,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_kills_adjacent_unpopulated() {
        let mut conway = Conway::new();
        conway.spawn((0, 0));
        conway.spawn((0, 1));
        conway.spawn((0, 2));
        conway = conway.decay();
        assert_eq!(conway.status((0, 0)), Cell::Dead);
        assert_eq!(conway.status((0, 1)), Cell::Alive);
        assert_eq!(conway.status((0, 2)), Cell::Dead);
    }

    #[test]
    fn test_decay_kills_diagonal_unpopulated() {
        let mut conway = Conway::new();
        conway.spawn((0, 0));
        conway.spawn((1, 1));
        conway.spawn((2, 2));
        conway = conway.decay();
        assert_eq!(conway.status((0, 0)), Cell::Dead);
        assert_eq!(conway.status((1, 1)), Cell::Alive);
        assert_eq!(conway.status((2, 2)), Cell::Dead);
    }

    #[test]
    fn test_neighbors() {
        let mut conway = Conway::new();
        conway.spawn((0, 0));
        conway.spawn((0, 1));
        conway.spawn((1, 0));
        assert_eq!(conway.neighbors((0, 0)), vec![(0, 1), (1, 0)])
    }
}
