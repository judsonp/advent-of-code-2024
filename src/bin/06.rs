use advent_of_code::util::{
    direction::Direction,
    iter::{CountIf, CountIfParallel},
    point::Point2D,
};
use enumset::EnumSet;
use grid::Grid;
use rayon::iter::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(6);

struct Input {
    grid: Grid<bool>,
    start_location: (isize, isize),
    start_direction: Direction,
}

fn parse_input(input: &str) -> Input {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().trim().len();
    let mut grid = Grid::new(rows, cols);
    let mut start_location = (0, 0);
    let mut start_direction = Direction::North;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            let y = (rows - y - 1) as isize;
            match c {
                '#' => {
                    *grid.get_mut(y, x).unwrap() = true;
                }
                '^' => {
                    start_location = (x, y);
                    start_direction = Direction::North;
                }
                'v' => {
                    start_location = (x, y);
                    start_direction = Direction::South;
                }
                '<' => {
                    start_location = (x, y);
                    start_direction = Direction::West;
                }
                '>' => {
                    start_location = (x, y);
                    start_direction = Direction::East;
                }
                '.' => {}
                _ => panic!("unexpected character: {}", c),
            }
        }
    }

    Input {
        grid,
        start_location,
        start_direction,
    }
}

struct GuardWalk<'a> {
    grid: &'a Grid<bool>,
    location: Point2D<isize>,
    direction: Direction,
    extra_block: Option<Point2D<isize>>,
    done: bool,
}

impl GuardWalk<'_> {
    fn new(grid: &Grid<bool>, location: Point2D<isize>, direction: Direction) -> GuardWalk {
        GuardWalk {
            grid,
            location,
            direction,
            extra_block: None,
            done: false,
        }
    }

    fn new_with_extra_block(
        grid: &Grid<bool>,
        location: Point2D<isize>,
        direction: Direction,
        extra_block: Point2D<isize>,
    ) -> GuardWalk {
        GuardWalk {
            grid,
            location,
            direction,
            extra_block: Some(extra_block),
            done: false,
        }
    }
}

impl Iterator for GuardWalk<'_> {
    type Item = (Point2D<isize>, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let next_location = self.location + self.direction.into();

        let occupied = self.grid.get(next_location.y(), next_location.x());
        if occupied.is_none() {
            // Ran off the grid; done.
            self.done = true;
            return None;
        }

        let occupied = *occupied.unwrap() || self.extra_block == Some(next_location);

        if occupied {
            // Turn right
            self.direction = self.direction.rotate_right();
        } else {
            // Go forward
            self.location = next_location;
        }

        Some((self.location, self.direction))
    }
}

fn find_visited(input: &Input) -> Grid<bool> {
    let mut visited: Grid<bool> = Grid::new(input.grid.rows(), input.grid.cols());

    *visited
        .get_mut(input.start_location.1, input.start_location.0)
        .unwrap() = true;

    GuardWalk::new(
        &input.grid,
        Point2D::from(input.start_location),
        input.start_direction,
    )
    .for_each(|(location, _)| {
        *visited.get_mut(location.y(), location.x()).unwrap() = true;
    });

    visited
}

fn block_makes_cycle(input: &Input, location: Point2D<isize>) -> bool {
    let walk = GuardWalk::new_with_extra_block(
        &input.grid,
        Point2D::from(input.start_location),
        input.start_direction,
        location,
    );

    let mut visited: Grid<EnumSet<Direction>> = Grid::new(input.grid.rows(), input.grid.cols());

    for (location, direction) in walk {
        let v = visited.get_mut(location.y(), location.x()).unwrap();
        if v.contains(direction) {
            return true;
        } else {
            v.insert(direction);
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let visited = find_visited(&input);

    let num_visited = visited.iter().count_if(|&v| v);
    Some(num_visited as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let visited = find_visited(&input);

    let blocking_locations = visited
        .indexed_iter()
        .par_bridge()
        .filter(|(_, &v)| v)
        .count_if(|(location, _)| {
            block_makes_cycle(
                &input,
                Point2D::new(location.1 as isize, location.0 as isize),
            )
        });

    Some(blocking_locations as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
