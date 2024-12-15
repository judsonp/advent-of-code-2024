use advent_of_code::util::{direction::Direction, point::Point2D};
use grid::Grid;
use itertools::Itertools;
use smallvec::SmallVec;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let Input {
        mut map,
        robot,
        instructions,
    } = input;

    follow_instructions(&mut map, robot, &instructions);

    Some(map_score(&map))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let Input {
        map,
        robot,
        instructions,
    } = input;

    let mut map = widen_map(map);
    let robot = Point2D::new(robot.x() * 2, robot.y());

    follow_instructions_wide(&mut map, robot, &instructions);

    Some(map_score(&map))
}

fn widen_map(map: Map) -> Map {
    let mut new_map = Grid::init(map.rows(), map.cols() * 2, MapItem::Empty);
    map.indexed_iter().for_each(|((y, x), &item)| {
        let (left, right) = match item {
            MapItem::Empty => (MapItem::Empty, MapItem::Empty),
            MapItem::Wall => (MapItem::Wall, MapItem::Wall),
            MapItem::Box => (MapItem::LeftBox, MapItem::RightBox),
            MapItem::LeftBox => panic!(),
            MapItem::RightBox => panic!(),
        };
        new_map[(y, x * 2)] = left;
        new_map[(y, x * 2 + 1)] = right;
    });
    new_map
}

