use std::{collections::HashMap, time::Instant};

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let shelves = ShelfWall::from_str(input);

    let rolls = shelves.count_removable_rolls();

    let duration = start.elapsed();
    println!(
        "{} removable rolls found in {} milliseconds",
        rolls,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let mut shelves = ShelfWall::from_str(input);

    let rolls = shelves.remove_as_many_rolls_as_possible();

    let duration = start.elapsed();
    println!(
        "{} removeable rolls found in {} milliseconds",
        rolls,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

#[derive(Debug)]
struct ShelfWall {
    size: Vector2i,
    shelves: HashMap<Vector2i, bool>,
}

impl ShelfWall {
    fn remove_as_many_rolls_as_possible(&mut self) -> i32 {
        let mut count = 0;
        loop {
            let rolls_removed = self.remove_rolls();
            if rolls_removed == 0 {
                return count;
            }
            count += rolls_removed;
        }
    }

    fn remove_rolls(&mut self) -> i32 {
        let mut count = 0;
        let mut rolls_to_remove: Vec<Vector2i> = vec![];

        for (coord, v) in self.shelves.iter() {
            if *v {
                let neighbors = &self.count_neighbors(coord);
                if *neighbors < 4 {
                    count += 1;
                    rolls_to_remove.push(*coord);
                }
            }
        }

        for coord in rolls_to_remove.iter() {
            self.shelves.insert(*coord, false);
        }
        count
    }

    fn count_removable_rolls(&self) -> i32 {
        let mut count = 0;
        for (coord, v) in &self.shelves {
            if *v {
                let neighbors = self.count_neighbors(coord);
                if neighbors < 4 {
                    count += 1
                }
            }
        }
        count
    }

    fn count_neighbors(&self, at: &Vector2i) -> usize {
        let mut neighbors = 0;

        let from_x = at.x.saturating_sub(1);
        let to_x = (at.x + 1).min(self.size.x);
        let from_y = at.y.saturating_sub(1);
        let to_y = (at.y + 1).min(self.size.y);

        for x in from_x..=to_x {
            for y in from_y..=to_y {
                if Vector2i::new(x, y) == *at {
                    continue;
                }
                let roll = self.shelves.get(&Vector2i::new(x, y));
                match roll {
                    Some(v) => {
                        if *v {
                            neighbors += 1
                        }
                    }
                    None => (),
                }
            }
        }
        neighbors
    }

    fn from_str(input: &str) -> Self {
        let rows = input.split_whitespace().enumerate();
        let mut max_row = 0;
        let mut max_col = 0;

        let mut shelves = HashMap::new();
        for (y, row) in rows {
            if y > max_row {
                max_row = y
            }
            for (x, char) in row.chars().enumerate() {
                if x > max_col {
                    max_col = x
                }
                let roll = char == '@';
                shelves.insert(Vector2i::new(x, y), roll);
            }
        }
        Self {
            shelves,
            size: Vector2i::new(max_col, max_row),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Vector2i {
    x: usize,
    y: usize,
}

impl Vector2i {
    fn new(x: usize, y: usize) -> Self {
        return Self { x, y };
    }
}

// pt 1
// 1495 removeable rolls found in 50.144 milliseconds
