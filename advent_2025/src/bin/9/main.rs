use std::{collections::BTreeMap, time::Instant};

use utils::Vector2i;

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    // pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn _pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let red_tiles: Vec<Vector2i> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Vector2i::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut areas = BTreeMap::new();

    for i in 0..red_tiles.len() - 1 {
        for j in i + 1..red_tiles.len() {
            let tile_a = red_tiles.get(i).unwrap();
            let tile_b = red_tiles.get(j).unwrap();

            let area = (*tile_a - *tile_b).area();
            areas.insert(area, (tile_a, tile_b));
        }
    }

    let largest_area = areas.last_key_value().unwrap().0;

    let duration = start.elapsed();
    println!(
        "largest area {} found in {} milliseconds",
        largest_area,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let red_tiles: Vec<Vector2i> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Vector2i::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut areas = BTreeMap::new();

    for i in 0..red_tiles.len() - 1 {
        for j in i + 1..red_tiles.len() {
            let tile_a = red_tiles.get(i).unwrap();
            let tile_b = red_tiles.get(j).unwrap();

            let area = (*tile_a - *tile_b).area();
            areas.insert(area, (tile_a, tile_b));
        }
    }

    let mut found_area: Option<i64> = None;

    'outer: for (size, area) in areas.iter().rev() {
        // check every area starting from the biggest.
        for a in red_tiles.iter() {
            if area_contains_point(*area, a) {
                continue 'outer;
            }

            // let b = red_tiles.get((i + 1) % red_tiles.len()).unwrap();

            // let area_a = area.0;
            // let area_b = &Vector2i {
            //     x: area.0.x,
            //     y: area.1.y,
            // };
            // let area_c = area.1;
            // let area_d = &Vector2i {
            //     x: area.1.x,
            //     y: area.0.y,
            // };

            // let area_edges = vec![
            //     (area_a, area_b),
            //     (area_b, area_c),
            //     (area_c, area_d),
            //     (area_d, area_a),
            // ];

            // for edge in area_edges {
            //     let perimiter_intersects_area = edges_intersect((a, b), edge);
            //     // println!(
            //     //     "edges_intersect with {:?}, {:?}, {:?} -> {}",
            //     //     a, b, edge, perimiter_intersects_area
            //     // );
            //     if perimiter_intersects_area {
            //         continue 'outer;
            //     }
            // }
        }
        for (i, a) in red_tiles.iter().enumerate() {
            let b = red_tiles.get((i + 1) % red_tiles.len()).unwrap();
            if area_contains_line_midpoint(*area, (a, b)) {
                continue 'outer;
            }
        }

        found_area = Some(*size);
        break;
    }

    let found_area = found_area.unwrap();
    let duration = start.elapsed();
    println!(
        "Max area {} found in {} milliseconds",
        found_area,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

pub fn orientation(a: &Vector2i, b: &Vector2i, c: &Vector2i) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

pub fn edges_intersect(ab: (&Vector2i, &Vector2i), cd: (&Vector2i, &Vector2i)) -> bool {
    let o1 = orientation(ab.0, ab.1, cd.0);
    let o2 = orientation(ab.0, ab.1, cd.1);
    let o3 = orientation(cd.0, cd.1, ab.0);
    let o4 = orientation(cd.0, cd.1, ab.1);

    o1.signum() != o2.signum()
        && o3.signum() != o4.signum()
        && o1 != 0
        && o2 != 0
        && o3 != 0
        && o4 != 0
}

pub fn area_contains_point(area: (&Vector2i, &Vector2i), point: &Vector2i) -> bool {
    let min_x = area.0.x.min(area.1.x);
    let max_x = area.0.x.max(area.1.x);
    let min_y = area.0.y.min(area.1.y);
    let max_y = area.0.y.max(area.1.y);
    point.x > min_x && point.x < max_x && point.y > min_y && point.y < max_y
}

pub fn area_contains_line_midpoint(
    area: (&Vector2i, &Vector2i),
    line: (&Vector2i, &Vector2i),
) -> bool {
    let midpoint = Vector2i {
        x: (line.0.x + line.1.x) / 2,
        y: (line.0.y + line.1.y) / 2,
    };
    area_contains_point(area, &midpoint)
}
