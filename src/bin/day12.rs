use std::collections::HashSet;

use rand::{prelude::SeedableRng, seq::IteratorRandom};

fn main() {
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    let mut heights = Vec::new();

    for (row, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();
        heights.push(
            line.char_indices()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = Position { x: col, y: row };
                        0
                    } else if c == 'E' {
                        end = Position { x: col, y: row };
                        25
                    } else {
                        c as u8 - 'a' as u8
                    }
                })
                .collect::<Vec<_>>(),
        );
    }

    let width = heights[0].len();
    let map = heights.iter().flatten().copied().collect::<Vec<_>>();

    let world = World {
        map: &map,
        width,
        end,
    };

    let mut data = Workspace {
        rng: rand::rngs::SmallRng::from_entropy(),
        moves: Vec::with_capacity(2048),
        positions: HashSet::with_capacity(2048),
    };

    let mut moves = usize::MAX;
    for _attempt in 0..1_000_000 {
        match world.try_climb(start, &mut data) {
            Ok(m) => moves = moves.min(m),
            Err(positions) => {
                println!("Failed: {positions}");
                return;
            }
        }
    }
    println!("Part 1: {moves}")
}

struct World<'a> {
    map: &'a [u8],
    width: usize,
    end: Position,
}

impl World<'_> {
    fn elevation(&self, p: Position) -> u8 {
        self.map[p.y * self.width + p.x]
    }

    fn directions(
        &self,
        p: Position,
        pred: impl Fn(u8) -> bool,
    ) -> impl Iterator<Item = Direction> {
        let not_at_x_min = p.x != 0;
        let not_at_y_min = p.y != 0;
        let not_at_x_max = p.x != self.width - 1;
        let not_at_y_max = p.y != self.map.len() / self.width - 1;

        (not_at_y_min && pred(self.elevation(p.apply(Direction::Up))))
            .then_some(Direction::Up)
            .into_iter()
            .chain(
                (not_at_y_max && pred(self.elevation(p.apply(Direction::Down))))
                    .then_some(Direction::Down),
            )
            .chain(
                (not_at_x_min && pred(self.elevation(p.apply(Direction::Left))))
                    .then_some(Direction::Left),
            )
            .chain(
                (not_at_x_max && pred(self.elevation(p.apply(Direction::Right))))
                    .then_some(Direction::Right),
            )
    }

    fn upward_directions(&self, p: Position) -> impl Iterator<Item = Direction> {
        let up_one = self.elevation(p) + 1;
        self.directions(p, move |v| v == up_one)
    }

    fn same_height_directions(&self, p: Position) -> impl Iterator<Item = Direction> {
        let here = self.elevation(p);
        self.directions(p, move |v| v == here)
    }

    fn downward_directions(&self, p: Position) -> impl Iterator<Item = Direction> {
        let here = self.elevation(p);
        self.directions(p, move |v| v < here)
    }

    fn try_climb(&self, mut position: Position, data: &mut Workspace) -> Result<usize, usize> {
        data.moves.clear();
        data.positions.clear();
        data.positions.insert(position);

        while position != self.end {
            let filter_fn = |d: &Direction| !data.positions.contains(&position.apply(*d));

            if let Some(dir) = self
                .upward_directions(position)
                .filter(filter_fn)
                .choose(&mut data.rng)
                .or_else(|| {
                    self.same_height_directions(position)
                        .filter(filter_fn)
                        .choose(&mut data.rng)
                })
                .or_else(|| {
                    self.downward_directions(position)
                        .filter(filter_fn)
                        .choose(&mut data.rng)
                })
            {
                data.moves.push(dir);
                position.go(dir);
                data.positions.insert(position);
            } else if let Some(dir) = data.moves.pop() {
                position.go(dir.opposite());
            } else {
                return Err(data.positions.len());
            }
        }

        Ok(data.moves.len())
    }
}

struct Workspace {
    rng: rand::rngs::SmallRng,
    moves: Vec<Direction>,
    positions: HashSet<Position>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn apply(mut self, direction: Direction) -> Self {
        self.go(direction);
        self
    }

    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}
