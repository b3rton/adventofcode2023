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
    let mut point_sum = 0;
    // Array of the cards where the value is the # of cards below you win!, format is # of matching numbers, # of copies
    let mut cards: Vec<(usize,usize)> = vec![];
    let mut total_cards = 0;
    // For each line, process its number of cards earned equal to the 



    // Read through the file and process it
    for line in reader.lines(){
        let line = line?;
        // println!("{:?}",line);
        let (l_points,l_matches) = process_card(line);
        cards.push((l_matches,1));
        // println!("Value of Card: {}", l_points);
        point_sum += l_points;
    }

    // for it card process it adding the # of copies you have to the cards that are # of matches below
    let mut card_num = 0; 
    while card_num < cards.len(){
        // println!("Working with {} copies of card {}", cards[card_num].1, card_num+1);
        // println!("{:?}",cards);
        total_cards += cards[card_num].1;
        for i in 0..cards[card_num].0{
            // println!("adding to the quantity of cardnum + {i}");
            cards[card_num+i+1].1 += cards[card_num].1;
        }
        card_num += 1;
    }

    println!("Total Cards: {}",total_cards);
    println!("Sum: {}", point_sum);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}


fn process_card(line: String) -> (usize,usize){
    let mut value: usize = 0;
    let mut num_matches = 0;
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
            num_matches += 1;
        }
    }
    // return the value achieved
    (value,num_matches)
}