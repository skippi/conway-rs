use std::collections::HashSet;
use std::iter;

static RELATIVE_CORDS: &'static [(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Eq, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn evolve(&self, alive_neighbor_count: usize) -> Cell {
        match (self, alive_neighbor_count) {
            (Cell::Alive, 2) => Cell::Alive,
            (Cell::Alive, 3) => Cell::Alive,
            (Cell::Dead, 3) => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

pub struct Conway {
    grid: HashSet<(i32, i32)>,
}

impl Conway {
    fn new() -> Conway {
        Conway {
            grid: HashSet::new(),
        }
    }

    #[cfg(test)]
    fn with_cells(points: &[(i32, i32)]) -> Self {
        Self::with_iter(points.iter().cloned())
    }

    pub fn with_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = (i32, i32)>,
    {
        let mut conway = Conway::new();
        conway.grid.extend(iter);
        conway
    }

    fn count_alive_neighbors(&self, point: (i32, i32)) -> usize {
        self.get_neighbors(point)
            .filter(|p| self.grid.contains(p))
            .count()
    }

    pub fn get(&self, point: (i32, i32)) -> Cell {
        if self.grid.contains(&point) {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }

    pub fn next(&self) -> Self {
        let grid = self
            .grid
            .iter()
            .flat_map(|&p| self.get_neighborhood(p))
            .filter(|&p| {
                let alive_count = self.count_alive_neighbors(p);
                self.get(p).evolve(alive_count) == Cell::Alive
            })
            .collect();
        Conway { grid }
    }

    fn get_neighbors(&self, (row, col): (i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
        RELATIVE_CORDS.iter().map(move |(x, y)| (row + x, col + y))
    }

    fn get_neighborhood(&self, point: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.get_neighbors(point).chain(iter::once(point))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evolve_saves_cell_with_two_alive_neighbors() {
        assert_eq!(Cell::Alive.evolve(2), Cell::Alive);
    }

    #[test]
    fn evolve_saves_cell_with_three_alive_neighbors() {
        assert_eq!(Cell::Alive.evolve(3), Cell::Alive);
        assert_eq!(Cell::Dead.evolve(3), Cell::Alive);
        assert_eq!(Cell::Alive.evolve(4), Cell::Dead);
        assert_eq!(Cell::Alive.evolve(5), Cell::Dead);
    }

    #[test]
    fn evolve_revives_cell_with_three_alive_neighbors() {
        assert_eq!(Cell::Dead.evolve(3), Cell::Alive);
        assert_eq!(Cell::Alive.evolve(4), Cell::Dead);
        assert_eq!(Cell::Alive.evolve(5), Cell::Dead);
    }

    #[test]
    fn evolve_kills_cell_with_less_than_two_neighbors() {
        assert_eq!(Cell::Alive.evolve(0), Cell::Dead);
        assert_eq!(Cell::Alive.evolve(1), Cell::Dead);
    }

    #[test]
    fn evolve_kills_cell_with_more_than_three_neighbors() {
        assert_eq!(Cell::Alive.evolve(4), Cell::Dead);
        assert_eq!(Cell::Alive.evolve(5), Cell::Dead);
    }

    #[test]
    fn count_alive_neighbors_counts_one_neighbor() {
        let conway = Conway::with_cells(&[(0, 0), (0, 1)]);
        assert_eq!(conway.count_alive_neighbors((0, 0)), 1);
    }

    #[test]
    fn count_alive_neighbors_counts_two_neighbors() {
        let conway = Conway::with_cells(&[(0, 5), (0, 6), (0, 7)]);
        assert_eq!(conway.count_alive_neighbors((0, 6)), 2);
    }

    #[test]
    fn get() {
        let conway = Conway::with_cells(&[(0, 0)]);
        assert_eq!(conway.get((0, 0)), Cell::Alive);
        assert_eq!(conway.get((0, 1)), Cell::Dead);
    }

    #[test]
    fn next_evolves_alive_cells() {
        let points = [(0, 0), (0, 1), (0, 2)];
        let conway = Conway::with_cells(&points);
        let result = conway.next();
        for &point in points.iter() {
            let alive_count = conway.count_alive_neighbors(point);
            assert_eq!(result.get(point), conway.get(point).evolve(alive_count))
        }
    }

    #[test]
    fn next_evolves_dead_but_involved_cells() {
        let points = [(0, 0), (1, 0), (0, 1)];
        let conway = Conway::with_cells(&points);
        let result = conway.next();
        let alive_neighbor_count = conway.count_alive_neighbors((1, 1));
        assert_eq!(
            result.get((1, 1)),
            conway.get((1, 1)).evolve(alive_neighbor_count)
        )
    }

    #[test]
    fn with_iter() {
        let conway = Conway::with_iter(iter::once((1, 0)));
        assert_eq!(conway.get((1, 0)), Cell::Alive);
        assert_eq!(conway.get((0, 0)), Cell::Dead);
    }
}
