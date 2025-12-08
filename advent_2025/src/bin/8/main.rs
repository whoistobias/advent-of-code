use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    time::Instant,
};

use utils::Vector3i;

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    const EXTENSION_CORDS: usize = 1000;
    pt1(input, EXTENSION_CORDS)?;
    pt2(input)?;
    Ok(())
}

fn pt1(input: &str, extension_cords: usize) -> std::io::Result<()> {
    let start = Instant::now();

    let junction_boxes: Vec<Vector3i> = input
        .lines()
        .map(|line| {
            let mut v = line.split(',').map(|v| v.parse::<i64>().unwrap());
            Vector3i::new(v.next().unwrap(), v.next().unwrap(), v.next().unwrap())
        })
        .collect();

    let mut distances = BTreeMap::new();

    for i in 0..(junction_boxes.len() - 1) {
        for j in (i + 1)..(junction_boxes.len()) {
            let box_a = junction_boxes.get(i).unwrap();
            let box_b = junction_boxes.get(j).unwrap();
            let distance = box_a.distance_to_squared(box_b);
            distances.insert(distance, (box_a, box_b));
        }
    }

    let mut extension_cords_used = 0;
    let mut circuits = junction_boxes
        .iter()
        .map(|v| vec![v])
        .collect::<HashSet<Vec<&Vector3i>>>();

    for (from, to) in distances.values() {
        if extension_cords_used >= extension_cords {
            break;
        }
        extension_cords_used += 1;
        let mut circuit_a = circuits
            .iter()
            .find(|junctions| junctions.contains(from))
            .unwrap()
            .clone();
        let mut circuit_b = circuits
            .iter()
            .find(|junctions| junctions.contains(to))
            .unwrap()
            .clone();
        if circuit_a == circuit_b {
            continue;
        }
        circuits.remove(&circuit_b);
        circuits.remove(&circuit_a);
        circuit_a.append(&mut circuit_b);
        circuits.insert(circuit_a);
    }

    let mut circuits = circuits.into_iter().collect::<Vec<_>>();
    circuits.sort_by(|a, b| {
        if a.len() < b.len() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let result = circuits.iter().take(3).fold(1, |acc, v| acc * v.len());

    let duration = start.elapsed();
    println!(
        "Top 3 circuit lengths multiplied is {} found in {} milliseconds",
        result,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let junction_boxes: Vec<Vector3i> = input
        .lines()
        .map(|line| {
            let mut v = line.split(',').map(|v| v.parse::<i64>().unwrap());
            Vector3i::new(v.next().unwrap(), v.next().unwrap(), v.next().unwrap())
        })
        .collect();

    let mut distances = BTreeMap::new();

    for i in 0..(junction_boxes.len() - 1) {
        for j in (i + 1)..(junction_boxes.len()) {
            let box_a = junction_boxes.get(i).unwrap();
            let box_b = junction_boxes.get(j).unwrap();
            let distance = box_a.distance_to_squared(box_b);
            distances.insert(distance, (box_a, box_b));
        }
    }

    let mut circuits = junction_boxes
        .iter()
        .map(|v| vec![v])
        .collect::<HashSet<Vec<&Vector3i>>>();

    let mut result = 0;
    for (from, to) in distances.values() {
        let mut circuit_a = circuits
            .iter()
            .find(|junctions| junctions.contains(from))
            .unwrap()
            .clone();
        let mut circuit_b = circuits
            .iter()
            .find(|junctions| junctions.contains(to))
            .unwrap()
            .clone();
        if circuit_a == circuit_b {
            continue;
        }
        circuits.remove(&circuit_b);
        circuits.remove(&circuit_a);

        circuit_a.append(&mut circuit_b);
        circuits.insert(circuit_a);
        if circuits.len() == 1 {
            result = from.x * to.x;
            break;
        }
    }

    let mut circuits = circuits.into_iter().collect::<Vec<_>>();
    circuits.sort_by(|a, b| {
        if a.len() < b.len() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let duration = start.elapsed();
    println!(
        "Last two boxes multiplied x coordinates is {} found in {} milliseconds",
        result,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}
