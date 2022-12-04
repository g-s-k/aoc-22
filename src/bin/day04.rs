use std::ops::RangeInclusive;

fn main() {
    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (first_range, second_range) = line.split_once(',').unwrap();
        let first_range = parse_range(first_range);
        let second_range = parse_range(second_range);

        if ranges_nested(&first_range, &second_range) || ranges_nested(&second_range, &first_range)
        {
            part1_sum += 1;
        }

        if ranges_overlap(&first_range, &second_range)
            || ranges_overlap(&second_range, &first_range)
        {
            part2_sum += 1;
        }
    }
    println!("Part 1: {part1_sum}");
    println!("Part 2: {part2_sum}");
}

fn parse_range(r: &str) -> RangeInclusive<usize> {
    let (beginning, end) = r.split_once('-').unwrap();
    beginning.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
}

fn ranges_nested(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

fn ranges_overlap(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r1.contains(r2.start()) || r1.contains(r2.end())
}
