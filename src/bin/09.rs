use itertools::Itertools;
use std::iter::{repeat, successors};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    compact(&mut map);
    let checksum = disk_checksum(&map);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    nonfragmenting_compact(&mut map);
    let checksum = disk_checksum(&map);
    Some(checksum)
}

pub type DiskMap = Vec<DiskBlock>;
pub type DiskBlock = Option<u32>;

fn parse_input(input: &str) -> DiskMap {
    let file_ids = successors(Some(0), |id| Some(id + 1)).map(Some);
    let ids = Itertools::intersperse(file_ids, None);
    let sizes = input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as usize);

    sizes
        .zip(ids)
        .flat_map(|(size, id)| repeat(id).take(size))
        .collect()
}

fn pop_until<T>(vec: &mut Vec<T>, pred: impl Fn(&T) -> bool) -> Option<T> {
    while let Some(entry) = vec.pop() {
        if pred(&entry) {
            return Some(entry);
        }
    }
    None
}

fn compact(disk_map: &mut DiskMap) {
    let mut idx = 0;

    while idx < disk_map.len() {
        if disk_map[idx].is_none() {
            let last_nonempty_block = pop_until(disk_map, |entry| entry.is_some());
            if let Some(last_nonempty_block) = last_nonempty_block {
                if idx >= disk_map.len() {
                    disk_map.push(last_nonempty_block);
                } else {
                    disk_map[idx] = last_nonempty_block;
                }
            }
        }
        idx += 1;
    }
}

fn find_next_file_reverse(
    disk_map: &DiskMap,
    mut idx: usize,
) -> Option<(Option<usize>, usize, usize, u32)> {
    while disk_map[idx].is_none() {
        if idx == 0 {
            return None;
        }
        idx -= 1;
    }

    let id = disk_map[idx].unwrap();

    let mut size = 0;
    while disk_map[idx].is_some() && disk_map[idx].unwrap() == id {
        size += 1;
        if idx == 0 {
            return Some((None, 0, size, id));
        }
        idx -= 1;
    }

    Some((Some(idx), idx + 1, size, id))
}

// There are no regions of contiguous free space of at least N
// blocks before index index[N].
type EmptySpaceSearchCache = [usize; 10];

fn update_cache(cache: &mut EmptySpaceSearchCache, size: usize, index: usize) {
    for item in cache.iter_mut().take(10).skip(size) {
        if index > *item {
            *item = index;
        }
    }
}

fn find_empty_space(
    disk_map: &DiskMap,
    cache: &mut EmptySpaceSearchCache,
    file_block: usize,
    file_size: usize,
) -> Option<usize> {
    let mut idx = cache[file_size];
    let mut size = 0;

    while idx < file_block {
        if disk_map[idx].is_none() {
            size += 1;
            if size == file_size {
                update_cache(cache, file_size, idx + 1);
                return Some(idx - file_size + 1);
            }
        } else {
            size = 0;
        }
        idx += 1;
    }

    None
}

fn nonfragmenting_compact(disk_map: &mut DiskMap) {
    let mut file_idx = disk_map.len() - 1;
    let mut cache = EmptySpaceSearchCache::default();

    while let Some((new_idx, start_block, size, file_id)) =
        find_next_file_reverse(disk_map, file_idx)
    {
        if let Some(free_start_block) = find_empty_space(disk_map, &mut cache, start_block, size) {
            for item in disk_map.iter_mut().skip(free_start_block).take(size) {
                *item = Some(file_id);
            }
            for item in disk_map.iter_mut().skip(start_block).take(size) {
                *item = None;
            }
        }

        match new_idx {
            Some(idx) => file_idx = idx,
            None => return,
        }
    }
}

fn disk_checksum(disk_map: &DiskMap) -> u64 {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(block, &entry)| entry.map(|id| block as u64 * id as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
