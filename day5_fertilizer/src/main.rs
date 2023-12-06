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
    // create a map for each range, should be 100 tuples that go (0,0), (1,1) etc 

    // process each range set into the tuples 

    // calculate out the destination for each seed
    // seed -> soil -> fert -> water -> light -> temp -> humid ->location

    // Read through the file and process it
    for line in reader.lines(){
        let line = line?;
        // println!("{:?}",line);
        let holder = process_range(line);
    }



    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

fn process_range(line: String) -> usize{
    1
}