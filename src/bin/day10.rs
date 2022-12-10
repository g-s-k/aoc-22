fn main() {
    let mut program = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let instr = line.parse::<Instruction>().unwrap();
        program.push(instr);
    }

    let mut machine = Machine::default();
    machine.execute(program);

    println!("Part 1: {}", machine.accumulator);
    println!("Part 2:");
    machine.screen.render();
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            None => Ok(Self::Noop),
            Some(("addx", num)) => Ok(Self::Addx(num.parse().unwrap())),
            _ => todo!(),
        }
    }
}

struct Machine {
    reg_x: isize,
    accumulator: isize,
    cycle: usize,
    deferred: Option<(usize, Instruction)>,
    screen: Screen,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            reg_x: 1,
            accumulator: 0,
            cycle: 0,
            deferred: None,
            screen: Default::default(),
        }
    }
}

impl Machine {
    fn tick(&mut self, program: &mut Vec<Instruction>) {
        self.cycle += 1;

        if self.cycle == 20 || (self.cycle > 20 && (self.cycle - 20) % 40 == 0) {
            self.accumulator += self.cycle as isize * self.reg_x;
        }

        self.screen.tick((self.reg_x - 1)..=(self.reg_x + 1));

        match &mut self.deferred {
            None => match program.pop().unwrap() {
                Instruction::Noop => {}
                i @ Instruction::Addx(_) => {
                    self.deferred = Some((1, i));
                }
            },
            Some((1, Instruction::Addx(incr))) => {
                self.reg_x += *incr;
                self.deferred.take();
            }
            Some((ref mut cycles, _)) => {
                *cycles -= 1;
            }
        }
    }

    fn execute(&mut self, mut program: Vec<Instruction>) {
        program.reverse();
        while !program.is_empty() || self.deferred.is_some() {
            self.tick(&mut program);
        }
    }
}

struct Screen {
    pixel: usize,
    data: [bool; Self::WIDTH * Self::HEIGHT],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixel: 0,
            data: [false; Self::WIDTH * Self::HEIGHT],
        }
    }
}

impl Screen {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    fn tick(&mut self, sprite: impl std::ops::RangeBounds<isize>) {
        if sprite.contains(&((self.pixel % Self::WIDTH) as isize)) {
            self.data[self.pixel] = true;
        }
        self.pixel = (self.pixel + 1) % self.data.len();
    }

    fn render(&self) {
        println!("+{:-<1$}+", "", Self::WIDTH);
        for chunk in self.data.chunks(Self::WIDTH) {
            println!(
                "|{}|",
                chunk
                    .iter()
                    .map(|p| if *p { '#' } else { ' ' })
                    .collect::<String>()
            );
        }
        println!("+{:-<1$}+", "", Self::WIDTH);
    }
}
