use std::io;

fn main() -> io::Result<()> {
    let mut elves = vec![Elf::default()];
    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            elves.push(Elf::default());
        } else {
            elves.last_mut().unwrap().calories += line.parse::<usize>().unwrap();
        }
    }

    elves.sort_by_key(|e| e.calories);

    let top_three = &elves[elves.len() - 3..];
    println!("{:#?}", top_three);
    println!();
    println!("{}", top_three.iter().map(|e| e.calories).sum::<usize>());

    Ok(())
}

#[derive(Default, Debug)]
struct Elf {
    calories: usize,
}
