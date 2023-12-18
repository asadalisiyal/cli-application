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
    let pattern = std::env::args().nth(1).expect("no pattern given"); 
    let path = std::env::args().nth(2).expect("no path given"); 

    let args = Cli {
        pattern: pattern, 
        path: std::path::PathBuf::from(path), 
    }; 



    println!("The pattern is {:?} and path is {:?} ", args.pattern, args.path); 



}   
