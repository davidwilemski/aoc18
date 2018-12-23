extern crate regex;

use std::collections::HashMap;
use std::io::{self, BufRead};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let entries : Vec<Nanobot> = handle.lines().map(|x| parse_line(&x.unwrap())).collect();

    let strongest = entries.iter().max_by_key(|x| x.r).unwrap();

    println!("The strongest nanobot is {:?}", strongest);

    let count = entries.iter().map(|x| manhattan_distance(strongest, x)).filter(|x| *x <= strongest.r).count();
    println!("{} nanobots in its radius", count);

    let mut space = HashMap::new();
    for (idx, nanobot) for entries.as_slice.iter() {
        println!("computing 3d space for nanobot {}", idx);
        for b2 as entries.as_slice().iter() {
            let dist = manhattan_distance(nanobot, b2);

            if dist >= nanobot.r {
                println!("nanobot {:?} overlaps nanobot {:?}", nanobot, b2);
            }
        }
    }

    let (pos, count) = space.iter().max_by_key(|item| item.1).unwrap();
    println!("Max count {} at {:?}", count, pos);
}

// Example inputs:
// pos=<0,0,0>, r=4
// pos=<1,0,0>, r=1
// pos=<4,0,0>, r=3
fn parse_line(line: &str) -> Nanobot{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"pos=<(\-?\d+),(\-?\d+),(\-?\d+)>, r=(\d+)").unwrap();
    }
    let captures = RE.captures(line).unwrap();


    let x = captures[1].parse().unwrap();
    let y = captures[2].parse().unwrap();
    let z = captures[3].parse().unwrap();
    let r = captures[4].parse().unwrap();

    Nanobot{x, y, z, r}
}

#[derive(Debug)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

fn manhattan_distance(one: &Nanobot, two: &Nanobot) -> i32 {
    (one.x - two.x).abs() + (one.y - two.y).abs() + (one.z - two.z).abs()
}
