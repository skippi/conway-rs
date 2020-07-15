use std::collections::HashSet;

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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn neighborhood(&self) -> Vec<Point> {
        let mut neighbors = self.neighbors();
        neighbors.push(self.clone());
        neighbors
    }

    fn neighbors(&self) -> Vec<Point> {
        RELATIVE_CORDS
            .iter()
            .map(|(x, y)| Point(self.0 + x, self.1 + y))
            .collect()
    }
}

struct Conway {
    grid: HashSet<Point>,
}

impl Conway {
    pub fn new() -> Conway {
        Conway {
            grid: HashSet::new(),
        }
    }

    pub fn cycle(mut self) -> Conway {
        self.grid = self
            .grid
            .iter()
            .flat_map(|point| point.neighborhood())
            .filter(|&point| self.will_live(point))
            .collect();
        self
    }

    fn will_live(&self, point: Point) -> bool {
        let alive_neighbor_count = self.alive_neighbors_count(point);
        if self.is_alive(point) {
            (2..3).contains(&alive_neighbor_count)
        } else {
            alive_neighbor_count == 3
        }
    }

    fn is_alive(&self, point: Point) -> bool {
        self.grid.contains(&point)
    }

    fn alive_neighbors_count(&self, point: Point) -> usize {
        point
            .neighbors()
            .iter()
            .filter(|&&p| self.is_alive(p))
            .count()
    }

    #[cfg(test)]
    fn set_alive(&mut self, point: Point, alive: bool) {
        if alive {
            self.grid.insert(point);
        } else {
            self.grid.remove(&point);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_kills_adjacent_unpopulated() {
        let mut conway = Conway::new();
        conway.set_alive(Point(0, 0), true);
        conway.set_alive(Point(0, 1), true);
        conway.set_alive(Point(0, 2), true);
        conway = conway.cycle();
        assert!(!conway.is_alive(Point(0, 0)));
        assert!(conway.is_alive(Point(0, 1)));
        assert!(!conway.is_alive(Point(0, 2)));
    }

    #[test]
    fn test_cycle_kills_diagonal_unpopulated() {
        let mut conway = Conway::new();
        conway.set_alive(Point(0, 0), true);
        conway.set_alive(Point(1, 1), true);
        conway.set_alive(Point(2, 2), true);
        conway = conway.cycle();
        assert!(!conway.is_alive(Point(0, 0)));
        assert!(conway.is_alive(Point(1, 1)));
        assert!(!conway.is_alive(Point(2, 2)));
    }

    #[test]
    fn test_cycle_kills_overpopulated() {
        let mut conway = Conway::new();
        conway.set_alive(Point(1, 1), true);
        conway.set_alive(Point(0, 0), true);
        conway.set_alive(Point(2, 0), true);
        conway.set_alive(Point(0, 2), true);
        conway.set_alive(Point(2, 2), true);
        conway = conway.cycle();
        assert!(!conway.is_alive(Point(1, 1)))
    }

    #[test]
    fn test_cycle_reproduces() {
        let mut conway = Conway::new();
        conway.set_alive(Point(0, 0), true);
        conway.set_alive(Point(1, 0), true);
        conway.set_alive(Point(0, 1), true);
        conway = conway.cycle();
        assert!(conway.is_alive(Point(1, 1)));
    }
}
