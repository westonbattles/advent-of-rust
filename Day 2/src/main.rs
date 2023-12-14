#[macro_use(concat_string)]
extern crate concat_string;

mod game;
mod game_parser;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::env::args;

use game::Game;
use game_parser::GameParseError;

fn main() {

    let _bag_limit: [u32; 3] = [12, 13, 14];
    let mut total: u32 = 0;

    let lines = get_input_file_lines();
    for line in lines {
        
        let game_string: String = line.unwrap();
        let game: Result<Game, GameParseError> = game_string.parse();


        let _id: u8;
        let game_sets: Vec<[u32; 3]>;

        match game {
            Ok(g) => (_id, game_sets) = (g.id().to_owned(), g.sets().to_owned()),
            Err(e) => {
                println!("Error parsing game string | {{Error: {:?}}}", e);
                break;
            }
        }


        // PART 1 ANSWER

        /* 

        let mut game_is_possible = true;

        for set in &game_sets{
            let set_is_not_valid = !set.iter().zip(bag_limit.iter()).all(|(&a, &b)| a <= b);
            if set_is_not_valid {
                game_is_possible = false;
            }
        }
        
        if game_is_possible {
            total += id as u32;
        }*/

        // PART 2 ANSWER

        let mut max_set = game_sets[0];
        for set in &game_sets[1..] {
            for (&val, max) in set.iter().zip(max_set.iter_mut()){
                if val > *max {
                    *max = val;
                }
            }
        }

        total += max_set.iter().product::<u32>(); // I love rust so fucking much

    }

    println!("{}", total);

}

fn get_input_file_lines () -> Lines<BufReader<File>>{

    // Grab the first command line arg, and if none was provided, default to input.txt
    let mut filename = args().nth(1);
    if filename == None {
        println!("No input file provided, defaulting to input.txt...");
        filename = Some("input.txt".to_string());
    }

    // Grab the value of filename (Option<String>),
    // and save the result of file_line_iterator(filename)
    let filename = filename.unwrap();
    let lines_result = file_line_iterator(filename.as_str());
    
    // If the result is an error (meaning the file doesn't exist), panic 
    if lines_result.is_err() {
        panic!("{}", concat_string!("No such file '", filename, "' in root directory"));
    }

    // Unwrap the lines (won't panic because error was previously handled)
    return lines_result.unwrap();
}

fn file_line_iterator(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

