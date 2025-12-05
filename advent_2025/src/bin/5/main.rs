use std::{collections::HashSet, time::Instant};

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();
    let mut fresh_ids = 0;

    let inputs = input.lines().collect::<Vec<&str>>();
    let mut inputs = inputs.split(|v| *v == "");
    let ranges = inputs
        .next()
        .unwrap()
        .iter()
        .map(|range| range.split_once('-').unwrap())
        .map(|nums| (nums.0.parse().unwrap(), nums.1.parse().unwrap()))
        .collect::<Vec<(u64, u64)>>();
    let ids = inputs.next().unwrap();

    for id in ids {
        let id: u64 = id.parse().unwrap();
        for (from, to) in &ranges {
            if id >= *from && id <= *to {
                // println!("fresh id {} found in range {}-{}", id, from, to);
                fresh_ids += 1;
                break;
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "{} fresh ids found in {} milliseconds",
        fresh_ids,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let inputs = input.lines().collect::<Vec<&str>>();
    let mut inputs = inputs.split(|v| *v == "");
    let ranges = inputs
        .next()
        .unwrap()
        .iter()
        .map(|range| range.split_once('-').unwrap())
        .map(|nums| (nums.0.parse().unwrap(), nums.1.parse().unwrap()))
        .collect::<Vec<(u64, u64)>>();

    let mut range_set: HashSet<(u64, u64)> = HashSet::new();

    for (from, to) in ranges {
        let mut overlapping_sets = Vec::<(u64, u64)>::new();
        for range in &range_set {
            if from >= range.0 && from <= range.1
                || to >= range.0 && to <= range.1
                || range.0 >= from && range.0 <= to
            {
                overlapping_sets.push(*range);
            }
        }
        let mut new_range = (from, to);
        for overlap in overlapping_sets {
            new_range = (new_range.0.min(overlap.0), new_range.1.max(overlap.1));
            range_set.remove(&overlap);
        }
        range_set.insert(new_range);
    }

    let fresh_ids = range_set.iter().fold(0, |acc, v| acc + (v.1 - v.0) + 1);

    let duration = start.elapsed();
    println!(
        "{} fresh ids found in {} milliseconds",
        fresh_ids,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}
