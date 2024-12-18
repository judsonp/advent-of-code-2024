use std::collections::BinaryHeap;

use advent_of_code::util::{
    direction::DIRECTIONS,
    grid::{GridGetPoint as _, GridGetPointMut as _},
    point::Point2D,
    DistanceState,
};
use grid::Grid;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    part_one_inner(input, 71, 71, 1024)
}

fn part_one_inner(input: &str, width: usize, height: usize, fallen: usize) -> Option<u64> {
    let bytes = parse_input(input);
    let grid = build_grid(width, height, &bytes[0..fallen]);
    shortest_path_length(
        &grid,
        (0, 0).into(),
        (width as isize - 1, height as isize - 1).into(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_inner(input, 71, 71, 1024)
}

fn part_two_inner(input: &str, width: usize, height: usize, previsit: usize) -> Option<String> {
    let bytes = parse_input(input);

    let mut grid = Grid::init(height, width, true);

    let start = (0, 0).into();
    let end = (width as isize - 1, height as isize - 1).into();

    for byte in &bytes[0..previsit] {
        *grid.point_mut(*byte).unwrap() = false;
    }
    let (_, mut visited) = has_path(&grid, start, end);

    for &byte in &bytes[previsit..] {
        *grid.point_mut(byte).unwrap() = false;

        if *visited.point(byte).unwrap() {
            let (has, new_visited) = has_path(&grid, start, end);
            if !has {
                return Some(format!("{},{}", byte.x(), byte.y()));
            }
            visited = new_visited;
        }
    }

    None
}

fn shortest_path_length(
    grid: &Grid<bool>,
    start: Point2D<isize>,
    end: Point2D<isize>,
) -> Option<u64> {
    let mut visited_distances = Grid::init(grid.rows(), grid.cols(), None);
    let mut visit = BinaryHeap::new();
    *visited_distances.point_mut(start).unwrap() = Some(0);
    visit.push(DistanceState::new(0, start));

    while let Some(DistanceState {
        distance,
        state: location,
    }) = visit.pop()
    {
        if location == end {
            return Some(distance);
        }

        for &dir in DIRECTIONS.iter() {
            let new_loc = location + dir.into();
            if new_loc.x() < 0
                || new_loc.y() < 0
                || new_loc.x() >= grid.cols() as isize
                || new_loc.y() >= grid.rows() as isize
            {
                continue;
            }

            if !*grid.point(new_loc).unwrap() {
                continue;
            }

            let new_distance = distance + 1;
            let visited_distance = visited_distances.point_mut(new_loc).unwrap();
            if visited_distance.is_none() || new_distance < visited_distance.unwrap() {
                *visited_distance = Some(new_distance);
                visit.push(DistanceState::new(new_distance, new_loc));
            }
        }
    }

    None
}

fn has_path(grid: &Grid<bool>, start: Point2D<isize>, end: Point2D<isize>) -> (bool, Grid<bool>) {
    let mut visited = Grid::init(grid.rows(), grid.cols(), false);
    let mut stack = vec![start];

    while let Some(loc) = stack.pop() {
        if loc.x() < 0
            || loc.y() < 0
            || loc.x() >= grid.cols() as isize
            || loc.y() >= grid.rows() as isize
        {
            continue;
        }

        if !*grid.point(loc).unwrap() || *visited.point(loc).unwrap() {
            continue;
        }

        if loc == end {
            return (true, visited);
        }

        *visited.point_mut(loc).unwrap() = true;
        DIRECTIONS
            .iter()
            .rev()
            .for_each(|&dir| stack.push(loc + dir.into()));
    }

    (false, visited)
}

fn build_grid(width: usize, height: usize, bytes: &[Point2D<isize>]) -> Grid<bool> {
    let mut grid = Grid::init(width, height, true);

    for byte in bytes {
        *grid.point_mut(*byte).unwrap() = false;
        let (x, y) = (byte.x(), byte.y());
        *grid.point_mut((x, y)).unwrap() = false;
    }

    grid
}

fn parse_input(input: &str) -> Vec<Point2D<isize>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point2D::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_inner(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_inner(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
