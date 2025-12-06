use std::time::Instant;

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let table = input
        .lines()
        .map(|row| row.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let rows = table.len();
    let cols = table.first().unwrap().len();

    let mut final_result = 0;

    for col in 0..cols {
        let operator = table.last().unwrap()[col];
        let mut result = table.first().unwrap()[col].parse::<u64>().unwrap();

        for row in 1..(rows - 1) {
            let v: u64 = table[row][col].parse().unwrap();
            match operator {
                "*" => result *= v,
                "+" => result += v,
                _ => unreachable!(),
            }
        }
        final_result += result;
    }

    let duration = start.elapsed();
    println!(
        "Math result {} found in {} milliseconds",
        final_result,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let table = input.lines();
    // .map(|row| row.split_whitespace().collect::<Vec<&str>>())
    // .collect::<Vec<Vec<&str>>>();
    let longest_line = table.clone().fold(0, |acc, v| acc.max(v.len()));
    let table = table
        .map(|line| format!("{:<longest_line$}", line).chars().collect())
        .collect::<Vec<Vec<char>>>();

    let cols = table.first().unwrap().len();
    let rows = table.len();
    // println!("{:?}", cols);

    let mut sequence: Vec<String> = Vec::new();

    for col in (0..cols).rev() {
        let mut digits = Vec::<char>::new();
        for row in 0..(rows - 1) {
            let v = table.get(row).unwrap().get(col).unwrap();
            digits.push(*v);
        }

        let v = digits.iter().collect::<String>();
        sequence.push(v);

        let operator = table.get(rows - 1).unwrap().get(col).unwrap();
        if operator != &' ' {
            sequence.push(operator.to_string());
        }
        //         // let v = table.first().unwrap()[col];
    }

    let problems = sequence.iter().map(|v| v.trim()).collect::<Vec<&str>>();
    let problems: Vec<&[&str]> = problems.split(|v| v == &"").collect();

    let mut final_result = 0;

    for problem in problems {
        let operator = problem.last().unwrap();
        let mut problem = problem.iter().take(problem.len() - 1);

        let mut result = problem.next().unwrap().parse::<u64>().unwrap();

        for part in problem {
            let v: u64 = part.parse().unwrap();
            match *operator {
                "*" => result *= v,
                "+" => result += v,
                _ => unreachable!(),
            }
        }
        final_result += result;
    }

    let duration = start.elapsed();
    println!(
        "Math result {} found in {} milliseconds",
        final_result,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}
