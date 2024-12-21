use std::{
    collections::HashMap,
    iter::{once, repeat, successors},
};

use advent_of_code::util::point::Point2D;
use arrayvec::ArrayVec;
use derive_more::derive::Constructor;
use itertools::Itertools;

advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirectionalPad {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

const DIR_PAD_BUTTONS: [DirectionalPad; 5] = [
    DirectionalPad::Up,
    DirectionalPad::Down,
    DirectionalPad::Left,
    DirectionalPad::Right,
    DirectionalPad::Activate,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Keypad(u8);

const KEYPAD_BUTTONS: [Keypad; 11] = Keypad::buttons();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Constructor)]
struct Movement<T> {
    from: T,
    to: T,
}

type Direction = DirectionalPad;
type Path = ArrayVec<Direction, 5>;

// The accumulated cost of pressing a button on a directional pad, if this robot is already pointing at
// a given location (the from in the Movement)
type DirpadCostMap = HashMap<Movement<DirectionalPad>, u64>;
// The accumulated cost of pressing a button on the keypad, if the robot is already pointing at
// a given location (the from in the Movement)
type KeypadCostMap = HashMap<Movement<Keypad>, u64>;

type Code = ArrayVec<u8, 4>;
type Input = Vec<Code>;

trait Pathable {
    fn path(self, to: Self) -> ArrayVec<Path, 2>;
}

pub fn part_one(input: &str) -> Option<u64> {
    // Human button-pressing just costs one action per button press and is our baseline.
    let human_costs = human_dirpad_press_costs();
    // The cost of pressing buttons using the very cold robot is based on how many human button presses it took.
    let cold_robot_costs = compute_dirpad_press_costs(&human_costs);
    // That cost passes through to the radiation robot, whose costs are based on the costs by the cold robot.
    let rad_robot_costs = compute_dirpad_press_costs(&cold_robot_costs);
    // The robot in a vacuum with a keypad, finally, is based on the costs of the radiation robot (but with a different keypad layout).
    let vacuum_robot_costs = compute_keypad_press_costs(&rad_robot_costs);

    Some(part_any(input, &vacuum_robot_costs))
}

pub fn part_two(input: &str) -> Option<u64> {
    // dirpad_costs[0] is simply the 1 costs of a human pressing its directional pad buttons
    // dirpad_costs[N] is the cost of pressing buttons on the Nth robot, whose costs are based on the
    //   costs of the N-1th robot (or human)
    let dirpad_costs = successors(Some(human_dirpad_press_costs()), |previous_cost| {
        Some(compute_dirpad_press_costs(previous_cost))
    })
    .take(26)
    .collect_vec();

    // the keypad robot is based on the 25th intermediate robot
    let keypad_costs = compute_keypad_press_costs(&dirpad_costs[25]);

    Some(part_any(input, &keypad_costs))
}

fn part_any(input: &str, keypad_costs: &KeypadCostMap) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|code| cost_to_enter(&code, keypad_costs) * numeric_part(&code))
        .sum()
}

// Returns the cost to enter a given code, assuming that all robots start with their pusher over the 'A' button.
fn cost_to_enter(code: &Code, cost_map: &KeypadCostMap) -> u64 {
    // the sequence of keypad states: A->1->2->3->A (the terminal A is already part of the code)
    once(Keypad(10))
        .chain(code.iter().map(|k| Keypad(*k)))
        // the sequence of Movements (a from, to pair) that is entering the code, e.g., (A, 1)
        .tuple_windows()
        .map(|(from, to)| Movement::new(from, to))
        // the cost of those movements
        .map(|action| &cost_map[&action])
        .sum()
}

// This is the cost to move to and press a key on the keypad,
// given the cost of making actions on the last directional pad.
// To press a key, you have to move to it from the previous key,
// using the directional pad, and then press A on the dirpad.
// Immediately after pressing a key , all directional pads
// must necessarily be pointing at A. So, before and after resolving
// a keypress, all directional pads are at A.
fn compute_keypad_press_costs(above_costs: &DirpadCostMap) -> KeypadCostMap {
    compute_press_costs(&KEYPAD_BUTTONS, above_costs)
}

