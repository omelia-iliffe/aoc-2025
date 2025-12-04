use std::iter::repeat;

use advent_of_code::{cood_2_index, index_2_cood};

advent_of_code::solution!(4);

fn surrounding_indexes(index: usize, line_len: usize) -> impl Iterator<Item = usize> {
    let (x, y) = index_2_cood(index, line_len);
    (-1..=1).flat_map(move |dy| {
        (-1..=1)
            .zip(repeat(dy))
            .filter(|(dx, dy)| !(dx == &0 && dy == &0))
            .flat_map(move |(dx, dy)| {
                (x.checked_add_signed(dx).zip(y.checked_add_signed(dy)))
                    .map(|c| cood_2_index(c, line_len))
            })
    })
}

fn is_surrounded(index: usize, line_len: usize, input: &[u8]) -> bool {
    surrounding_indexes(index, line_len)
        .filter(|index| input.get(*index).is_some_and(|c| c == &b'@'))
        .count()
        < 4
}
fn remove_rolls_p1(input: &[u8], line_len: usize) -> (usize, Vec<u8>) {
    let mut output = input.to_vec();
    let count = input
        .iter()
        .enumerate()
        .filter_map(|(index, c)| (c == &b'@').then_some(index))
        .filter(|index| is_surrounded(*index, line_len, input))
        .inspect(|index| output[*index] = b'x')
        .count();
    (count, output)
}

pub fn part_one(input: &str) -> Option<u64> {
    let line_len = input.find("\n").unwrap() + 1;
    let input = input.as_bytes();

    let (count, _output) = remove_rolls_p1(input, line_len);

    // println!("{}", String::from_utf8(_output).unwrap());

    Some(count as u64)
}

fn remove_rolls_p2(input: &mut [u8], line_len: usize) -> usize {
    let mut count = 0;
    for index in 0..input.len() {
        if input[index] != b'@' {
            continue;
        }
        if is_surrounded(index, line_len, input) {
            input[index] = b'x';
            count += 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let line_len = input.find("\n").unwrap() + 1;
    let mut input = input.to_string();

    let mut total = 0;
    loop {
        let count = remove_rolls_p2(unsafe { input.as_bytes_mut() }, line_len);
        // println!("{count}\n{}", input);
        if count == 0 {
            break;
        }
        total += count
    }
    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(index_2_cood(20, 11), (9, 1));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
