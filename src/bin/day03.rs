fn main() {
    let mut pt1_sum = 0;
    let mut pt2_sum = 0;

    let mut group_common = Vec::new();
    let mut line_chars = Vec::new();

    for (idx, line) in std::io::stdin().lines().enumerate() {
        line_chars.clear();
        line_chars.extend(line.unwrap().chars());

        let halfway = line_chars.len() / 2;
        let (first_half, second_half) = line_chars.split_at(halfway);
        let duplicate = second_half.iter().find(|c| first_half.contains(c)).unwrap();
        let priority = get_priority(duplicate);
        pt1_sum += priority;

        if idx % 3 == 0 {
            group_common.clear();
            group_common.extend(line_chars.iter());
        } else {
            group_common.retain(|c| line_chars.contains(c));
        }

        if idx % 3 == 2 {
            let badge = group_common.drain(..).next().unwrap();
            let badge_priority = get_priority(&badge);
            pt2_sum += badge_priority;
        }
    }

    println!("Part 1: {pt1_sum}");
    println!("Part 2: {pt2_sum}");
}

fn get_priority(c: &char) -> usize {
    match c {
        'a'..='z' => *c as usize - 'a' as usize + 1,
        'A'..='Z' => *c as usize - 'A' as usize + 27,
        _ => todo!(),
    }
}
