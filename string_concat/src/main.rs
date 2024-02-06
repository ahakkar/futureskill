#![allow(unused)]
use std::{collections::*, hash::Hash};



fn main() {
    let list_of_strings = vec!["ab", "ba", "aa"];
    let mut max_counts: HashMap<char, usize> = HashMap::new();

    for str in list_of_strings {
        let mut letters: HashMap<char, usize> = HashMap::new();

        for c in str.chars() {
            if let Some(x) = letters.get_mut(&c) {
                *x += 1;
            } else {
                letters.insert(c, 1);
            }            
        }

        for (c, count) in letters.iter() {
            let max_count = max_counts.entry(*c).or_insert(0);
            if *count > *max_count {
                *max_count = *count;
            }
        }
    }

    let mut chars: Vec<char> = vec![];
    for (c, &count) in max_counts.iter() {
        for _ in 0..count {
            chars.push(*c);
        }
    }

    chars.sort_unstable();
    let result: String = chars.into_iter().collect(); 

    println!("{}", result);
}


