use advent_of_code::util::{
    grid::{IntoGridStrideIterator, Offset},
    iter::CountIf,
};
use grid::Grid;
use itertools::equal;

advent_of_code::solution!(4);

const DIRECTIONS: [Offset; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const XA_OFFSETS: [(Offset, Offset); 2] = [((-1, -1), (1, 1)), ((1, 1), (-1, -1))];

const XB_OFFSETS: [(Offset, Offset); 2] = [((-1, 1), (1, -1)), ((1, -1), (-1, 1))];

pub fn parse_input(input: &str) -> Grid<char> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().trim().len();
    let mut grid = Grid::new(rows, cols);

    grid.iter_mut()
        .zip(input.lines().flat_map(|line| line.chars()))
        .for_each(|(cell, c)| {
            *cell = c;
        });

    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let result: usize = grid
        .indexed_iter()
        .map(|(index, _)| {
            DIRECTIONS.iter().count_if(|stride| {
                equal(
                    grid.stride_iter(index, *stride).take(4).copied(),
                    "XMAS".chars(),
                )
            })
        })
        .sum();

    Some(result as u64)
}

fn is_ms(grid: &Grid<char>, index: (usize, usize), offsets: &[(Offset, Offset)]) -> bool {
    offsets.iter().any(|(offset_a, offset_b)| {
        let index_a = (
            index.0.checked_add_signed(offset_a.0).unwrap(),
            index.1.checked_add_signed(offset_a.1).unwrap(),
        );
        let index_b = (
            index.0.checked_add_signed(offset_b.0).unwrap(),
            index.1.checked_add_signed(offset_b.1).unwrap(),
        );
        grid[index_a] == 'M' && grid[index_b] == 'S'
    })
}

fn in_bounds(grid: &Grid<char>, index: &(usize, usize)) -> bool {
    index.0 > 0 && index.0 < grid.rows() - 1 && index.1 > 0 && index.1 < grid.cols() - 1
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let result = grid.indexed_iter().count_if(|(index, &value)| {
        value == 'A'
            && in_bounds(&grid, &index)
            && is_ms(&grid, index, &XA_OFFSETS)
            && is_ms(&grid, index, &XB_OFFSETS)
    });

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
