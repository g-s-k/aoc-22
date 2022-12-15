fn main() {
    let mut args = std::env::args();
    let row_query = args.nth(1).unwrap().parse::<isize>().unwrap();
    let max_box = args.next().unwrap().parse::<isize>().unwrap();

    let mut sensors = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let line = line.strip_prefix("Sensor at x=").unwrap();
        let (sensor_x, line) = line.split_once(',').unwrap();
        let line = line.strip_prefix(" y=").unwrap();
        let (sensor_y, line) = line.split_once(':').unwrap();
        let line = line.strip_prefix(" closest beacon is at x=").unwrap();
        let (beacon_x, line) = line.split_once(',').unwrap();
        let beacon_y = line.strip_prefix(" y=").unwrap();

        let sensor_position = Position {
            x: sensor_x.parse().unwrap(),
            y: sensor_y.parse().unwrap(),
        };
        let beacon_position = Position {
            x: beacon_x.parse().unwrap(),
            y: beacon_y.parse().unwrap(),
        };

        let distance = sensor_position.manhattan(&beacon_position);

        sensors.push(Sensor {
            position: sensor_position,
            distance,
        });
    }

    let check_range = -1_000_000..=10_000_000;
    let mut excluded = 0;
    let mut x = *check_range.start();

    while check_range.contains(&x) {
        let p = Position { x, y: row_query };

        if let Some(sensor) = sensors
            .iter()
            .find(|s| s.position.manhattan(&p) <= s.distance)
        {
            let relative_x = sensor.position.x - p.x;
            let relative_y = (sensor.position.y - p.y).abs();
            let remaining_exclusion = sensor.distance as isize + relative_x - relative_y;

            excluded += remaining_exclusion + 1;
            x += remaining_exclusion;
        }

        x += 1;
    }

    println!("Part 1: {}", excluded - 1);

    let xy_box = 0..=max_box;
    let mut position = Position { x: 0, y: 0 };

    'outer: for y in xy_box.clone() {
        let mut x = *xy_box.start();
        'x_range: while xy_box.contains(&x) {
            let p = Position { x, y };
            for sensor in &sensors {
                if sensor.position.manhattan(&p) <= sensor.distance {
                    x += sensor.distance as isize + (sensor.position.x - x)
                        - (sensor.position.y - y).abs()
                        + 1;
                    continue 'x_range;
                }
            }
            position = p;
            break 'outer;
        }
    }

    println!("Part 2: {}", position.x * 4_000_000 + position.y);
}

struct Sensor {
    position: Position,
    distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn manhattan(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