#[allow(dead_code)]
fn print_map(map: &Grid<MapItem>, robot: &Point2D<isize>) {
    for y in (0..map.rows()).rev() {
        for x in 0..map.cols() {
            let ch = match map[(y, x)] {
                MapItem::Empty => '.',
                MapItem::Wall => '#',
                MapItem::Box => 'O',
                MapItem::LeftBox => '[',
                MapItem::RightBox => ']',
            };

            if *robot == Point2D::new(x as isize, y as isize) {
                print!("@");
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }
}

fn follow_instructions_wide(map: &mut Map, mut robot: Robot, instructions: &[Direction]) -> Robot {
    for instruction in instructions {
        if can_robot_move(map, robot, *instruction) {
            do_robot_move(map, &mut robot, *instruction);
        }
    }
    robot
}

fn do_robot_move(map: &mut Map, robot: &mut Robot, instruction: Direction) {
    *robot = *robot + instruction.into();
    match map[(robot.y() as usize, robot.x() as usize)] {
        MapItem::Empty => (),
        MapItem::Wall => panic!(),
        MapItem::Box => panic!(),
        MapItem::LeftBox => push_box(map, *robot, instruction),
        MapItem::RightBox => push_box(map, *robot - Point2D::new(1, 0), instruction),
    }
}

fn push_box(map: &mut Map, left_box_loc: Point2D<isize>, instruction: Direction) {
    let right_box_loc = left_box_loc + Point2D::new(1, 0);

    match instruction {
        Direction::East => {
            let clear_loc = right_box_loc + instruction.into();
            match map[(clear_loc.y() as usize, clear_loc.x() as usize)] {
                MapItem::Empty => (),
                MapItem::Wall => panic!(),
                MapItem::Box => panic!(),
                MapItem::LeftBox => push_box(map, clear_loc, instruction),
                MapItem::RightBox => panic!(),
            }
            map[(clear_loc.y() as usize, clear_loc.x() as usize)] = MapItem::RightBox;
            map[(right_box_loc.y() as usize, right_box_loc.x() as usize)] = MapItem::LeftBox;
            map[(left_box_loc.y() as usize, left_box_loc.x() as usize)] = MapItem::Empty;
        }

        Direction::West => {
            let clear_loc = left_box_loc + instruction.into();
            match map[(clear_loc.y() as usize, clear_loc.x() as usize)] {
                MapItem::Empty => (),
                MapItem::Wall => panic!(),
                MapItem::Box => panic!(),
                MapItem::LeftBox => panic!(),
                MapItem::RightBox => push_box(map, clear_loc - Point2D::new(1, 0), instruction),
            }
            map[(clear_loc.y() as usize, clear_loc.x() as usize)] = MapItem::LeftBox;
            map[(left_box_loc.y() as usize, left_box_loc.x() as usize)] = MapItem::RightBox;
            map[(right_box_loc.y() as usize, right_box_loc.x() as usize)] = MapItem::Empty;
        }

        Direction::North | Direction::South => {
            let clear_locs = [
                left_box_loc + instruction.into(),
                right_box_loc + instruction.into(),
            ];

            let left_box_locs_to_push = clear_locs
                .iter()
                .filter_map(|&loc| match map[(loc.y() as usize, loc.x() as usize)] {
                    MapItem::Empty => None,
                    MapItem::Wall => panic!(),
                    MapItem::Box => panic!(),
                    MapItem::LeftBox => Some(loc),
                    MapItem::RightBox => Some(loc - Point2D::new(1, 0)),
                })
                .unique()
                .collect::<SmallVec<[_; 2]>>();

            left_box_locs_to_push
                .into_iter()
                .for_each(|loc| push_box(map, loc, instruction));

            map[(clear_locs[0].y() as usize, clear_locs[0].x() as usize)] = MapItem::LeftBox;
            map[(clear_locs[1].y() as usize, clear_locs[1].x() as usize)] = MapItem::RightBox;
            map[(left_box_loc.y() as usize, left_box_loc.x() as usize)] = MapItem::Empty;
            map[(right_box_loc.y() as usize, right_box_loc.x() as usize)] = MapItem::Empty;
        }
    }
}

fn can_robot_move(map: &Map, robot: Robot, instruction: Direction) -> bool {
    let next_position = robot + instruction.into();
    match map[(next_position.y() as usize, next_position.x() as usize)] {
        MapItem::Empty => true,
        MapItem::Wall => false,
        MapItem::Box => panic!(),
        MapItem::LeftBox => can_box_move(map, next_position, instruction),
        MapItem::RightBox => can_box_move(map, next_position - Point2D::new(1, 0), instruction),
    }
}

fn can_box_move(map: &Map, left_box_position: Point2D<isize>, instruction: Direction) -> bool {
    let right_box_position = left_box_position + Point2D::new(1, 0);
    assert!(
        map[(
            left_box_position.y() as usize,
            left_box_position.x() as usize
        )] == MapItem::LeftBox
    );
    assert!(
        map[(
            right_box_position.y() as usize,
            right_box_position.x() as usize
        )] == MapItem::RightBox
    );

    match instruction {
        Direction::East => {
            let next_position = right_box_position + instruction.into();
            match map[(next_position.y() as usize, next_position.x() as usize)] {
                MapItem::Empty => true,
                MapItem::Wall => false,
                MapItem::Box => panic!(),
                MapItem::LeftBox => can_box_move(map, next_position, instruction),
                MapItem::RightBox => panic!(),
            }
        }

        Direction::West => {
            let next_position = left_box_position + instruction.into();
            match map[(next_position.y() as usize, next_position.x() as usize)] {
                MapItem::Empty => true,
                MapItem::Wall => false,
                MapItem::Box => panic!(),
                MapItem::LeftBox => panic!(),
                MapItem::RightBox => {
                    can_box_move(map, next_position - Point2D::new(1, 0), instruction)
                }
            }
        }

        Direction::North | Direction::South => {
            let next_positions = [
                left_box_position + instruction.into(),
                right_box_position + instruction.into(),
            ];
            next_positions.iter().all(|&next_position| {
                match map[(next_position.y() as usize, next_position.x() as usize)] {
                    MapItem::Empty => true,
                    MapItem::Wall => false,
                    MapItem::Box => panic!(),
                    MapItem::LeftBox => can_box_move(map, next_position, instruction),
                    MapItem::RightBox => {
                        can_box_move(map, next_position - Point2D::new(1, 0), instruction)
                    }
                }
            })
        }
    }
}

fn follow_instructions(map: &mut Map, mut robot: Robot, instructions: &[Direction]) -> Robot {
    for instruction in instructions {
        do_move(map, &mut robot, *instruction);
    }
    robot
}

fn do_move(map: &mut Map, robot: &mut Robot, instruction: Direction) {
    let mut first_nonbox_space = *robot + instruction.into();

    while map[(
        first_nonbox_space.y() as usize,
        first_nonbox_space.x() as usize,
    )] == MapItem::Box
    {
        first_nonbox_space = first_nonbox_space + instruction.into();
    }

    if map[(
        first_nonbox_space.y() as usize,
        first_nonbox_space.x() as usize,
    )] == MapItem::Wall
    {
        return;
    }

    *robot = *robot + instruction.into();

    map[(robot.y() as usize, robot.x() as usize)] = MapItem::Empty;

    let mut box_position = *robot;
    while box_position != first_nonbox_space {
        box_position = box_position + instruction.into();
        map[(box_position.y() as usize, box_position.x() as usize)] = MapItem::Box;
    }
}

fn map_score(map: &Map) -> u64 {
    map.indexed_iter()
        .filter(|(_, &item)| item == MapItem::Box || item == MapItem::LeftBox)
        .map(|((y, x), _)| box_gps_score(x, y, map.rows()))
        .sum()
}

fn box_gps_score(x: usize, y: usize, map_height: usize) -> u64 {
    let distance_from_top = map_height - y - 1;
    (distance_from_top * 100 + x) as u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    Empty,
    Wall,
    Box,
    LeftBox,
    RightBox,
}

type Map = Grid<MapItem>;
type Robot = Point2D<isize>;

#[derive(Debug, Clone)]
struct Input {
    map: Map,
    robot: Robot,
    instructions: Vec<Direction>,
}

fn parse_input(input: &str) -> Input {
    let rows = input.lines().take_while(|line| !line.is_empty()).count();
    let cols = input.lines().next().unwrap().trim().len();

    let mut map = Grid::init(rows, cols, MapItem::Empty);
    let mut robot = None;

    let mut lines = input.lines();

    for (y, line) in lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
    {
        for (x, ch) in line.chars().enumerate() {
            let y = rows - y - 1;
            map[(y, x)] = match ch {
                '#' => MapItem::Wall,
                '.' => MapItem::Empty,
                'O' => MapItem::Box,
                '@' => {
                    robot = Some(Point2D::new(x as isize, y as isize));
                    MapItem::Empty
                }
                _ => panic!("Unexpected character in map input"),
            };
        }
    }

    let mut instructions = Vec::new();

    for line in lines {
        for ch in line.chars() {
            instructions.push(match ch {
                '^' => Direction::North,
                'v' => Direction::South,
                '>' => Direction::East,
                '<' => Direction::West,
                _ => panic!("Unexpected character in instructions input"),
            });
        }
    }

    Input {
        map,
        robot: robot.expect("Robot position not found in input"),
        instructions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(input.map.rows(), 10);
        assert_eq!(input.map.cols(), 10);
        assert_eq!(input.map[(0, 0)], MapItem::Wall);
        assert_eq!(input.map[(1, 5)], MapItem::Box);
        assert_eq!(input.map[(1, 6)], MapItem::Empty);
        assert_eq!(input.robot, Point2D::new(4, 5));
        assert_eq!(input.instructions.len(), 70 * 10);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
