fn main() {
    let mut positions_pt1 = std::collections::HashSet::new();
    let mut positions_pt2 = std::collections::HashSet::new();
    let mut rope_pt1 = MultiKnotRope {
        knots: [Position::default(); 2],
    };
    let mut rope_pt2 = MultiKnotRope {
        knots: [Position::default(); 10],
    };

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<isize>().unwrap();

        for _ in 0..distance {
            rope_pt1.step(direction);
            positions_pt1.insert(rope_pt1.knots[1]);
            rope_pt2.step(direction);
            positions_pt2.insert(rope_pt2.knots[9]);
        }
    }

    println!("Part 1: {}", positions_pt1.len());
    println!("Part 2: {}", positions_pt2.len());
}

struct MultiKnotRope<const N: usize> {
    knots: [Position; N],
}

impl<const N: usize> MultiKnotRope<N> {
    fn step(&mut self, direction: &str) {
        match direction {
            "U" => self.knots[0].y += 1,
            "D" => self.knots[0].y -= 1,
            "R" => self.knots[0].x += 1,
            "L" => self.knots[0].x -= 1,
            _ => todo!(),
        }

        for idx in 1..N {
            let prev = idx - 1;
            let lag_x = self.knots[prev].x - self.knots[idx].x;
            let lag_y = self.knots[prev].y - self.knots[idx].y;

            let is_x_ok = (-1..=1).contains(&lag_x);
            let is_y_ok = (-1..=1).contains(&lag_y);

            if is_x_ok && is_y_ok {
                // do nothing
            } else if is_x_ok {
                if lag_x != 0 {
                    self.knots[idx].x = self.knots[prev].x;
                }
                self.knots[idx].y += lag_y.signum();
            } else if is_y_ok {
                self.knots[idx].x += lag_x.signum();
                if lag_y != 0 {
                    self.knots[idx].y = self.knots[prev].y;
                }
            } else {
                self.knots[idx].x += lag_x.signum();
                self.knots[idx].y += lag_y.signum();
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}
