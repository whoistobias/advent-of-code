use std::time::Instant;

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt").trim();
    pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let banks = input.split_whitespace();
    let mut joltage = 0;

    for bank in banks {
        joltage += get_battery_bank_joltage(bank, 2);
    }

    let duration = start.elapsed();
    println!(
        "Max joltage with two batteries {} found in {} milliseconds",
        joltage,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let banks = input.split_whitespace();
    let mut joltage = 0;

    for bank in banks {
        joltage += get_battery_bank_joltage(bank, 12);
    }

    let duration = start.elapsed();
    println!(
        "Max joltage with twelve batteries {} found in {} milliseconds",
        joltage,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn get_battery_bank_joltage(bank: &str, batteries_to_use: usize) -> u64 {
    let bank_size = bank.len();
    let batteries = bank.chars();

    let mut picked_batteries: Vec<(usize, char)> = Vec::with_capacity(batteries_to_use);

    let batteries_list: Vec<(usize, char)> = batteries.clone().enumerate().collect();

    let current_battery = batteries
        .enumerate()
        .take(bank_size - (batteries_to_use - 1))
        .reduce(|acc, v| if acc.1 >= v.1 { acc } else { v })
        .unwrap();

    picked_batteries.push(current_battery);

    while picked_batteries.len() < batteries_to_use {
        let last_battery_index = picked_batteries.last().unwrap().0;

        let eligible_batteries =
            bank_size - last_battery_index - (batteries_to_use - picked_batteries.len());

        let from = last_battery_index + 1;
        let to = from + eligible_batteries;

        let mut best_battery = batteries_list[from];
        for i in (from + 1)..to {
            let current_battery = batteries_list[i];
            if current_battery.1 > best_battery.1 {
                best_battery = current_battery
            }
        }
        picked_batteries.push(best_battery);
    }

    let result: String = picked_batteries.iter().map(|v| v.1).collect();
    result.parse().unwrap()
}
