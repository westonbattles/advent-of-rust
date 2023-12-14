use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashMap;
use substring::Substring;

fn main() {

    // collect() command line args and put them in a list (Vector)
    let args: Vec<String> = std::env::args().collect();

    // Name of the input file
    let file_name: &str;

    // If no filename passed, default to src/input.txt
    if args.len() < 2 {
        println!("No input file provided, defaulting to input.txt...");
        file_name = "input.txt";
    } else {
        file_name = &args[1];
    }
    


    let mut number_map : HashMap<&str, i8> = HashMap::with_capacity(10);
    number_map.insert("zero", 0);
    number_map.insert("one", 1);
    number_map.insert("two", 2);
    number_map.insert("three", 3);
    number_map.insert("four", 4);
    number_map.insert("five", 5);
    number_map.insert("six", 6);
    number_map.insert("seven", 7);
    number_map.insert("eight", 8);
    number_map.insert("nine", 9);

    let mut total: u32 = 0;

    // Iterate through all lines in given file, and print them
    for line_result in read_file_line_by_line(&file_name){
        
        let line: String = line_result.unwrap();

        let mut first: bool = false;
        let mut last: i8 = -1;

        for character_tuple in line.char_indices(){

            let index: usize = character_tuple.0;
            let character: char = character_tuple.1;

            if character.is_numeric() {
                if first == false {
                    total += 10 * (character as u32 - 48);
                    first = true;
                }
                last = character as i8 - 48;
            } else {
                let slices= [line.substring(index, index+3), 
                                        line.substring(index, index+4), 
                                        line.substring(index, index+5)];

                for slice in slices{
                    if let Some(slice_number) = number_map.get(slice) {
                        let number = *slice_number;
                        if first == false {
                            total += 10 * (number as u32);
                            first = true;
                        }
                        last = number;
                    }
                }
            }
            
        }

        if last < 0 {
            panic!("Each line must contain at least two numbers!");
        }
        total += last as u32;
        
    }

    println!("{}", total);
}

fn read_file_line_by_line(file_name: &str) -> Lines<BufReader<File>> {

    // Attempt to open the file and panic if attempt failed

    // Note: File::open has it's own panic message but that one doesn't include 
    // the name/path of the file, so I decided to be cooler than them
    let file = File::open(file_name).expect(&["File: '", file_name, "'"].join(""));
    
    // Make a new BufReader instance with the file and output the lines iterator
    let reader = BufReader::new(file);
    return reader.lines();
}