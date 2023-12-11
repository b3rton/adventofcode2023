use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::Instant;

/// Part two, two possible approaches?
/// go backwards finding min possible location that matches a seed 
///    While going backwards track a range of possible numbers then see if its in any of the seed ranges 
///     Process one range at a time, starting at the smallest location Range
/// Go forwards mapping ranges across the thing to find the minimum possible values 
/// 
/// Backwards approach:
/// Step 1: read the seeds into seedMap and have eval function
/// Step 2: Build a possible values map for the lowest range, need to consider how to handle the "default" map 
/// Step 3: track possible values backwards through the chain
/// Step 4: Check if any of the possible values for the lowest range exist in the seeds ranges
/// 

// Define a struct for each map entry
#[derive(Debug)]
struct RangeMap {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl FromStr for RangeMap {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = s
            .split_whitespace()
            .map(|part| part.parse().unwrap())
            .collect();
        Ok(Self {
            destination_start: parts[0],
            source_start: parts[1],
            range_length: parts[2],
        })
    }
}

impl RangeMap {
    // Function to check if a number is within the range and map it to the destination range
    fn map_value(&self, number: usize) -> Option<usize> {
        // Check if the number is within the range
        if number >= self.source_start && number < self.source_start + self.range_length {
            // Calculate the offset from the source start
            let offset = number - self.source_start;
            // Return the corresponding value in the destination range
            Some(self.destination_start + offset)
        } else {
            // Number is not within the range
            None
        }
    }
}

struct SeedRange{
    range_start: usize,
    range_length: usize
}

impl SeedRange{
    fn in_range(self, test_val: usize){

    }
}

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
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let input = fs::read_to_string(&args[1])?;

    let mut sections = input.split("\n\n");

    // Parse Seeds
    let seeds: Vec<usize> = sections
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    // Function to parse and sort map sections
    let parse_and_sort_map_section = |section: Option<&str>| -> Vec<RangeMap> {
        let mut map: Vec<RangeMap> = section.unwrap()
                                            .lines().skip(1)
                                            .map(|line| line.parse().unwrap())
                                            .collect();
        // Sorting the map by source_start
        map.sort_by_key(|k| k.source_start);
        map
    };


     // Parse each map
     let seed_to_soil_map = parse_and_sort_map_section(sections.next());
     let soil_to_fertilizer_map = parse_and_sort_map_section(sections.next());
     let fertilizer_to_water_map = parse_and_sort_map_section(sections.next());
     let water_to_light_map = parse_and_sort_map_section(sections.next());
     let light_to_temperature_map = parse_and_sort_map_section(sections.next());
     let temperature_to_humidity_map = parse_and_sort_map_section(sections.next());
     let humidity_to_location_map = parse_and_sort_map_section(sections.next());
 
     // Print each map
     println!("Seeds: {:?}", seeds);
     println!("Seed to Soil Map: {:?}", seed_to_soil_map);
     println!("Soil to Fertilizer Map: {:?}", soil_to_fertilizer_map);
     println!("Fertilizer to Water Map: {:?}", fertilizer_to_water_map);
     println!("Water to Light Map: {:?}", water_to_light_map);
     println!("Light to Temperature Map: {:?}", light_to_temperature_map);
     println!("Temperature to Humidity Map: {:?}", temperature_to_humidity_map);
     println!("Humidity to Location Map: {:?}", humidity_to_location_map);

    // Now map all of the seeds, as we map them track the minimum number
    let mut min_location: usize = usize::MAX;

    for seed in seeds{
        // map to soil
        let soil = process_through_map(&seed_to_soil_map, seed);
        // map to fertilizer
        let fertilizer = process_through_map(&soil_to_fertilizer_map, soil);
        // map to water 
        let water = process_through_map(&fertilizer_to_water_map, fertilizer);
        // map to light
        let light = process_through_map(&water_to_light_map, water);
        // map to temp
        let temp = process_through_map(&light_to_temperature_map, light);
        // map to humidity
        let humidity = process_through_map(&temperature_to_humidity_map, temp);
        // map to location
        let location = process_through_map(&humidity_to_location_map, humidity);
        if location < min_location{
            min_location = location;
        }
    }

    println!("Min Location: {}", min_location);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

fn process_range(line: String) -> usize {
    println!("{}", line);
    1
}

fn process_through_map(map: &Vec<RangeMap>, source_val: usize) -> usize{
    let mut val: usize = 0;
    for range in map{
        let in_range = range.map_value(source_val);
        match  in_range {
            Some(new_val) => val = new_val,
            None => continue
        }
        return val
    }
    if val == 0{
        return source_val
    }
    val
}