// This is the cost to move to and press a key on a directional pad,
// given the cost of making actions on the previous directional pad.
// To press a key, you have to move to it from the previous key,
// using the upper-level keypad, and then press A on the upper-level keypad.
// Immediately after pressing a key at this layer, all directional pads
// above it must necessarily be pointing at A. So, before and after resolving
// a keypress at this level, all keypads above this level are at A.
fn compute_dirpad_press_costs(above_costs: &DirpadCostMap) -> DirpadCostMap {
    compute_press_costs(&DIR_PAD_BUTTONS, above_costs)
}

fn compute_press_costs<T>(elements: &[T], above_costs: &DirpadCostMap) -> HashMap<Movement<T>, u64>
where
    T: Pathable + Clone + Copy + Eq + std::hash::Hash,
{
    elements
        .iter()
        .cloned()
        .cartesian_product(elements.iter().cloned())
        .map(|(from, to)| Movement::new(from, to))
        .map(|movement| {
            // All of the legal paths to get from `from` to `to`
            let cost = movement
                .from
                .path(movement.to)
                .iter()
                .map(|path| path_press_cost(path, above_costs))
                .min()
                .unwrap();
            (movement, cost)
        })
        .collect()
}

// Given a sequence of directional movements that need to be made to get to a button
// from the starting point of the robot (some other button), computes the total cost
// of moving to the button and also pressing it.
// Because all of the actions we capture are moving to and pressing a button, which
// requires ending on the A key, we know that the starting state of the previous robot
// is over the A key.
// So the cost to move a series of steps and then press the button is the cost of
// pressing each of the corresponding movement buttons at the upper-level robot, starting
// from A (where we last left off), and then pressing A.
fn path_press_cost(path: &[Direction], above_costs: &DirpadCostMap) -> u64 {
    // path (input) is the path in directions on this keypad to get to the new location

    // actions is the sequence of actions (in Movement) to take on the upper-level keypad
    // in order to move to the new location (follow the path) and press Activate, causing
    // the key to be pressed.
    // The upper-level pad starts at activate, and we end by going to and hitting activate.
    once(DirectionalPad::Activate)
        .chain(path.iter().cloned())
        .chain(once(DirectionalPad::Activate))
        .tuple_windows()
        .map(|(from, to)| Movement::new(from, to))
        // each Movement translates to a cost in the upper-level keypad
        .map(|movement| &above_costs[&movement])
        .sum()
}

fn human_dirpad_press_costs() -> DirpadCostMap {
    DIR_PAD_BUTTONS
        .iter()
        .cloned()
        .cartesian_product(DIR_PAD_BUTTONS.iter().cloned())
        .map(|(from, to)| Movement::new(from, to))
        .map(|movement| (movement, 1))
        .collect()
}

impl DirectionalPad {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+

    fn coords(self) -> Point2D<isize> {
        match self {
            DirectionalPad::Up => (1, 0),
            DirectionalPad::Down => (1, 1),
            DirectionalPad::Left => (0, 1),
            DirectionalPad::Right => (2, 1),
            DirectionalPad::Activate => (2, 0),
        }
        .into()
    }
}

