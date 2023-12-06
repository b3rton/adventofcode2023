use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fs::File;
use std::time::Instant;
use std::env;

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
    let mut sum = 0;
    // Read through the file and process it into the map
    for line in reader.lines(){
        let line = line?;
        // println!("{:?}",line);
        let l_int = process_card(line);
        println!("Value of Card: {}", l_int);
        sum += l_int;
    }

    println!("Sum: {}", sum);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}


fn process_card(line: String) -> usize{
    let mut value: usize = 0;
    // prep the card by splitting first on : then on | 
    let split1: Vec<&str> = line.split(":").collect();
    let numbers = String::from(split1[1]);
    let helpful_split: Vec<&str> = numbers.split('|').collect();
    // create list of winning numbers and our numbers
    let winning_numbers_str = String::from(helpful_split[0]);
    let winning_numbers: Vec<&str> = winning_numbers_str.split_whitespace().collect();
    let our_numbers_str = String::from(helpful_split[1]);
    let our_numbers: Vec<&str> =  our_numbers_str.split_whitespace().collect();
    // for each of our numbers see if its in winning numbers
    for number in our_numbers{
        // making the assumption no dupes 
        // if so do the thing for part 1 
        if winning_numbers.contains(&number){
            if value == 0{
                value = 1;
            }else{
                value = value * 2; 
            }
        }
    }
    // return the value achieved
    value
}