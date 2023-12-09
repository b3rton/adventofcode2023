use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> io::Result<()> {
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
    let reader = io::BufReader::new(file);

    // Create our state variables
    // create a map for each range, should be 100 tuples that go (0,0), (1,1) etc
    let mut seeds: Vec<usize> = Vec::new();
    let mut seed_to_soil_map: HashMap<usize, usize> = HashMap::new();
    let mut soil_to_fertilizer_map: HashMap<usize, usize> = HashMap::new();
    let mut fertilizer_to_water_map: HashMap<usize, usize> = HashMap::new();
    let mut water_to_light_map: HashMap<usize, usize> = HashMap::new();
    let mut light_to_temperature_map: HashMap<usize, usize> = HashMap::new();
    let mut temperature_to_humidity_map: HashMap<usize, usize> = HashMap::new();
    let mut humidity_to_location_map: HashMap<usize, usize> = HashMap::new();

    // process each range set into the tuples

    // calculate out the destination for each seed
    // seed -> soil -> fert -> water -> light -> temp -> humid ->location

    // Read through the file and process it
    for line in reader.lines() {
        let line = line?;
        // println!("{:?}",line);
        let holder = process_range(line.clone());

        let mut partsSplit = line.split(':');
        let mut parts = partsSplit.map(|s| s.trim());

        println!("{:?}")

        let label = parts.next().unwrap();
        let values = parts.next().unwrap();

        match label {
            "seeds" => parse_and_insert_seeds(&mut seeds, values)?,
            "seed-to-soil map" => parse_and_insert_map(&mut seed_to_soil_map, values)?,
            "soil-to-fertilizer map" => parse_and_insert_map(&mut soil_to_fertilizer_map, values)?,
            "fertilizer-to-water map" => parse_and_insert_map(&mut fertilizer_to_water_map, values)?,
            "water-to-light map" => parse_and_insert_map(&mut water_to_light_map, values)?,
            "light-to-temperature map" => parse_and_insert_map(&mut light_to_temperature_map, values)?,
            "temperature-to-humidity map" => parse_and_insert_map(&mut temperature_to_humidity_map, values)?,
            "humidity-to-location map" => parse_and_insert_map(&mut humidity_to_location_map, values)?,
            _ => {}
        }
    }

    // print collected data
    println!("Seeds: {:?}", seeds);
    println!("Seed-to-soil map: {:?}", seed_to_soil_map);
    println!("Soil-to-fertilizer map: {:?}", soil_to_fertilizer_map);
    println!("Fertilizer-to-water map: {:?}", fertilizer_to_water_map);
    println!("Water-to-light map: {:?}", water_to_light_map);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

fn process_range(line: String) -> usize {
    println!("{}",line);
    1
}

fn parse_and_insert_seeds(seeds: &mut Vec<usize>, values: &str) -> Result<(), std::io::Error> {
    for value in values.split(' ') {
        let val: usize = value.parse().unwrap();
        seeds.push(val);
    }

    Ok(())
}

fn parse_and_insert_map(map: &mut HashMap<usize, usize>, values: &str) -> Result<(), std::io::Error> {
    for value in values.split(' ') {
        let mut parts = value.split("->");

        let from = parts.next().unwrap().parse::<usize>().unwrap();
        let to = parts.next().unwrap().parse::<usize>().unwrap();

        map.insert(from, to);
    }

    Ok(())
}