impl Pathable for DirectionalPad {
    // Returns the set of possibly-most-efficient paths from this location
    // to the target location.
    // This uses the fact that there are only two potentially most efficient
    // paths: either doing all of the LR movement first, or doing all of the
    // UD movement first. This necessarily minimizes movement in the upper-level
    // directional pad.
    // If to and from are the same location, returns a single empty path.
    fn path(self, to: Self) -> ArrayVec<Path, 2> {
        let from = self;
        let delta = to.coords() - from.coords();

        if delta.x() == 0 && delta.y() == 0 {
            return ArrayVec::from_iter(once(ArrayVec::new()));
        }

        let lr = repeat(if delta.x() > 0 {
            Direction::Right
        } else {
            Direction::Left
        })
        .take(delta.x().unsigned_abs())
        .collect::<Path>();
        let ud = repeat(if delta.y() > 0 {
            Direction::Down
        } else {
            Direction::Up
        })
        .take(delta.y().unsigned_abs())
        .collect::<Path>();

        if from == Direction::Left {
            // to avoid the hole, only this path is valid
            ArrayVec::from_iter(once(lr.iter().chain(ud.iter()).cloned().collect::<Path>()))
        } else if to == Direction::Left {
            // to avoid the hole, only this path is valid
            ArrayVec::from_iter(once(ud.iter().chain(lr.iter()).cloned().collect::<Path>()))
        } else {
            // you can do one or the other of the buttons first
            ArrayVec::from([
                lr.iter().chain(ud.iter()).cloned().collect(),
                ud.iter().chain(lr.iter()).cloned().collect(),
            ])
        }
    }
}

impl Keypad {
    const fn buttons() -> [Keypad; 11] {
        let mut buttons = [Keypad(0); 11];
        let mut i = 0;
        while i < buttons.len() {
            buttons[i] = Keypad(i as u8);
            i += 1;
        }
        buttons
    }

    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    fn coords(self) -> Point2D<isize> {
        match self.0 {
            7 => (0, 0),
            8 => (1, 0),
            9 => (2, 0),
            4 => (0, 1),
            5 => (1, 1),
            6 => (2, 1),
            1 => (0, 2),
            2 => (1, 2),
            3 => (2, 2),
            0 => (1, 3),
            10 => (2, 3),
            _ => panic!(),
        }
        .into()
    }
}

impl Pathable for Keypad {
    // Returns the set of possibly-most-efficient paths from this location
    // to the target location.
    // This uses the fact that there are only two potentially most efficient
    // paths: either doing all of the LR movement first, or doing all of the
    // UD movement first. This necessarily minimizes movement in the upper-level
    // directional pad.
    // If to and from are the same location, returns a single empty path.
    fn path(self, to: Self) -> ArrayVec<Path, 2> {
        let from = self;
        let to_pt = to.coords();
        let from_pt = from.coords();
        let delta = to_pt - from_pt;

        if delta.x() == 0 && delta.y() == 0 {
            return ArrayVec::from_iter(once(ArrayVec::new()));
        }

        let lr = repeat(if delta.x() > 0 {
            Direction::Right
        } else {
            Direction::Left
        })
        .take(delta.x().unsigned_abs())
        .collect::<Path>();
        let ud = repeat(if delta.y() > 0 {
            Direction::Down
        } else {
            Direction::Up
        })
        .take(delta.y().unsigned_abs())
        .collect::<Path>();

        if from_pt.y() == 3 && to_pt.x() == 0 {
            ArrayVec::from_iter(once(ud.iter().chain(lr.iter()).cloned().collect::<Path>()))
        } else if to_pt.y() == 3 && from_pt.x() == 0 {
            ArrayVec::from_iter(once(lr.iter().chain(ud.iter()).cloned().collect::<Path>()))
        } else {
            ArrayVec::from([
                lr.iter().chain(ud.iter()).cloned().collect(),
                ud.iter().chain(lr.iter()).cloned().collect(),
            ])
        }
    }
}

