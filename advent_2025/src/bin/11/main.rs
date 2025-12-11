use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    pt1(input)?;
    pt2(input)?;
    Ok(())
}

fn pt1(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let devices: Vec<(&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let device = line.split_once(':').unwrap();
            (device.0, device.1.split_whitespace().collect())
        })
        .collect();

    let mut server_rack = HashMap::<&str, Device>::new();

    for device_record in devices.iter() {
        let device = Device::new(
            device_record.0.to_string(),
            device_record.1.iter().map(|v| v.to_string()).collect(),
        );
        server_rack.insert(device_record.0, device);
    }

    server_rack
        .entry("out")
        .or_insert(Device::new("out".to_string(), vec![]));

    let device_list: Vec<&Device> = server_rack.values().collect();

    let topologically_sorted_device_ids = topological_sort_devices(&device_list);

    let paths = count_paths(
        &topologically_sorted_device_ids,
        &server_rack,
        "you",
        "out",
        None,
    );

    let duration = start.elapsed();
    println!(
        "{} paths found in {} milliseconds",
        paths,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}

fn count_paths(
    device_ids: &Vec<String>,
    server_rack: &HashMap<&str, Device>,
    start: &str,
    end: &str,
    avoid: Option<&str>,
) -> u64 {
    let mut paths: HashMap<&str, u64> = HashMap::new();
    // let queue: VecDeque<&str> = VecDeque::new();

    paths.insert(start, 1);

    for id in device_ids {
        if let Some(avoid) = avoid
            && id == avoid
        {
            continue;
        }
        let current_paths = *paths.get(id.as_str()).unwrap_or(&0);

        for child in &server_rack.get(id.as_str()).unwrap().children {
            *paths.entry(child.as_str()).or_insert(0) += current_paths;
        }
    }

    return *paths.get(end).unwrap();
}

fn topological_sort_devices(devices: &Vec<&Device>) -> Vec<String> {
    let mut indegrees: HashMap<&str, usize> = HashMap::new();
    let mut device_map: HashMap<&str, &Device> = HashMap::new();
    let mut result: Vec<String> = Vec::new();
    let mut queue: VecDeque<&String> = VecDeque::new();

    // Start by calculating the indegrees
    for device in devices {
        device_map.insert(&device.id, device);
        for child in &device.children {
            *indegrees.entry(&child).or_insert(0) += 1;
        }
    }
    // Add all the nodes with indegree of 0 to the queue
    for device in devices {
        let indegree = indegrees.entry(&device.id).or_insert(0);
        if *indegree == 0 {
            queue.push_back(&device.id);
        }
    }

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        result.push(current.to_string());
        // println!("{:?}", device_map);
        // println!("{}", current);

        for child in &device_map.get(current.as_str()).unwrap().children {
            let child_indegrees = indegrees.entry(&child).and_modify(|v| *v -= 1).or_insert(0);
            if *child_indegrees == 0 {
                queue.push_back(child);
            }
        }
    }

    result
}

#[derive(Debug)]
struct Device {
    id: String,
    children: Vec<String>,
}

impl Device {
    fn new<'a>(id: String, children: Vec<String>) -> Device {
        Device { id, children }
    }
}

fn pt2(input: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let devices: Vec<(&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let device = line.split_once(':').unwrap();
            (device.0, device.1.split_whitespace().collect())
        })
        .collect();

    let mut server_rack = HashMap::<&str, Device>::new();

    for device_record in devices.iter() {
        let device = Device::new(
            device_record.0.to_string(),
            device_record.1.iter().map(|v| v.to_string()).collect(),
        );
        server_rack.insert(device_record.0, device);
    }

    server_rack
        .entry("out")
        .or_insert(Device::new("out".to_string(), vec![]));

    let device_list: Vec<&Device> = server_rack.values().collect();

    let topologically_sorted_device_ids = topological_sort_devices(&device_list);

    let a = "svr";
    let b = "out";
    let x = "dac";
    let y = "fft";

    let ax_y = count_paths(
        &topologically_sorted_device_ids,
        &server_rack,
        a,
        x,
        Some(y),
    );
    let xy = count_paths(&topologically_sorted_device_ids, &server_rack, x, y, None);
    let yb = count_paths(&topologically_sorted_device_ids, &server_rack, y, b, None);
    let ay_x = count_paths(
        &topologically_sorted_device_ids,
        &server_rack,
        a,
        y,
        Some(x),
    );
    let yx = count_paths(&topologically_sorted_device_ids, &server_rack, y, x, None);
    let xb = count_paths(&topologically_sorted_device_ids, &server_rack, x, b, None);

    let paths = ax_y * xy * yb + ay_x * yx * xb;

    let duration = start.elapsed();
    println!(
        "{} paths found in {} milliseconds",
        paths,
        duration.as_micros() as f64 / 1000.
    );
    Ok(())
}
