use crate::days::{DayError, DaySolver};
use itertools::Itertools;
pub struct Day9Solver;

#[derive(Debug, Clone, PartialEq)]
struct FileMapEntry {
    file_id: u64,
    file_size: u32,
    free_space: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct FileBlock(u64);

type FileBlocks = Vec<Option<FileBlock>>;

impl FileMapEntry {
    fn to_blocks(&self) -> FileBlocks {
        let mut block: FileBlocks = (0..self.file_size)
            .map(|_| Some(FileBlock(self.file_id)))
            .collect();
        let free_space: FileBlocks = (0..self.free_space).map(|_| None).collect();
        block.extend(free_space);

        block
    }
}

fn parse_input(input: &str) -> Vec<FileMapEntry> {
    input
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .enumerate()
        .map(|(index, chunk)| {
            let file_id: u64 = index.try_into().unwrap();
            match chunk {
                [first_char, second_char] => FileMapEntry {
                    file_id,
                    file_size: first_char.to_digit(10).unwrap(),
                    free_space: second_char.to_digit(10).unwrap(),
                },
                [first_char] => FileMapEntry {
                    file_id,
                    file_size: first_char.to_digit(10).unwrap(),
                    free_space: 0,
                },
                [] => panic!("Empty chunk given"),
                &[_, _, _, ..] => todo!(),
            }
        })
        .collect()
}

fn sort_file_blocks(file_blocks: &mut FileBlocks) {
    let last_index = file_blocks.len() - 1;
    let mut current_index = last_index;

    while current_index > 0 {
        let value = file_blocks[current_index].clone();
        let (first_free_index, _) = file_blocks
            .iter()
            .find_position(|block| block.is_none())
            .unwrap();

        if first_free_index > current_index {
            break;
        }

        if value.is_some() {
            file_blocks.remove(current_index);

            file_blocks.remove(first_free_index);
            file_blocks.insert(first_free_index, value);
            file_blocks.push(None);
        }
        current_index -= 1
    }
}

fn find_free_chunk_of_size(blocks: &[Option<FileBlock>], len: usize, size: usize) -> Option<usize> {
    (0..len).find(|i| blocks[*i..].iter().take(size).all(|x| x.is_none()))
}

fn find_next_block_chunk(blocks: &[Option<FileBlock>], start: usize) -> (usize, usize) {
    let end_index = (0..start).rev().find(|i| blocks[*i].is_some()).unwrap();
    let block = blocks[end_index].clone().unwrap();
    let mut start_index = end_index;
    while start_index > 0 && blocks[start_index - 1] == Some(block.clone()) {
        start_index -= 1;
    }

    (start_index, end_index)
}

fn sort_part_2(file_blocks: &mut FileBlocks) {
    let length = file_blocks.len();
    let (mut start_index, mut end_index) = find_next_block_chunk(file_blocks, length);

    while start_index > 0 {
        let size = end_index - start_index + 1;
        if let Some(first_free_index) = find_free_chunk_of_size(&file_blocks, length, size) {
            if first_free_index < start_index {
                (start_index..=end_index)
                    .zip(first_free_index..)
                    .for_each(|(i, j)| file_blocks.swap(i, j));
            }
        }
        (start_index, end_index) = find_next_block_chunk(&file_blocks, start_index);
    }
}

fn calculate_checksum(position: usize, block: &Option<FileBlock>) -> Option<u64> {
    if let Some(FileBlock(file_id)) = block {
        let checksum_part: u64 = file_id * (position as u64);
        return Some(checksum_part);
    }

    None
}

impl DaySolver for Day9Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let mut blocks: FileBlocks = parse_input(input)
            .iter()
            .flat_map(|map_entry| map_entry.to_blocks())
            .collect();
        sort_file_blocks(&mut blocks);

        let checksum: u64 = blocks
            .iter()
            .enumerate()
            .filter_map(|(position, block)| calculate_checksum(position, block))
            .sum();

        Ok(checksum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let mut blocks: FileBlocks = parse_input(input)
            .iter()
            .flat_map(|map_entry| map_entry.to_blocks())
            .collect();
        sort_part_2(&mut blocks);

        let checksum: u64 = blocks
            .iter()
            .enumerate()
            .filter_map(|(position, block)| calculate_checksum(position, block))
            .sum();

        Ok(checksum.to_string())
    }
}
#[cfg(test)]
mod tests {
    use crate::days::day09::{
        parse_input, Day9Solver, FileBlock,  FileMapEntry,
    };
    use crate::days::DaySolver;

    fn get_example_input() -> &'static str {
        "2333133121414131402"
    }

    #[test]
    fn test_parsing() {
        let map = parse_input("12345");

        assert_eq!(
            map,
            vec![
                FileMapEntry {
                    file_id: 0,
                    file_size: 1,
                    free_space: 2
                },
                FileMapEntry {
                    file_id: 1,
                    file_size: 3,
                    free_space: 4,
                },
                FileMapEntry {
                    file_id: 2,
                    file_size: 5,
                    free_space: 0
                }
            ]
        )
    }

    #[test]
    fn test_file_map_to_block() {
        assert_eq!(
            FileMapEntry {
                file_id: 1,
                file_size: 2,
                free_space: 2
            }
            .to_blocks(),
            vec![Some(FileBlock(1)), Some(FileBlock(1)), None, None]
        )
    }

    #[test]
    fn test_solve_part_1() {
        let solution = Day9Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "1928")
    }
    #[test]
    fn test_solve_part_2() {
        let solution = Day9Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "2858")
    }
}
