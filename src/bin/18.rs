use std::collections::HashMap;

use advent_of_code::util::{
    direction::DIRECTIONS,
    grid::{GridGetPoint as _, GridGetPointMut as _},
    point::Point2D,
};
use grid::Grid;
use petgraph::{algo::dijkstra, graph::NodeIndex, Graph, Undirected};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    part_one_inner(input, 71, 71, 1024)
}

fn part_one_inner(input: &str, width: usize, height: usize, fallen: usize) -> Option<u64> {
    let bytes = parse_input(input);
    let grid = build_grid(width, height, &bytes[0..fallen]);
    let (graph, nodes) = build_graph(width, height, grid);

    let start = *nodes.get(&Point2D::new(0, 0)).unwrap();
    let end = *nodes
        .get(&Point2D::new(width as isize - 1, height as isize - 1))
        .unwrap();

    let costs = dijkstra(&graph, start, Some(end), |_| 1);
    let cost = *costs.get(&end).unwrap();

    Some(cost)
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_inner(input, 71, 71)
}

fn part_two_inner(input: &str, width: usize, height: usize) -> Option<String> {
    let bytes = parse_input(input);

    let mut grid = Grid::init(height, width, true);

    let start = (0, 0).into();
    let end = (width as isize - 1, height as isize - 1).into();

    let mut visited = Grid::init(height, width, true);
    for byte in bytes {
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

fn build_graph(
    width: usize,
    height: usize,
    usable: Grid<bool>,
) -> (
    Graph<(), (), Undirected>,
    HashMap<Point2D<isize>, NodeIndex>,
) {
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    let mut nodes = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let point = Point2D::new(x as isize, y as isize);
            if !usable.point(point).unwrap() {
                continue;
            }

            let node = graph.add_node(());
            nodes.insert(point, node);
        }
    }

    for y in 0..height {
        for x in 0..width {
            let point = Point2D::new(x as isize, y as isize);
            let node = nodes.get(&point);
            if node.is_none() {
                continue;
            }
            let node = *node.unwrap();

            let neighbors = DIRECTIONS.map(|dir| point + dir.into());
            for neighbor in neighbors {
                if let Some(neighbor_node) = nodes.get(&neighbor) {
                    graph.add_edge(node, *neighbor_node, ());
                }
            }
        }
    }

    (graph, nodes)
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
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), 7, 7);
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
