use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    fmt::{self, Display, Formatter},
};
use rand::{Rng, thread_rng};

#[cfg(test)]
mod tests;

#[inline]
fn safe_add(u: usize, i: i64) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as usize as usize
    } else {
        u + i as usize
    }
}

pub type Point = (usize, usize);
pub type Path = Vec<Point>;

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    n: usize,
    m: usize,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(n: usize, m: usize) -> Self {
        Self { data: vec![T::default(); n * m], n, m }
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let (i, j) = index;
        if i >= self.n || j >= self.m { panic!("index out of range"); }

        &self.data[i * self.m + j]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let (i, j) = index;
        if i >= self.n || j >= self.m { panic!("index out of range"); }

        &mut self.data[i * self.m + j]
    }
}

/// Von Neumann neighbourhood.
struct Neighbours {
    point: Point,
    neighbour: usize,
}

impl Neighbours {
    /// Returns iterator on neighbours of a point.
    pub fn of(point: Point) -> Self {
        Self { point, neighbour: 0 }
    }
}

impl Iterator for Neighbours {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = match self.neighbour {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => return None,
        };
        let (x, y) = self.point;
        let x = safe_add(x, i);
        let y = safe_add(y, j);

        self.neighbour += 1;

        Some((x, y))
    }
}

pub type Maze = Matrix<bool>;

impl Maze {
    /// Generates a maze using the randomized Prim's algorithm.
    pub fn generate(n: usize, m: usize) -> Self {
        let mut maze = Self { data: vec![true; n * m], n, m };
        let mut cells = Vec::new();
        let mut rng = thread_rng();

        // Firstly, choose a default exit.
        let mut x = rng.gen_range(0..n);
        let mut y = rng.gen_range(0..m);
        if rng.gen::<bool>() {
            maze[(x, 0)] = false;
            y = 0;
        } else {
            maze[(0, y)] = false;
            x = 0;
        }
        let start = (x, y);

        // Then, add neighbouring filled cells to list.
        for neighbour in Neighbours::of(start) {
            if maze.is_valid(neighbour) && maze[neighbour] { cells.push(neighbour); }
        }

        while let Some(current) = cells.remove_random(&mut rng) {
            let mut explored = 0;
            for next in Neighbours::of(current) {
                if maze.is_valid(next) && !maze[next] { explored += 1; }
            }
            if explored < 2 {
                maze[current] = false;
                for next in Neighbours::of(current) {
                    if maze.is_valid(next) && maze[next] { cells.push(next) }
                }
            }
        }

        maze
    }

    fn is_exit(&self, point: Point) -> bool {
        point.0 == 0 || point.0 == self.n - 1
            || point.1 == 0 || point.1 == self.m - 1
    }

    /// Bounds check.
    fn is_valid(&self, point: Point) -> bool {
        point.0 < self.n && point.1 < self.m
    }

    /// Returns a path from the `start` point to an exit, if exists.
    pub fn solve(&self, start: Point) -> Option<Path> {
        if self[start] { return None; }

        let mut queue = VecDeque::new();
        let mut costs = Matrix::<usize>::new(self.n, self.m);
        let mut exit = None;

        costs[start] = 1;
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if exit.is_some() { break; }
            for next in Neighbours::of(current) {
                if !self.is_valid(next) || self[next] || costs[next] != 0 { continue; }
                if self.is_exit(next) { exit = Some(next); }
                costs[next] = costs[current] + 1;
                queue.push_back(next);
            }
        }

        // Restore a path.
        let mut current = if let Some(point) = exit { point } else { return None; };
        let mut path = vec![current];

        while current != start {
            for next in Neighbours::of(current) {
                if !self.is_valid(next) { continue; }

                if costs[next] != 0 && costs[next] < costs[current] {
                    current = next;
                    path.push(current);
                    break;
                }
            }
        }

        // Change direction.
        let path = path.iter()
            .rev()
            .cloned()
            .collect();

        Some(path)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                let _ = write!(f, "{}", self[(i, j)] as u8);
            }
            let _ = writeln!(f);
        }
        write!(f, "")
    }
}

trait RemoveRandom {
    type Item;

    fn remove_random<R: Rng>(&mut self, rng: &mut R) -> Option<Self::Item>;
}

impl<T> RemoveRandom for Vec<T> {
    type Item = T;

    fn remove_random<R: Rng>(&mut self, rng: &mut R) -> Option<Self::Item> {
        if self.len() == 0 {
            None
        } else {
            let index = rng.gen_range(0..self.len());
            Some(self.swap_remove(index))
        }
    }
}
