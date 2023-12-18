use std::*;
use clap::Parser; 


// search for a pattern in a file and display the lines that contain it. 
#[derive(Parser)]
struct Cli {

    // the pattern to look for
    pattern : String, 

    // the path to the file to read
    path: std::path::PathBuf, 
}


fn main() {


    // The Index 0 will be the name of program
    // let pattern = std::env::args().nth(1).expect("no pattern given"); 
    // let path = std::env::args().nth(2).expect("no path given"); 

    // let args = Cli {
    //     pattern: pattern, 
    //     path: std::path::PathBuf::from(path), 
    // }; 


    // This will be getting 2 arguments by using command and it will store into struct. 
    let args = Cli::parse(); 


    // Now reading the File
    let content = std::fs::read_to_string(&args.path).expect("was unable to read File"); 

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line); 
        }
    }

    println!("The pattern is {:?} and path is {:?} ", args.pattern, args.path); 


}   
