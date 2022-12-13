use std::cmp::{Ord, Ordering, PartialOrd};

fn main() {
    let mut packets = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            match Element::take(&line) {
                Ok((l, _)) => packets.push(l),
                Err(e) => panic!("{e:?}"),
            }
        }
    }

    let mut already_sorted = Vec::new();
    for (nr, pair) in packets.chunks(2).enumerate() {
        if pair[0] <= pair[1] {
            already_sorted.push(nr + 1);
        }
    }

    println!("Part 1: {}", already_sorted.iter().sum::<usize>());

    let divider_1 = Element::List(vec![Element::List(vec![Element::Int(2)])]);
    let divider_2 = Element::List(vec![Element::List(vec![Element::Int(6)])]);

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort();

    let mut divider_1_pos = 0;
    let mut divider_2_pos = 0;
    for idx in 0..packets.len() {
        if packets[idx] == divider_1 {
            divider_1_pos = idx + 1;
        }
        if packets[idx] == divider_2 {
            divider_2_pos = idx + 1;
        }
    }

    println!("Part 2: {}", divider_1_pos * divider_2_pos);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Int(u8),
    List(Vec<Element>),
}

impl Element {
    fn take(mut s: &str) -> Result<(Self, &str), ElementParseError> {
        if s.is_empty() {
            Err(ElementParseError::NotAList)
        } else if s.starts_with('[') {
            s = &s[1..];

            let mut elements = Vec::new();
            while !s.starts_with(']') {
                let element;
                (element, s) = Self::take(s)?;
                if s.starts_with(',') {
                    s = &s[1..];
                }
                elements.push(element);
            }
            s = &s[1..];

            Ok((Self::List(elements), s))
        } else {
            let mut num_text = s;
            for (i, c) in s.char_indices() {
                if !c.is_ascii_digit() {
                    (num_text, s) = s.split_at(i);
                    break;
                }
            }
            Ok((
                Self::Int(
                    num_text
                        .parse::<u8>()
                        .map_err(ElementParseError::ParseInt)?,
                ),
                s,
            ))
        }
    }
}

#[derive(Debug)]
enum ElementParseError {
    ParseInt(std::num::ParseIntError),
    NotAList,
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Int(a), Self::List(b)) => [Self::Int(*a)].as_slice().cmp(b),
            (Self::List(a), Self::Int(b)) => a.as_slice().cmp(&[Self::Int(*b)]),
        }
    }
}
