use std::env;
use std::f64;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", "Data/boston.txt");

    let contents =
        fs::read_to_string("Data/boston.txt").expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();

    let mut contents: Vec<Vec<f64>> = Vec::new();

    for line in lines {
        contents.push(
            line.split_whitespace()
                .map(|s: &str| s.parse::<f64>().unwrap())
                .collect(),
        )
    }

    print!("{}", contents[0][0]);
}
