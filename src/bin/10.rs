use advent_of_code::util::{direction::DIRECTIONS, point::Point2D};
use grid::Grid;
use petgraph::{
    algo::all_simple_paths,
    graph::NodeIndex,
    visit::{Bfs, Walker},
    Graph,
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let (graph, _) = build_graph(&grid);

    let result = graph
        .node_indices()
        .filter(|index| *graph.node_weight(*index).unwrap() == 0)
        .map(|index| trailhead_score(&graph, index))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let (graph, _) = build_graph(&grid);

    let result = graph
        .node_indices()
        .filter(|index| *graph.node_weight(*index).unwrap() == 0)
        .map(|index| trailhead_rating(&graph, index))
        .sum();

    Some(result)
}

fn trailhead_score(graph: &Graph<u8, ()>, index: NodeIndex) -> u64 {
    Bfs::new(graph, index)
        .iter(graph)
        .filter(|index| graph[*index] == 9)
        .count() as u64
}

fn trailhead_rating(graph: &Graph<u8, ()>, index: NodeIndex) -> u64 {
    Bfs::new(graph, index)
        .iter(graph)
        .filter(|target| graph[*target] == 9)
        .map(|target| all_simple_paths::<Vec<_>, _>(graph, index, target, 0, None).count() as u64)
        .sum()
}

fn parse_input(input: &str) -> Grid<u8> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().trim().len();
    let mut grid = Grid::new(rows, cols);

    grid.iter_mut()
        .zip(input.lines().flat_map(|line| line.chars()))
        .for_each(|(cell, c)| {
            *cell = c.to_digit(10).unwrap() as u8;
        });

    grid
}

fn build_graph(grid: &Grid<u8>) -> (Graph<u8, ()>, Grid<NodeIndex>) {
    let mut indexes = Grid::new(grid.rows(), grid.cols());
    let mut graph = Graph::new();

    for ((row, col), _) in grid.indexed_iter() {
        let index = graph.add_node(grid[(row, col)]);
        indexes[(row, col)] = index;
    }

    for ((row, col), &value) in grid.indexed_iter() {
        for (nr, nc) in neighbors(row, col, grid) {
            let nv = grid[(nr, nc)];
            if nv == value + 1 {
                let a = indexes[(row, col)];
                let b = indexes[(nr, nc)];
                graph.add_edge(a, b, ());
            }
        }
    }

    (graph, indexes)
}

fn neighbors<T>(row: usize, col: usize, grid: &Grid<T>) -> impl Iterator<Item = (usize, usize)> {
    let rows = grid.rows() as isize;
    let cols = grid.cols() as isize;

    let row = row as isize;
    let col = col as isize;

    DIRECTIONS.iter().filter_map(move |&dir| {
        let dir_pt: Point2D<isize> = dir.into();
        let r = row + dir_pt.y();
        let c = col + dir_pt.x();
        if r >= 0 && r < rows && c >= 0 && c < cols {
            Some((r as usize, c as usize))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
