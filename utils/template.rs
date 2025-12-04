use std::time::Instant;

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    pt1(input)?;
    Ok(())
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();
    let mut joltage = 0;

    let duration = start.elapsed();
    println!(
        "Max joltage {} found in {} milliseconds",
        joltage,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();
    let mut joltage = 0;

    let duration = start.elapsed();
    println!(
        "Max joltage {} found in {} milliseconds",
        joltage,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}
