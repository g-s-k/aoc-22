use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut packet = MessageDetector::with_capacity(4);
    for (i, c) in input.char_indices() {
        packet.push(c);
        if packet.is_unique() {
            println!("Part 1: {}", i + 1);
            break;
        }
    }

    let mut message = MessageDetector::with_capacity(14);
    for (i, c) in input.char_indices() {
        message.push(c);
        if message.is_unique() {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
}

struct MessageDetector {
    capacity: usize,
    history: VecDeque<char>,
}

impl MessageDetector {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            history: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, c: char) {
        if self.history.len() == self.capacity {
            let _ = self.history.pop_front();
        }
        self.history.push_back(c);
    }

    fn is_unique(&self) -> bool {
        self.history.iter().collect::<HashSet<_>>().len() == self.capacity
    }
}
