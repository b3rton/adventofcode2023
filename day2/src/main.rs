use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fs::File;
use std::time::Instant;


#[derive(Debug)]
struct CubeSet{
    red:usize,
    blue:usize,
    green:usize
}

impl CubeSet{
    fn validate_set(
        &self, 
        red_constraint: usize, 
        blue_constraint: usize,
        green_constraint:usize
    ) -> bool {
        self.red <= red_constraint && self.blue <= blue_constraint && self.green <= green_constraint
    }

    fn new() -> CubeSet{
        CubeSet{
            red:0, 
            blue:0, 
            green:0
        }
    }

    fn power(&self) -> usize{
        self.red * self.blue * self.green
    }
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    //Open the file 
    let file = File::open("./input.txt")?;
    // Create a buffered reader
    let reader = io::BufReader::new(file);

    // Create our state variables
    let mut game_map: HashMap<usize,Vec<CubeSet>> = HashMap::new();
    let mut valid_game_sum: usize = 0;
    let mut game_power_sum: usize = 0;
    let red_constraint = 12; 
    let green_constraint = 13; 
    let blue_constraint = 14; 

    // Read through the file and process it into the map
    for line in reader.lines(){
        let line = line?;
        // println!("{:?}",line);
        let l_int = process_line(&line, &mut game_map);
    }

    for game in game_map{
        let valid_game = !(game.1.iter().map(|cube_set| cube_set.validate_set(red_constraint, blue_constraint, green_constraint)).collect::<Vec<bool>>().contains(&false));
        if valid_game{
            valid_game_sum+= game.0;
        }

        let mut game_set = CubeSet::new();
        for set  in game.1{
            if set.red > game_set.red{
                game_set.red = set.red;
            }
            if set.blue > game_set.blue{
                game_set.blue = set.blue;
            }
            if set.green > game_set.green{
                game_set.green = set.green;
            }
        }
        game_power_sum += game_set.power();
    }


    println!("Valid Game Sum: {}", valid_game_sum);
    println!("Power Sum: {}", game_power_sum);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

fn process_line(line: &String, game_map: &mut HashMap<usize,Vec<CubeSet>> ){
    let mut game_sets = Vec::new();
    let mut game_id: usize = 0; 

    // lets go get game ID first
    let first_split: Vec<&str> = line.split(':').collect();
    game_id = String::from(first_split[0]).split(' ').collect::<Vec<&str>>()[1].parse().unwrap();

    // now lets build out its cube sets
    for game in String::from(first_split[1]).split(';'){
        let mut cube_set = CubeSet::new();
        let game_string = String::from(game);

        for cube in game_string.split(','){
            // need to use a let here to make the string live longer in scope
            let cube_string = String::from(cube);
            let cube_split: Vec<&str> = cube_string.split_whitespace().collect();

            match cube_split[1]{
                "red" => cube_set.red = cube_split[0].parse().unwrap(),
                "blue" => cube_set.blue = cube_split[0].parse().unwrap(),
                "green" => cube_set.green = cube_split[0].parse().unwrap(),
                _ => panic!()
            }
        }
        game_sets.push(cube_set);   
    }
    game_map.insert(game_id, game_sets);
}