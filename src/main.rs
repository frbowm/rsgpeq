extern crate rand;

mod node;
pub mod point;
mod gp;
mod params;

use gp::run_gp;

use crate::ind::{Ind};
use crate::point::*;
use crate::params::Params;
mod ind;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread;
use std::sync::mpsc::{channel};

fn main() {
    let fit_data: Vec<Point> = read_lines("./bigtestdata1.txt".to_string());
    
    let p: Params = Params {
        pop_size: 1000,
        max_depth: 12,
        max_generations: 50, //-1 says to run indefinitely until user hits return key.
        tourney_size: 3,
        p_crossover: 0.90,
        p_mutate: 0.05,
        p_copy: 0.05
    };

    let (tx, rx) = channel();    
    let child = thread::spawn(move || {
        let best_of_run: Box<Ind> = run_gp(&p, &fit_data, rx);
        println!("GP finished, this is the solution found.");
        println!("best_of_gen: {}", best_of_run);
    });

    let _ = thread::spawn(move || {
        let stdin = io::stdin();
        let input = &mut String::new();
        input.clear();
        stdin.read_line(input);
        tx.send("Quit").unwrap();
        println!("User forcing a stop to this!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");    
    });

    let _ = child.join();
}

fn read_lines(filename: String) -> Vec<Point> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut fit_data: Vec<Point> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        let x: f64 = split[0].parse().unwrap();
        let y: f64 = split[1].parse().unwrap();
        fit_data.push((Point { x, y }).clone());
    }
    fit_data
}