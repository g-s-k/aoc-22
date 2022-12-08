use std::collections::HashSet;

fn main() {
    let mut horiz = Vec::new();
    let mut vert = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut horiz_line = Vec::new();
        for (i, c) in line.char_indices() {
            let height = c as usize - '0' as usize;
            horiz_line.push(height);
            if vert.len() <= i {
                vert.push(Vec::new());
            }
            vert[i].push(height);
        }
        horiz.push(horiz_line);
    }

    let visible = visible_from_outside(&horiz)
        .into_iter()
        .chain(
            visible_from_outside(&vert)
                .into_iter()
                .map(|(row, column)| (column, row)),
        )
        .collect::<HashSet<_>>();

    println!("Part 1: {}", visible.len());

    let max_score = (0..vert.len() - 1)
        .flat_map(|column| (0..horiz.len() - 1).map(move |row| (column, row)))
        .map(|(column, row)| score_1d(&horiz[row], column) * score_1d(&vert[column], row))
        .max()
        .unwrap_or_default();

    println!("Part 2: {max_score}");
}

fn visible_from_outside(mat: &Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    let mut visible = HashSet::new();

    let height = mat.len();

    for row in 0..height {
        let row_data = &mat[row];

        visible.insert((0, row));
        visible.insert((row_data.len() - 1, row));

        let mut fold_fn = |max, (column, &height)| {
            if height > max {
                visible.insert((column, row));
                height
            } else {
                max
            }
        };
        row_data.iter().enumerate().fold(0, &mut fold_fn);
        row_data.iter().enumerate().rev().fold(0, &mut fold_fn);
    }

    visible
}

fn score_1d(data: &[usize], location: usize) -> usize {
    let start_height = data[location];

    let left_score = (0..location)
        .rfind(|i| data[*i] >= start_height)
        .map(|i| location - i)
        .unwrap_or(location);

    let right_score = (location + 1..data.len())
        .find(|i| data[*i] >= start_height)
        .map(|i| i - location)
        .unwrap_or(data.len() - location - 1);

    left_score * right_score
}
