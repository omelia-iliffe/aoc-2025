advent_of_code::solution!(2);

fn parse(input: &str) -> impl Iterator<Item = (i64, i64)> {
    input.split(",").map(|s| {
        let (n1, n2) = s.trim().split_once("-").unwrap();
        (n1.parse().unwrap(), n2.parse().unwrap())
    })
}

fn analyse_number_p1(num: &i64) -> bool {
    let num_str = num.to_string();
    if num_str.len() % 2 == 1 {
        return false;
    }

    let (s1, s2) = num_str.split_at_checked(num_str.len() / 2).unwrap();
    s1 == s2
}

fn analyse_number_p2(num: &i64) -> bool {
    let num_str = num.to_string();
    for (index, _) in num_str.char_indices() {
        let (rep, remainder) = num_str.split_at_checked(index).unwrap();
        if remainder.trim_start_matches(rep).is_empty() {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let count: i64 = parse(input)
        .flat_map(|(start, end)| {
            (start..=end).filter(analyse_number_p1)
            // .inspect(|n| println!("{n}"))
        })
        .sum();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let count: i64 = parse(input)
        .flat_map(|(start, end)| {
            (start..=end).filter(analyse_number_p2)
            // .inspect(|n| println!("{n}"))
        })
        .sum();
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
