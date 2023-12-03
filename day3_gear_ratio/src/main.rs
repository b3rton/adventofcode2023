use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use std::collections::HashSet;

/// part 1
/// So its a 2d vector
/// [row][column]
/// adjacent means from any character +1 -1 and 0 in each row/column.
/// So we first create the whole vector space. Then for each character we say "are you a digit"
/// if you are then we start creating a # while tracking if its a "part number" or not by seeing
/// if any of the digits that make up the number are adjacent to a symbol
/// When we come to a character that isn't a digit we see if we were tracking a digit. If yes and
/// that digit was a part number we parse it into a usize and add it to sum

/// Part 2 
/// need to find adjacent numbers to *s 
/// this means checking if there is a digit in each row that connect 
/// make gear map with arrays of digits that touch the gear, then loop through the map for gears with only 2 part numbers then sum those 


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
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let chars = line.chars().collect();
        grid.push(chars);
    }

    let (part_sum,gear_sum) = parts_sums(grid);



    println!("Part Sum: {}", part_sum);
    println!("Gear Sum: {}", gear_sum);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}


// returns (is_part,is_gear)
fn check_if_part_number(grid: &Vec<Vec<char>>, origin_row: usize, origin_char: usize) -> (bool,bool,HashSet<(usize,usize)>) {
    // Relative positions to check (including diagonals)
    let positions = [
        (-1, -1), (-1, 0), (-1, 1),  // above
        (0, -1),           (0, 1),    // sides
        (1, -1),  (1, 0),  (1, 1),    // below
    ];
    let mut is_part = false;
    let mut is_gear = false; 
    let mut gear_set = HashSet::new();

    for &(row_offset, col_offset) in &positions {
        let new_row = origin_row as isize + row_offset;
        let new_col = origin_char as isize + col_offset;
        // Check bounds
        if new_row >= 0 && new_row < grid.len() as isize && new_col >= 0 && new_col < grid[new_row as usize].len() as isize
        {
            
            // Check if character is punctuation
            if grid[new_row as usize][new_col as usize].is_ascii_punctuation() && grid[new_row as usize][new_col as usize] != '.' {
                let val = grid[new_row as usize][new_col as usize];
                // print!("There is punctuation at {new_row},{new_col},{val}");
                if val == '*'{
                    is_gear = true;
                    gear_set.insert((new_row as usize,new_col as usize));
                }
                is_part = true;
            }
        }
    }
    (is_part,is_gear,gear_set)
}

/// Returns (parts_sum,gear_sum)
fn parts_sums(grid: Vec<Vec<char>>) -> (usize,usize) {
    // looping vars
    let mut active_number = false;
    let mut is_part_number = false;
    let mut gear_list: HashSet<(usize,usize)> = HashSet::new();
    let mut sum: usize = 0;
    let mut is_gear = false;
    let mut gear_sum = 0;
    // gear is k: gear location v: list of touching #s
    let mut gear_map: HashMap<(usize,usize),Vec<usize>> = HashMap::new(); 

    for (row_index, row) in grid.iter().enumerate() {
        let mut active_num_string = String::from("");

        for (char_index, char) in row.iter().enumerate() {
            if char.is_ascii_digit() {
                active_num_string.push(*char);
                if !active_number {
                    active_number = true;
                }
                if !is_part_number || !is_gear { 
                    let part_check = check_if_part_number(&grid, row_index, char_index);
                    is_part_number = part_check.0;
                    is_gear = part_check.1;
                    for gear in part_check.2{
                        gear_list.insert(gear);       
                    }
                }
            } else {
                if active_number && is_part_number {
                    let parsed_number: usize = active_num_string.parse().unwrap();
                    // println!("Importing parsed number: {}", parsed_number);
                    sum += parsed_number;
                    if is_gear{
                        for gear in &gear_list{
                            // if key doesnt exist
                            if gear_map.contains_key(&gear){
                                gear_map.get_mut(&gear).map(|val| val.push(parsed_number));
                            }else{
                                gear_map.insert(*gear, vec![parsed_number]);
                            }
                        }
                    }
                }
                // now clean up the vars since this is the end of a number
                active_num_string = String::from("");
                active_number = false;
                is_part_number = false;
                is_gear = false;
                gear_list.clear();
            }
        }
        // clean up if digit ends line
        if active_number && is_part_number {
            let parsed_number: usize = active_num_string.parse().unwrap();
            // println!("Importing parsed number: {}", parsed_number);
            sum += parsed_number;
            if is_gear{
                for gear in &gear_list{
                    // if key doesnt exist
                    if gear_map.contains_key(&gear){
                        gear_map.get_mut(&gear).map(|val| val.push(parsed_number));
                    }else{
                        gear_map.insert(*gear, vec![parsed_number]);
                    }
                }
            }
        }
        // now clean up the vars since this is the end of a number
        active_number = false;
        is_part_number = false;
        is_gear = false;
        gear_list.clear();
    }
    // println!("Gear Map {:?}", gear_map);
    // now we have a map of gears with corresponding parts 
    for (_gear_loc, parts) in gear_map{
        if parts.len() == 2{
            gear_sum += parts[0] * parts[1];
        }
    }

    (sum,gear_sum) 
}
