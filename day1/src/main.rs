use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fs::File;
use std::time::Instant;

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    //Open the file 
    let file = File::open("./input.txt")?;
    // Create a buffered reader
    let reader = io::BufReader::new(file);

    // Create our sum variable
    let mut sum: usize = 0;

    for line in reader.lines(){
        let line = line?;
        // println!("{:?}",line);
        let l_int = process_line(line);
        // add to sum
        sum += l_int
    }


    println!("Sum: {}", sum);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

fn process_line(line: String ) -> usize{
    let mut first_digit: char = '\0';
    let mut last_digit:char = ' ';

    for (index,lchar) in line.char_indices(){
        // find the first and last char that is a digit in the row != null for the first digit  then just overwrite the last digit repeatedly 
        let mut digit = '\0';
        if lchar.is_ascii_digit(){
            digit = lchar;
        } else if let Some(wc) = word_check(&line, index){
            digit = wc;
        }
        if digit != '\0'{
            if first_digit == '\0'{
                first_digit = digit;
            }
            last_digit = digit;
        }
    }
    // now process the char digits into the two digit # 
    let mut l_string = String::from("");
    l_string.push(first_digit);
    l_string.push(last_digit);
    let digit = l_string.parse::<usize>().unwrap();
    return digit; 
}

fn word_check(line: &String, index: usize) -> Option<char>{
    let mut res = None;
    let mut digit_mapping = HashMap::new();

    digit_mapping.insert("one", '1');
    digit_mapping.insert("two", '2');
    digit_mapping.insert("three", '3');
    digit_mapping.insert("four", '4');
    digit_mapping.insert("five", '5');
    digit_mapping.insert("six", '6');
    digit_mapping.insert("seven", '7');
    digit_mapping.insert("eight", '8');
    digit_mapping.insert("nine", '9');

    let l_sub_str = &line[index..];
    for sub in digit_mapping.keys(){
        if l_sub_str.starts_with(sub){
            res= Some(digit_mapping[sub]);
        }
    }
    res
}