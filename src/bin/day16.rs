use std::collections::HashMap;

fn main() {
    let mut valves = HashMap::new();
    let mut tunnels = HashMap::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line = line.strip_prefix("Valve ").unwrap();
        let (name, line) = line.split_once(' ').unwrap();
        let line = line.strip_prefix("has flow rate=").unwrap();
        let (flow_rate, line) = line.split_once(';').unwrap();

        valves.insert(
            name.to_string(),
            Valve {
                flow_rate: flow_rate.parse().unwrap(),
                open: false,
            },
        );

        let line = line
            .strip_prefix(" tunnels lead to valves ")
            .or_else(|| line.strip_prefix(" tunnel leads to valve "))
            .unwrap();
        tunnels.insert(
            name.to_string(),
            line.split(", ").map(String::from).collect::<Vec<_>>(),
        );
    }

    let mut location = "AA".to_string();
    let mut opening = false;
    let mut moving: Option<&String> = None;
    let mut pressure_released = 0;
    let mut current_flow_rate = 0;

    for minute in 0..30 {
        pressure_released += current_flow_rate;
        println!("start minute {minute} at {location} flow rate {current_flow_rate}");

        if let Some(next) = moving.take() {
            location = next.to_string();
        }

        let this_valve = &valves[&location];

        if opening {
            println!("\topen valve {location}");
            current_flow_rate += this_valve.flow_rate;
            valves.get_mut(&location).unwrap().open = true;
            opening = false;
        } else if !this_valve.open
            && this_valve.flow_rate
                > get_neighbors(&location, &valves, &tunnels)
                    .map(|(_, rate)| rate)
                    .max()
                    .unwrap_or_default()
        {
            opening = true;
            continue;
        }

        moving = get_neighbors(&location, &valves, &tunnels)
            .max_by_key(|(n, rate)| {
                rate + get_neighbors(n, &valves, &tunnels)
                    .map(|(_, r)| r)
                    .max()
                    .unwrap_or_default()
            })
            .map(|(name, _)| name);
    }

    println!("Part 1: {pressure_released}");
}

struct Valve {
    flow_rate: usize,
    open: bool,
}

fn get_neighbors<'a>(
    location: impl AsRef<str>,
    valves: &'a HashMap<String, Valve>,
    tunnels: &'a HashMap<String, Vec<String>>,
) -> impl Iterator<Item = (&'a String, usize)> + 'a {
    tunnels[location.as_ref()]
        .iter()
        .filter_map(|n| (!valves[n].open).then_some((n, valves[n].flow_rate)))
}
