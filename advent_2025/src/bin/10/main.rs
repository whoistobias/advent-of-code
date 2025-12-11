use good_lp::{Expression, Solution, SolverModel, Variable, default_solver, variable, variables};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashSet, VecDeque},
    time::Instant,
};

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    // pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn process_input(input: &str) -> Vec<(usize, u16, Vec<u16>, Vec<u16>)> {
    input
        .lines()
        .map(|line| {
            let mut diagram_parts = line.split_whitespace();
            let indicator_light_diagram = diagram_parts.next().unwrap();
            let indicator_light_diagram = indicator_light_diagram
                .get(1..indicator_light_diagram.len().saturating_sub(1))
                .unwrap();
            let len = indicator_light_diagram.len();
            let indicator_light_diagram: u16 = indicator_light_diagram
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let on = match c {
                        '#' => true,
                        _ => false,
                    };
                    (on as u16) << i
                })
                .sum();
            let joltages = diagram_parts.next_back().unwrap();
            let joltages: Vec<u16> = joltages
                .get(1..joltages.len().saturating_sub(1))
                .unwrap()
                .split(',')
                .map(|v| v.parse::<u16>().unwrap())
                .collect();
            let buttons: Vec<u16> = diagram_parts
                .map(|button| {
                    button
                        .get(1..button.len().saturating_sub(1))
                        .unwrap()
                        .split(',')
                        .map(|v| 1 << v.parse::<usize>().unwrap())
                        .sum()
                })
                .collect();
            (len, indicator_light_diagram, buttons, joltages)
        })
        .collect()
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let button_wiring_diagrams = process_input(input);

    let mut button_presses = 0;

    for (_len, lights, buttons, _joltages) in button_wiring_diagrams {
        let starting_lights = 0u16;

        let result = search_for_light_combo(starting_lights, lights, &buttons);

        button_presses += result;
    }

    let duration = start.elapsed();
    println!(
        "Pressed {} buttons in {} milliseconds",
        button_presses,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn search_for_light_combo(from: u16, to: u16, possible_transformations: &Vec<u16>) -> i32 {
    // List of all states we can get to from the initial state.
    let mut open_list = VecDeque::from_iter([(from, 0)]);
    // List of all known states.
    let mut closed_list: HashSet<u16> = HashSet::new();

    // println!("Start: {:010b}", from);
    // println!("Goal: {:010b}", to);
    while let Some((current, steps)) = open_list.pop_front() {
        // println!("current:  {:010b}, steps: {}", current, steps);

        if current == to {
            // we have found the shortest path to the goal! return the path
            return steps;
        }
        if !closed_list.contains(&current) {
            closed_list.insert(current);
            let neighbors: Vec<(u16, i32)> = get_neighbors(current, &possible_transformations)
                .iter()
                .map(|v| (*v, steps + 1))
                .collect();
            open_list.extend(neighbors);
        }
    }
    panic!("Pathfinding failed");
}

fn get_neighbors(value: u16, transformations: &Vec<u16>) -> Vec<u16> {
    transformations
        .iter()
        .map(|transform| value ^ transform)
        .collect()
}

/// Gets the number of differing bits between two integers.
fn _distance(x: u16, y: u16) -> u8 {
    let mut result = 0;
    let mut value = x ^ y;
    while value > 0 {
        result += 1;
        value = value & (value - 1);
    }
    return result;
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let button_wiring_diagrams = process_input(input);

    let mut button_presses = 0;

    for (len, _lights, buttons, joltages) in button_wiring_diagrams {
        println!("{:?}", joltages);

        let starting_joltages = vec![0; len];

        let result =
            search_for_joltage_combo_using_linear_algebra(&starting_joltages, &joltages, &buttons);

        button_presses += result;
    }

    let duration = start.elapsed();
    println!(
        "Pressed {} buttons in {} milliseconds",
        button_presses,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn search_for_joltage_combo(
    from: &Vec<u16>,
    to: &Vec<u16>,
    possible_transformations: &Vec<u16>,
) -> i32 {
    // List of all states we can get to from the initial state.
    // let mut open_list = VecDeque::from_iter([(from.clone(), 0)]);
    let mut open_list = BinaryHeap::new();

    open_list.push((Reverse(0), Reverse(0), from.clone()));
    // List of all known states.
    let mut closed_list: HashSet<Vec<u16>> = HashSet::new();

    let mut iterations = 0;

    while let Some((Reverse(heuristic), Reverse(steps), current)) = open_list.pop() {
        iterations += 1;
        // println!("Checking: {}", heuristic);

        if current == *to {
            // we have found the shortest path to the goal! return the path
            println!("iterations: {}", iterations);

            return steps;
        }
        if !closed_list.contains(&current) {
            let neighbors = get_joltage_neighbors(&current, to, possible_transformations);
            // println!("{:?}", neighbors.len());

            for neighbor in neighbors {
                let heuristic = joltage_heuristic(&neighbor, &to);
                // println!("inserting");
                open_list.push((
                    Reverse(heuristic + steps as u16),
                    Reverse(steps + 1),
                    neighbor,
                ));
            }
            // println!("{:?}", open_list);
            if iterations % 10000 == 0 {
                println!("h:{} - steps:{} - {:?}", heuristic, steps, current);
            }
            // open_list.extend(neighbors);
            closed_list.insert(current);
        }
    }
    // println!("{:?}", closed_list);

    panic!("Pathfinding failed");
}

fn search_for_joltage_combo_using_linear_algebra(
    from: &Vec<u16>,
    to: &Vec<u16>,
    buttons: &Vec<u16>,
) -> u32 {
    let mut vars = variables!();

    let presses: Vec<Variable> = (0..buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();
    let total_presses: Expression = presses.iter().sum();
    let mut problem = vars.minimise(total_presses).using(default_solver);

    for (jolt_idx, &target) in to.iter().enumerate() {
        let mut expr = Expression::from(0.0);

        for (btn_idx, button) in buttons.iter().enumerate() {
            // if button is relevant, add its press variable to the constraint

            for i in 0..u16::BITS as usize {
                let bit = (button >> i) & 1;
                if bit == 1 && i == jolt_idx {
                    expr += presses[btn_idx];
                }
            }
        }
        // sum of relevant presses == target joltage
        problem.add_constraint(expr.eq(target as f64));

        // // sum of relevant presses == target joltage

        // problem.add_constraint(expr.eq(target as f64));
    }

    let solution = problem.solve().unwrap();

    presses
        .iter()
        .map(|v| solution.value(*v).round() as u32)
        .sum()
}

fn joltage_heuristic(from: &Vec<u16>, to: &Vec<u16>) -> u16 {
    from.iter()
        .zip(to)
        .map(|(f, t)| f.abs_diff(*t))
        .max()
        .unwrap_or(0)
}

fn get_joltage_neighbors(
    value: &Vec<u16>,
    destination: &Vec<u16>,
    transformations: &Vec<u16>,
) -> Vec<Vec<u16>> {
    let mut new_states = Vec::new();
    'transformy: for transform in transformations {
        {
            let mut new_value = value.clone();
            for i in 0..u16::BITS as usize {
                let bit = (transform >> i) & 1;
                if bit == 1 {
                    let v = new_value.get_mut(i).unwrap();
                    *v = *v + bit;
                    if *v > *destination.get(i).unwrap() {
                        continue 'transformy;
                    }
                }
            }
            new_states.push(new_value);
        }
    }
    new_states
}
