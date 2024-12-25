advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut pairs = 0;
    for lock in &input.locks {
        for key in &input.keys {
            if fits(lock, key) {
                pairs += 1;
            }
        }
    }

    Some(pairs)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn fits(lock: &Lock, key: &Key) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }
    true
}

type Lock = [u8; 5];
type Key = [u8; 5];

#[derive(Debug)]
struct Input {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

fn parse_input(input: &str) -> Input {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut lines = input.lines();
    loop {
        let mut this = lines.by_ref().take(7);
        let is_lock = this.next().unwrap().starts_with("#");

        let mut keylock = [0; 5];

        for line in this.by_ref().take(5) {
            for (n, c) in line.chars().enumerate() {
                if c == '#' {
                    keylock[n] += 1;
                }
            }
        }

        for _ in this {}

        if is_lock {
            locks.push(keylock);
        } else {
            keys.push(keylock);
        }

        if lines.next().is_none() {
            break;
        }
    }

    Input { locks, keys }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(input.keys.len(), 3);
        assert_eq!(input.locks.len(), 2);
        assert_eq!(input.locks[0], [0, 5, 3, 4, 3]);
        assert_eq!(input.keys[0], [5, 0, 2, 1, 3]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
