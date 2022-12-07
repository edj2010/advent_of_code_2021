fn parse(input: &str) -> impl Iterator<Item = (&str, isize)> {
    input.lines().map(|s| {
        let (a, b) = s.split_once(" ").unwrap();
        (a, b.parse::<isize>().unwrap())
    })
}

#[allow(dead_code)]
pub fn part1(input: &str) -> isize {
    let (a, b) = parse(input).fold((0, 0), |(distance, depth), (instr, n)| match instr {
        "forward" => (distance + n, depth),
        "down" => (distance, depth + n),
        "up" => (distance, depth - n),
        _ => panic!("unexpected instruction {}", instr),
    });
    a * b
}

#[allow(dead_code)]
pub fn part2(input: &str) -> isize {
    let (a, b, _) = parse(input).fold(
        (0, 0, 0),
        |(distance, depth, aim), (instr, n)| match instr {
            "forward" => (distance + n, depth + aim * n, aim),
            "down" => (distance, depth, aim + n),
            "up" => (distance, depth, aim - n),
            _ => panic!("unexpected instruction {}", instr),
        },
    );
    a * b
}
