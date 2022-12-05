fn main() {
    let mut reading_stacks = true;
    let mut stacks_pt1: Vec<Vec<char>> = Vec::new();
    let mut stacks_pt2: Vec<Vec<char>> = Vec::new();

    'lines: for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            reading_stacks = false;
            for stack in &mut stacks_pt1 {
                stack.reverse();
            }
            stacks_pt2 = stacks_pt1.clone();
            continue;
        }

        if reading_stacks {
            let mut line_chars = line.chars();
            line_chars.nth(0);
            for (column, possible_crate) in line_chars.step_by(4).enumerate() {
                if possible_crate.is_ascii_digit() {
                    continue 'lines;
                } else if possible_crate != ' ' {
                    for _ in stacks_pt1.len()..=column {
                        stacks_pt1.push(Vec::new());
                    }
                    stacks_pt1[column].push(possible_crate);
                }
            }
        } else {
            let mut line_words = line.split(' ');
            let quantity = line_words.nth(1).unwrap().parse::<usize>().unwrap();
            let source = line_words.nth(1).unwrap().parse::<usize>().unwrap();
            let destination = line_words.nth(1).unwrap().parse::<usize>().unwrap();
            for _ in 0..quantity {
                if let Some(item) = stacks_pt1[source - 1].pop() {
                    stacks_pt1[destination - 1].push(item);
                }
            }

            let start_index = stacks_pt2[source - 1].len() - quantity;
            let items = stacks_pt2[source - 1]
                .drain(start_index..)
                .collect::<Vec<_>>();
            stacks_pt2[destination - 1].extend(items);
        }
    }

    println!(
        "Part 1: {}",
        stacks_pt1
            .iter()
            .map(|s| s.last().unwrap_or(&' '))
            .collect::<String>()
    );
    println!(
        "Part 2: {}",
        stacks_pt2
            .iter()
            .map(|s| s.last().unwrap_or(&' '))
            .collect::<String>()
    );
}
