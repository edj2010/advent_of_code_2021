use std::cmp::Ordering;

fn bit_one_zero_cmp(input: &Vec<&str>, bit_idx: usize) -> Ordering {
    let count = input.len();
    input
        .iter()
        .map(|s| (s.as_bytes()[bit_idx] - b'0') as usize * 2)
        .sum::<usize>()
        .cmp(&count)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let line_len = lines[0].len();
    let gamma = (0..line_len)
        .map(|idx| match bit_one_zero_cmp(&lines, idx) {
            Ordering::Greater => 1,
            _ => 0,
        })
        .fold(0, |n, d| n * 2 + d);
    let epsilon = (0..line_len)
        .map(|idx| match bit_one_zero_cmp(&lines, idx) {
            Ordering::Greater => 0,
            _ => 1,
        })
        .fold(0, |n, d| n * 2 + d);
    gamma * epsilon
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut generator: Vec<&str> = lines.clone();
    let mut scrubber: Vec<&str> = lines.clone();
    let mut generator_result: Vec<usize> = vec![];
    let mut scrubber_result: Vec<usize> = vec![];
    let line_len = lines[0].len();
    for idx in 0..line_len {
        let generator_bit = match bit_one_zero_cmp(&generator, idx) {
            Ordering::Greater | Ordering::Equal => b'1',
            Ordering::Less => b'0',
        };

        generator = generator
            .into_iter()
            .filter(|s| s.as_bytes()[idx] == generator_bit)
            .collect();
        generator_result.push((generator_bit - b'0') as usize);
    }
    for idx in 0..line_len {
        let scrubber_bit = if scrubber.len() == 1 {
            scrubber[0].as_bytes()[idx]
        } else {
            match bit_one_zero_cmp(&scrubber, idx) {
                Ordering::Greater | Ordering::Equal => b'0',
                Ordering::Less => b'1',
            }
        };

        scrubber = scrubber
            .into_iter()
            .filter(|s| s.as_bytes()[idx] == scrubber_bit)
            .collect();
        scrubber_result.push((scrubber_bit - b'0') as usize);
    }

    generator_result.into_iter().fold(0, |n, d| n * 2 + d)
        * scrubber_result.into_iter().fold(0, |n, d| n * 2 + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn part2_test() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(part2(&input), 230);
    }
}
