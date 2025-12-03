use itertools::Itertools;

advent_of_code::solution!(3);

fn analyse_p1(bank: &[u8]) -> u8 {
    let (largest_index, largest) = bank
        .split_last()
        .unwrap()
        .1
        .iter()
        .enumerate()
        .reduce(|acc, n| if n.1 > acc.1 { n } else { acc })
        .unwrap();
    let second_largest = bank[largest_index + 1..]
        .iter()
        .reduce(|acc, n| if n > acc { n } else { acc })
        .unwrap();

    // convert from ascii numbers to ints
    (largest - 48) * 10 + (second_largest - 48)
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|l| l.as_bytes())
        .map(analyse_p1)
        .map(|n| n as u64)
        .sum();
    Some(sum)
}

fn analyse_p2(bank: &[u8]) -> u64 {
    let mut bank = bank.to_vec();
    let remove = bank.len() - 12;
    for _ in 0..remove {
        let index = bank
            .iter()
            .tuple_windows()
            .position(|(n1, n2)| n1 < n2)
            .unwrap_or(bank.len() - 1);
        bank.remove(index);
    }

    bank.iter().fold(0, |acc, n| (acc * 10) + (n - 48) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input.lines().map(|l| l.as_bytes()).map(analyse_p2).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
