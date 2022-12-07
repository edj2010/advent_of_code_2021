use itertools::Itertools;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum()
}