fn numeric_part(code: &Code) -> u64 {
    code.iter().take(3).fold(0, |a, c| a * 10 + *c as u64)
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0'..='9' => c.to_digit(10).unwrap() as u8,
                    'A' => 10u8,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Keypad {
        fn apply(self, instruction: DirectionalPad) -> Option<Keypad> {
            match (self.0, instruction) {
                // bottom row
                (0, DirectionalPad::Up) => Some(2),
                (0, DirectionalPad::Right) => Some(10),
                (10, DirectionalPad::Up) => Some(3),
                (10, DirectionalPad::Left) => Some(0),
                // 1-3 row
                (1, DirectionalPad::Up) => Some(4),
                (1, DirectionalPad::Right) => Some(2),
                (2, DirectionalPad::Left) => Some(1),
                (2, DirectionalPad::Right) => Some(3),
                (2, DirectionalPad::Down) => Some(0),
                (2, DirectionalPad::Up) => Some(5),
                (3, DirectionalPad::Up) => Some(6),
                (3, DirectionalPad::Down) => Some(10),
                (3, DirectionalPad::Left) => Some(2),
                // 4-6 row
                (4, DirectionalPad::Up) => Some(7),
                (4, DirectionalPad::Right) => Some(5),
                (4, DirectionalPad::Down) => Some(1),
                (5, DirectionalPad::Left) => Some(4),
                (5, DirectionalPad::Right) => Some(6),
                (5, DirectionalPad::Down) => Some(2),
                (5, DirectionalPad::Up) => Some(8),
                (6, DirectionalPad::Up) => Some(9),
                (6, DirectionalPad::Down) => Some(3),
                (6, DirectionalPad::Left) => Some(5),
                // 7-9 row
                (7, DirectionalPad::Right) => Some(8),
                (7, DirectionalPad::Down) => Some(4),
                (8, DirectionalPad::Left) => Some(7),
                (8, DirectionalPad::Right) => Some(9),
                (8, DirectionalPad::Down) => Some(5),
                (9, DirectionalPad::Down) => Some(6),
                (9, DirectionalPad::Left) => Some(8),
                _ => None,
            }
            .map(Keypad)
        }
    }

    impl DirectionalPad {
        fn apply(self, instruction: DirectionalPad) -> Option<DirectionalPad> {
            match (self, instruction) {
                (DirectionalPad::Up, DirectionalPad::Down) => Some(DirectionalPad::Down),
                (DirectionalPad::Up, DirectionalPad::Right) => Some(DirectionalPad::Activate),
                (DirectionalPad::Down, DirectionalPad::Up) => Some(DirectionalPad::Up),
                (DirectionalPad::Down, DirectionalPad::Left) => Some(DirectionalPad::Left),
                (DirectionalPad::Down, DirectionalPad::Right) => Some(DirectionalPad::Right),
                (DirectionalPad::Left, DirectionalPad::Right) => Some(DirectionalPad::Down),
                (DirectionalPad::Right, DirectionalPad::Left) => Some(DirectionalPad::Down),
                (DirectionalPad::Right, DirectionalPad::Up) => Some(DirectionalPad::Activate),
                (DirectionalPad::Activate, DirectionalPad::Down) => Some(DirectionalPad::Right),
                (DirectionalPad::Activate, DirectionalPad::Left) => Some(DirectionalPad::Up),
                _ => None,
            }
        }
    }

    #[test]
    fn test_dirpad_paths() {
        for i in DIR_PAD_BUTTONS {
            for j in DIR_PAD_BUTTONS {
                let paths = i.path(j);
                if i == j {
                    assert_eq!(paths.len(), 1);
                    assert_eq!(paths[0].len(), 0);
                } else {
                    for path in paths {
                        let mut loc = i;
                        for instruction in path {
                            loc = loc.apply(instruction).unwrap();
                        }
                        assert_eq!(loc, j);
                    }
                }
            }
        }
    }

    #[test]
    fn test_keypad_paths() {
        for i in 0..=10 {
            for j in 0..=10 {
                let i = Keypad(i);
                let j = Keypad(j);
                let paths = i.path(j);
                if i == j {
                    assert_eq!(paths.len(), 1);
                    assert_eq!(paths[0].len(), 0);
                } else {
                    for path in paths {
                        let mut loc = i;
                        for instruction in path {
                            loc = loc.apply(instruction).unwrap();
                        }
                        assert_eq!(loc, j);
                    }
                }
            }
        }
    }

    #[test]
    fn test_numeric_part() {
        let code = [2, 4, 7, 10].into();
        let result = numeric_part(&code);
        assert_eq!(result, 247);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
