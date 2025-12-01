advent_of_code::solution!(1);

fn parse(input: &str) -> impl Iterator<Item = i32> {
    input.lines().map(|l| {
        let first_char = l.chars().next().unwrap();
        let num = l
            .trim_start_matches(char::is_alphabetic)
            .parse::<i32>()
            .unwrap();
        match first_char {
            'R' => num,
            'L' => -num,
            c => unreachable!("{c}"),
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    parse(input).fold(50, |dial, num| {
        let mut acc = dial + num;
        loop {
            if acc > 99 {
                acc -= 100;
            } else if acc < 0 {
                acc += 100;
            } else {
                break;
            }
        }
        if acc == 0 {
            count += 1;
        }
        // println!("The dial is rotated {} to point at {}.", num, acc);
        acc
    });
    Some(count)
}

struct Dial(i32);

impl Dial {
    #[allow(unused)]
    fn rotate(&mut self, clicks: i32) -> u32 {
        let mut count = 0;
        let start = self.0;
        let wierd_case = self.0 == 0 && clicks.is_negative()
            || (self.0 + clicks) % 100 == 0 && clicks.is_positive();
        self.0 += clicks;
        loop {
            if self.0 > 99 {
                self.0 -= 100;
            } else if self.0 < 0 {
                self.0 += 100;
            } else {
                if self.0 == 0 {
                    count += 1;
                }
                if wierd_case {
                    count -= 1;
                }
                break count;
            }
            count += 1;
        }
    }

    #[allow(unused)]
    fn rotate_loop(&mut self, clicks: i32) -> u32 {
        let mut count = 0;
        let dir = if clicks.is_negative() { -1 } else { 1 };
        for click in 1..=clicks.abs() {
            self.0 += dir;
            if self.0 > 99 {
                self.0 -= 100;
            } else if self.0 < 0 {
                self.0 += 100;
            }
            if self.0 == 0 {
                count += 1;
            }
        }
        count
    }

    fn rotate_fast(&mut self, clicks: i32) -> u32 {
        let start = self.0;
        self.0 = calculate_rotation(start, clicks);
        calculated_zero_crosses(start, clicks)
    }
}

fn calculated_zero_crosses(start: i32, clicks: i32) -> u32 {
    let rotated = start + clicks;
    let r = (rotated / 100).unsigned_abs();
    if rotated <= 0 && start != 0 { r + 1 } else { r }
}

fn calculate_rotation(start: i32, clicks: i32) -> i32 {
    (start + clicks).rem_euclid(100)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial(50);
    let count = parse(input).fold(0, |count, num| {
        count + dial.rotate_fast(num)
        // println!(
        // "The dial is rotated {} to point at {}. count is {count}",
        // num, dial.0
        // );
    });
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;
    proptest! {

        #[test]
        fn test_dial(y in -1000i32..1000) {
            let mut dial = Dial(0);
            let calculated_zeros = calculated_zero_crosses(0, y);
            let calculated_rotation = calculate_rotation(0, y);
            assert_eq!(dial.rotate(y), calculated_zeros, "failed with y={y}");
            assert_eq!(dial.0, calculated_rotation, "failed with y={y}");
        }
        #[test]
        fn test_dial_offset(start in 0..99, y in -1000i32..1000) {
            dbg!(start, y);
            let mut dial1 = Dial(start);
            let mut dial2 = Dial(start);
            let mut dial3 = Dial(start);
            let calculated_zeros = calculated_zero_crosses(start, y);
            let calculated_rotation = calculate_rotation(start, y);
            assert_eq!(
                dial1.rotate(y),
                calculated_zeros,
            );
            assert_eq!(
                dial1.0, calculated_rotation,
            );
            assert_eq!(
                dial2.rotate_loop(y),
                calculated_zeros,
            );
            assert_eq!(
                dial2.0, calculated_rotation,
            );
            assert_eq!(
                dial3.rotate_fast(y),
                calculated_zeros,
            );
            assert_eq!(
                dial3.0, calculated_rotation,
            );
        }

    }

    #[test]
    fn test_dial_both() {
        let mut dial1 = Dial(95);
        let mut dial2 = Dial(95);
        let clicks = 705;
        assert_eq!(dial1.rotate(clicks), dial2.rotate_loop(clicks));
        assert_eq!(dial1.0, dial2.0);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
