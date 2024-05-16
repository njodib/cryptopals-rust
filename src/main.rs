use std::env;

use challenges::ch01;
use challenges::ch02;
use challenges::ch03;
use challenges::ch04;
use challenges::ch05;
use challenges::ch06;
use challenges::ch07;
use challenges::ch08;
use challenges::ch09;
//use challenges::ch10;

fn main() {
    
    for ch in challenge_indices(9){
        match ch {
             1 => ch01::print(),
             2 => ch02::print(),
             3 => ch03::print(),
             4 => ch04::print(),
             5 => ch05::print(),
             6 => ch06::print(),
             7 => ch07::print(),
             8 => ch08::print(),
             9 => ch09::print(),
            //10 => ch10::print(),
            _ => panic!("\nERROR: Tried to run challenge which either doesn't exist or hasn't been implemented.\n")
        }    
    }
}

fn challenge_indices(challenges_count: usize) -> Vec<usize>{
    //get the arguments from command line
    let args = env::args();

    //first arg is calling the program, handle case of no challenges specified
    if args.len() <= 1 {
        return (1..=challenges_count).collect();
    }

    //put args into 'indices' vector, if they exist
    let mut indices = Vec::new();
    for arg in args.skip(1) {
        if let Ok(index) = arg.parse::<usize>() {
            if index >= 1 && index <= challenges_count {
                indices.push(index);
                continue;
            }
        }
        panic!("\nERROR: Tried to run challenge which either doesn't exist or hasn't been implemented.\n")
    }

    //return full indices vector
    indices
}