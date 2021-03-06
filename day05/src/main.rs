use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut polychain_str = String::new();
    stdin.read_to_string(&mut polychain_str)?;
    println!("Part 1 result: {}", part_1(&polychain_str));

    let alpha = "abcdefghijklmnopqrstuvwxyz";

    let min_chain = alpha.chars().map(|letter| part_1(&polychain_str.as_str().replace(letter, "").as_str().replace(letter.to_ascii_uppercase(), ""))).min().unwrap();

    println!("{}", min_chain);

    Ok(())
}

fn reacts (c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

fn part_1(polychain_str: &str) -> usize {
    let mut polychain : Vec<u8> = polychain_str.chars().filter(|c| c.is_ascii_alphabetic()).map(|c| c as u8).collect();

    let mut prev_len = polychain.len() + 1;
    let mut new_polychain : Vec<u8> = Vec::new();
    while polychain.len() < prev_len {
        let mut dropped_prev = false;
        let windows = polychain.as_slice().windows(2);
        let windows_len = windows.len();
        for (idx, pair) in windows.enumerate() {
            let c1 = pair[0] as char;
            let c2 = pair[1] as char;

            if dropped_prev {
                dropped_prev = false;
                continue;
            }

            assert!(dropped_prev == false);

            if reacts(c1, c2) {
                dropped_prev = true;
                continue;
            } else {
                new_polychain.push(pair[0]);

                if idx == windows_len - 1 {
                    new_polychain.push(pair[1]);
                }
            }
        }

        prev_len = polychain.len();
        polychain = new_polychain.clone();
        assert!(polychain == new_polychain);
        new_polychain.clear();
    }

    polychain.len()
}
