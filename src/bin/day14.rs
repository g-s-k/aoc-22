use std::ops::RangeInclusive;

fn main() {
    let mut scan_raw = Vec::new();

    let mut x_min = usize::MAX;
    let mut x_max = 0;
    let mut y_max = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        line.split(" -> ")
            .map(|point| {
                let (x, y) = point.split_once(',').unwrap();
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();

                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_max = y_max.max(y);

                (x, y)
            })
            .reduce(|last, current| {
                scan_raw.push(Structure::from((last, current)));
                current
            });
    }

    // y_min is always 0
    let height = y_max + 1;

    let width_pt1 = x_max - x_min + 1;
    let mut scan_pt1: Vec<_> = (0..width_pt1 * height).map(|_| Matter::Air).collect();

    let left_padding = 120;
    let right_padding = 160;
    let width_pt2 = width_pt1 + left_padding + right_padding;
    let mut scan_pt2: Vec<_> = (0..width_pt2 * height).map(|_| Matter::Air).collect();

    for structure in scan_raw {
        match structure {
            Structure::Wall(x, y) => {
                for y in y {
                    scan_pt1[y * width_pt1 + x - x_min] = Matter::Rock;
                    scan_pt2[y * width_pt2 + x - x_min + left_padding] = Matter::Rock;
                }
            }
            Structure::Floor(x, y) => {
                for x in x {
                    scan_pt1[y * width_pt1 + x - x_min] = Matter::Rock;
                    scan_pt2[y * width_pt2 + x - x_min + left_padding] = Matter::Rock;
                }
            }
        }
    }

    scan_pt2.extend((0..width_pt2).map(|_| Matter::Air));
    scan_pt2.extend((0..width_pt2).map(|_| Matter::Rock));

    while !try_add_sand(&mut scan_pt1, x_min..=x_max) {}

    while !try_add_sand(&mut scan_pt2, x_min - left_padding..=x_max + right_padding)
        && scan_pt2[500 - x_min + left_padding] != Matter::Sand
    {}

    print_scan(&scan_pt1, width_pt1);
    print_scan(&scan_pt2, width_pt2);

    println!(
        "Part 1: {}",
        scan_pt1.iter().filter(|m| **m == Matter::Sand).count()
    );

    println!(
        "Part 2: {}",
        scan_pt2.iter().filter(|m| **m == Matter::Sand).count()
    );
}

enum Structure {
    Wall(usize, RangeInclusive<usize>),
    Floor(RangeInclusive<usize>, usize),
}

impl From<((usize, usize), (usize, usize))> for Structure {
    fn from((a, b): ((usize, usize), (usize, usize))) -> Self {
        if a.0 == b.0 {
            Self::Wall(a.0, a.1.min(b.1)..=a.1.max(b.1))
        } else {
            Self::Floor(a.0.min(b.0)..=a.0.max(b.0), a.1)
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
#[repr(u8)]
enum Matter {
    Air,
    Rock,
    Sand,
}

fn try_add_sand(scan: &mut [Matter], x_range: RangeInclusive<usize>) -> bool {
    let mut new_sand_cell = (500, 0);
    let width = x_range.end() - x_range.start() + 1;
    let height = scan.len() / width;
    let get_idx = |x, y| y * width + x - x_range.start();
    loop {
        let next_y = new_sand_cell.1 + 1;

        if new_sand_cell.0 == *x_range.start()
            || new_sand_cell.0 == *x_range.end()
            || next_y == height
        {
            return true;
        }

        if let Matter::Air = &scan[get_idx(new_sand_cell.0, next_y)] {
            new_sand_cell.1 = next_y;
        } else if let Matter::Air = &scan[get_idx(new_sand_cell.0 - 1, next_y)] {
            new_sand_cell.0 -= 1;
            new_sand_cell.1 = next_y;
        } else if let Matter::Air = &scan[get_idx(new_sand_cell.0 + 1, next_y)] {
            new_sand_cell.0 += 1;
            new_sand_cell.1 = next_y;
        } else {
            scan[get_idx(new_sand_cell.0, new_sand_cell.1)] = Matter::Sand;
            return false;
        }
    }
}

fn print_scan(scan: &[Matter], width: usize) {
    for idx in 0..scan.len() {
        if idx % width == 0 {
            println!();
        }
        match &scan[idx] {
            Matter::Air => print!("."),
            Matter::Rock => print!("#"),
            Matter::Sand => print!("o"),
        }
    }
    println!();
}
