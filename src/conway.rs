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

struct Conway {
    grid: HashSet<(i32, i32)>,
}

impl Conway {
    pub fn new() -> Conway {
        Conway {
            grid: HashSet::new(),
        }
    }

    pub fn cycle(mut self) -> Conway {
        println!("{:?}", self.reachable_points());
        self.grid = self.reachable_points().iter().filter(|&&point| {
            if self.alive(point) {
                return (2..3).contains(&self.neighbors(point).iter().filter(|&&point| self.alive(point)).count())
            } else {
                return self.neighbors(point).iter().filter(|&&point| self.alive(point)).count() == 3
            }
        }).cloned().collect();
        println!("{:?}", self.grid);
        self
    }

    fn reachable_points(&self) -> HashSet<(i32, i32)> {
        self.grid.iter().flat_map(|&point| {
            let mut neighbors = self.neighbors(point);
            neighbors.insert(point);
            neighbors
        }).collect()
    }

    pub fn neighbors(&self, (row, col): (i32, i32)) -> HashSet<(i32, i32)> {
        RELATIVE_CORDS
            .iter()
            .map(|(x, y)| (row + x, col + y))
            .collect()
    }

    fn alive(&self, pos: (i32, i32)) -> bool {
        return self.grid.contains(&pos)
    }

    #[cfg(test)]
    fn set_alive(&mut self, pos: (i32, i32), alive: bool) {
        if alive {
            self.grid.insert(pos);
        } else {
            self.grid.remove(&pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_kills_adjacent_unpopulated() {
        let mut conway = Conway::new();
        conway.set_alive((0, 0), true);
        conway.set_alive((0, 1), true);
        conway.set_alive((0, 2), true);
        conway = conway.cycle();
        assert!(!conway.alive((0, 0)));
        assert!(conway.alive((0, 1)));
        assert!(!conway.alive((0, 2)));
    }

    #[test]
    fn test_cycle_kills_diagonal_unpopulated() {
        let mut conway = Conway::new();
        conway.set_alive((0, 0), true);
        conway.set_alive((1, 1), true);
        conway.set_alive((2, 2), true);
        conway = conway.cycle();
        assert!(!conway.alive((0, 0)));
        assert!(conway.alive((1, 1)));
        assert!(!conway.alive((2, 2)));
    }

    #[test]
    fn test_cycle_kills_overpopulated() {
        let mut conway = Conway::new();
        conway.set_alive((1, 1), true);
        conway.set_alive((0, 0), true);
        conway.set_alive((2, 0), true);
        conway.set_alive((0, 2), true);
        conway.set_alive((2, 2), true);
        conway = conway.cycle();
        assert!(!conway.alive((1, 1)))
    }

    #[test]
    fn test_cycle_reproduces() {
        let mut conway = Conway::new();
        conway.set_alive((0, 0), true);
        conway.set_alive((1, 0), true);
        conway.set_alive((0, 1), true);
        conway = conway.cycle();
        assert!(conway.alive((1, 1)));
    }
}
