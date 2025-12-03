use std::{fs, time::Instant};

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let start = Instant::now();

    let id_ranges = contents.trim().split(",").map(|range| {
        let (start, end) = range.split_once("-").unwrap();
        let result: (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());
        result
    });

    let mut mirrored_total = 0;
    let mut patterned_total = 0;

    for (from, to) in id_ranges {
        for i in from..=to {
            let string = i.to_string();

            let length = (i.ilog10() + 1) as u64;

            if length == 1 {
                continue;
            }

            if length % 2 == 0 {
                let (left, right) = string.split_at((length / 2) as usize);
                if left == right {
                    mirrored_total += i;
                }
            }

            let factors = length.factor();

            for factor in factors {
                let (pattern, rest) = string.split_at(factor);

                if rest.chunk(factor).all(|chunk| pattern == chunk) {
                    patterned_total += i;
                    break;
                }
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "Mirrored ids totaled to {} and all patterned ids totaled to {} in {} milliseconds",
        mirrored_total,
        patterned_total,
        duration.as_micros() as f64 / 1000.
    );

    Ok(())
}

trait Chunkable {
    type Chunk<'a>
    where
        Self: 'a;

    fn chunk<'a>(&'a self, size: usize) -> Chunks<'a, Self>;
}

struct Chunks<'a, T: ?Sized> {
    value: &'a T,
    size: usize,
    position: usize,
}

impl<'a, T: ?Sized> Chunks<'a, T> {
    fn new(value: &'a T, size: usize) -> Self {
        Chunks {
            size,
            value,
            position: 0,
        }
    }
}

impl Chunkable for str {
    type Chunk<'a>
        = &'a str
    where
        Self: 'a;

    fn chunk<'a>(&'a self, size: usize) -> Chunks<'a, Self> {
        Chunks::new(self, size)
    }
}

impl<'a> Iterator for Chunks<'a, str> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.value.len();
        let from = self.position;
        if from == len {
            return None;
        };
        let to = (self.position + self.size).min(self.value.len());
        self.position = to;
        Some(&self.value[from..to])
    }
}

trait Factor {
    fn factor(&self) -> Vec<usize>;
}

impl Factor for u64 {
    fn factor(&self) -> Vec<usize> {
        let mut factors: Vec<usize> = Vec::new();
        let mut j: usize = 1;
        let product = *self as usize;

        while j * j <= product {
            if product % j == 0 {
                factors.push(j);
                if j * j != product && j != 1 {
                    factors.push(product / j);
                }
            }
            j += 1;
        }
        factors
    }
}
