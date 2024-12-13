use advent_of_code::util::{direction::DIRECTIONS, iter::CountIf, point::Point2D};
use grid::Grid;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut cost = 0;

    visit_connected_regions(
        &input,
        FenceMetrics::default(),
        |state, grid, row, col| {
            state.plots += 1;
            state.perimeter +=
                4 - neighbors(row, col, grid).count_if(|(r, c)| grid[(r, c)] == grid[(row, col)]);
        },
        |state| {
            cost += state.plots * state.perimeter;
        },
    );

    Some(cost as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut cost = 0;

    visit_connected_regions(
        &input,
        BulkFenceMetrics::default(),
        |state, grid, row, col| {
            state.plots += 1;
            state.corners += count_corners(row, col, grid);
        },
        |state| {
            cost += state.plots * state.corners;
        },
    );

    Some(cost as u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct FenceMetrics {
    plots: usize,
    perimeter: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct BulkFenceMetrics {
    plots: usize,
    corners: usize,
}

fn visit_connected_regions<T, S, R, D>(
    grid: &Grid<T>,
    discovery_visitor_initial_state: S,
    discovery_visitor: D,
    mut region_visitor: R,
) where
    T: Clone + PartialEq,
    S: Clone,
    R: FnMut(S),
    D: Fn(&mut S, &Grid<T>, usize, usize),
{
    let mut visited = Grid::new(grid.rows(), grid.cols());

    let mut region_seeds = vec![(0, 0)];

    while let Some((row, col)) = region_seeds.pop() {
        if visited[(row, col)] {
            continue;
        }

        let mut state = discovery_visitor_initial_state.clone();

        let identity = grid[(row, col)].clone();
        let mut visit = vec![(row, col)];
        while let Some((row, col)) = visit.pop() {
            if visited[(row, col)] {
                continue;
            }

            visited[(row, col)] = true;
            discovery_visitor(&mut state, grid, row, col);

            for (r, c) in neighbors(row, col, grid) {
                if grid[(r, c)] == identity {
                    visit.push((r, c));
                } else {
                    region_seeds.push((r, c));
                }
            }
        }

        region_visitor(state);
    }
}

fn count_corners<T>(row: usize, col: usize, grid: &Grid<T>) -> usize
where
    T: Clone + PartialEq,
{
    let mut corners = 0;

    for cnbs in corner_neighbors(row, col, grid) {
        let adjacent_values: [Option<T>; 2] = array_init::from_iter(
            cnbs.adjacent
                .iter()
                .map(|&pos| pos.map(|(r, c)| grid[(r, c)].clone())),
        )
        .unwrap();
        let diagonal_value = cnbs.diagonal.map(|(r, c)| grid[(r, c)].clone());

        // convex corner: both adjacent cells have values different from ours
        if adjacent_values
            .iter()
            .all(|v| *v != Some(grid[(row, col)].clone()))
        {
            corners += 1;
        }

        // concave corner: both adjacent cells have the same value as ours,
        // but the diagonal cell has a value different from ours
        if adjacent_values
            .iter()
            .all(|v| *v == Some(grid[(row, col)].clone()))
            && diagonal_value != Some(grid[(row, col)].clone())
        {
            corners += 1;
        }
    }

    corners
}

struct CornerNeighbors {
    adjacent: [Option<(usize, usize)>; 2],
    diagonal: Option<(usize, usize)>,
}

fn constrain(r: isize, c: isize, rows: isize, cols: isize) -> Option<(usize, usize)> {
    if r >= 0 && r < rows && c >= 0 && c < cols {
        Some((r as usize, c as usize))
    } else {
        None
    }
}

fn corner_neighbors<T>(
    row: usize,
    col: usize,
    grid: &Grid<T>,
) -> impl Iterator<Item = CornerNeighbors>
where
    T: Clone + PartialEq,
{
    let rows = grid.rows() as isize;
    let cols = grid.cols() as isize;

    let row = row as isize;
    let col = col as isize;

    DIRECTIONS
        .iter()
        .map(|&dir| [dir, dir.rotate_right()])
        .map(move |dirs| {
            let dirs: [Point2D<isize>; 2] =
                array_init::from_iter(dirs.iter().map(|&dir| dir.into())).unwrap();
            let adjacent = array_init::from_iter(
                dirs.iter()
                    .map(move |&dir| constrain(row + dir.y(), col + dir.x(), rows, cols)),
            )
            .unwrap();
            let diagonal = constrain(
                row + dirs[0].y() + dirs[1].y(),
                col + dirs[0].x() + dirs[1].x(),
                rows,
                cols,
            );

            CornerNeighbors { adjacent, diagonal }
        })
}

fn neighbors<T>(row: usize, col: usize, grid: &Grid<T>) -> impl Iterator<Item = (usize, usize)> {
    let rows = grid.rows() as isize;
    let cols = grid.cols() as isize;

    let row = row as isize;
    let col = col as isize;

    DIRECTIONS.iter().filter_map(move |&dir| {
        let dir_pt: Point2D<isize> = dir.into();
        constrain(row + dir_pt.y(), col + dir_pt.x(), rows, cols)
    })
}

fn parse_input(input: &str) -> Grid<char> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
