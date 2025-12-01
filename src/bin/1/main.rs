use std::{fs, time::Instant};

struct SafeDial(i32);

impl SafeDial {
    fn new() -> Self {
        return Self(50);
    }
    fn right(&mut self, clicks: i32) -> usize {
        let mut zeroes = 0;
        let mut amount = self.0 + clicks;
        while amount > 99 {
            zeroes += 1;
            amount -= 100;
        }
        self.0 = amount;
        zeroes
    }
    fn left(&mut self, clicks: i32) -> usize {
        let mut zeroes: i32 = if self.0 == 0 { -1 } else { 0 };
        let mut amount = self.0 - clicks;
        while amount < 0 {
            zeroes += 1;
            amount += 100;
        }
        if amount == 0 {
            zeroes += 1;
        }
        self.0 = amount;
        zeroes as usize
    }
    fn turn(&mut self, input: (&str, &str)) -> usize {
        let clicks = input.1.parse::<i32>().unwrap();
        match input.0 {
            "R" => self.right(clicks),
            "L" => self.left(clicks),
            _ => unreachable!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("src/bin/1/input.txt")?;

    let start = Instant::now();

    let instructions: Vec<(&str, &str)> = contents
        .trim()
        .split_whitespace()
        .map(|v| v.split_at(1))
        .collect();

    let mut safe = SafeDial::new();

    let mut zeroes = 0;
    let mut total_zeroes = 0;

    for instruction in instructions {
        let found_zeroes = safe.turn(instruction);
        total_zeroes += found_zeroes;
        if safe.0 == 0 {
            zeroes += 1;
        }
    }
    let duration = start.elapsed();

    println!(
        "Stopped on {} zeroes and passed {} zeroes in {} milliseconds",
        zeroes,
        total_zeroes,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}
