use std::time::Instant;

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();
    let mut splits = 0;

    let rows: Vec<&str> = input.lines().collect();

    let mut beam_state: Vec<bool> = rows
        .first()
        .unwrap()
        .chars()
        .map(|c| match c {
            'S' => true,
            _ => false,
        })
        .collect();
    for row in rows.iter() {
        let mut new_beam_state = beam_state.clone();
        for (i, char) in row.chars().enumerate() {
            match char {
                '^' => {
                    if *beam_state.get(i).unwrap() {
                        splits += 1;

                        *new_beam_state.get_mut(i).unwrap() = false;
                        *new_beam_state.get_mut(i + 1).unwrap() = true;
                        *new_beam_state.get_mut(i - 1).unwrap() = true;
                    }
                }
                _ => (),
            }
        }
        beam_state = new_beam_state;
    }

    let duration = start.elapsed();
    println!(
        "{} beam splits found in {} milliseconds",
        splits,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let rows: Vec<&str> = input.lines().collect();

    let mut beam_state: Vec<(bool, usize)> = rows
        .first()
        .unwrap()
        .chars()
        .map(|c| match c {
            'S' => (true, 1),
            _ => (false, 0),
        })
        .collect();
    for row in rows.iter() {
        let mut new_beam_state = beam_state.clone();
        for (i, char) in row.chars().enumerate() {
            match char {
                '^' => {
                    if beam_state.get(i).unwrap().0 {
                        let current = *new_beam_state.get(i).unwrap();

                        let left = new_beam_state.get_mut(i - 1).unwrap();
                        *left = (true, left.1 + current.1);

                        let right = new_beam_state.get_mut(i + 1).unwrap();
                        *right = (true, right.1 + current.1);

                        *new_beam_state.get_mut(i).unwrap() = (false, 0);
                    }
                }
                _ => (),
            }
        }
        beam_state = new_beam_state;
    }
    let timelines = beam_state
        .iter()
        .fold(0, |acc, v| acc + if v.0 { v.1 } else { 0 });

    let duration = start.elapsed();
    println!(
        "{} timelines found in {} milliseconds",
        timelines,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}
