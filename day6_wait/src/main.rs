use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::Instant;


fn main()  -> io::Result<()> {

    let start_time = Instant::now();
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a filename was provided
    if args.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No file name provided",
        ));
    }

    //Open the file
    let file = File::open(&args[1])?;
    // Create a buffered reader
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut lines = reader.lines();

    let time_line = lines.next().unwrap()?;
    let distance_line = lines.next().unwrap()?;

    let times: Vec<usize> = time_line
        .split_whitespace()
        .skip(1) // skip the "Time:" part
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<usize> = distance_line
        .split_whitespace()
        .skip(1) // skip the "Distance:" part
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut possible_solves_vec = vec![];
    
    for (index,time) in times.into_iter().enumerate(){
        let possible_solves = race_boats(time,distances[index]);
        possible_solves_vec.push(possible_solves);
    }

    // multiply out possible solves 
    let mut answer = 1;
    for item in possible_solves_vec{
        answer = answer * item;
    }

    println!("Answer: {}", answer);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

fn race_boats(max_time:usize, record_distance:usize) -> usize{
    let mut possible_solves = 0; 

    // its a solve if the current index 
    let mut possible_hold_times: Vec<usize> = vec![];
    let mut current_hold_time = 0;
    while current_hold_time <= max_time{
        let distance = current_hold_time * (max_time-current_hold_time);
        if distance > record_distance{
            possible_hold_times.push(current_hold_time);
        }
        current_hold_time += 1;
    }
    let possible_solves = possible_hold_times.len();
    println!("Possible solves: {}",possible_solves);

    possible_solves
}