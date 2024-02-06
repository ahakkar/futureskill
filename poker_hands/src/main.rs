#![allow(unused)]
use std::collections::*;

static CARD_RANKS: &str = "AJTKQ23456789";    

fn is_flush(hand: Vec<String>) -> bool {
    let suit: char = hand.first().unwrap().chars().nth(1).unwrap();

    for card in hand {
        if !CARD_RANKS.contains(card.chars().next().unwrap()) ||
        card.chars().nth(1).unwrap() != suit {
            return false;
        }
    }

    true
}

// absolute garbage
fn is_straight(hand: Vec<String>) -> bool {
    let card_chars: Vec<char> = hand
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect(); 

    let mut card_nums: Vec<u32> = card_chars
        .iter()
        .map(|card| 
            match card {
                '2'..='9' => card.to_digit(10).unwrap(),
                'A' => 14,
                'K' => 13,
                'Q' => 12,   
                'J' => 11,          
                'T' => 10,
                _ => panic!(),
            })
        .collect();    

    card_nums.sort_unstable();
    let mut straight = true;
    
    // regular check
    for i in 0..4 {
        if card_nums.get(i).unwrap() + 1 != *card_nums.get(i+1).unwrap() {
            straight = false;
            break;
        }
    }

    // check for 1-5
    if !straight && card_nums.contains(&14) {
        for num in &mut card_nums {
            if *num == 14 { *num = 1; }
        } 
        card_nums.sort_unstable();  

        for i in 0..4 {
            if card_nums.get(i).unwrap() + 1 != *card_nums.get(i+1).unwrap() {
                return false;
            }  
        }
        straight = true;
    }

    if !straight {
        return false;
    }

    true
}

fn is_rsf(hand: Vec<String>) -> bool {
    if is_straight(hand.clone()) && is_flush(hand.clone()) {
        let card_chars: Vec<char> = hand
            .iter()
            .map(|s| s.chars().next().unwrap())
            .collect(); 

        if card_chars.contains(&'A') && card_chars.contains(&'K') {
            return true;
        }        
    }

    false
}

fn main() {
    
    let hand1: Vec<String> = vec!["Ad".to_string(), "Jd".to_string(), "Td".to_string(), "8d".to_string(), "4d".to_string()];
    let hand2: Vec<String> = vec!["Ad".to_string(), "Jd".to_string(), "Th".to_string(), "8d".to_string(), "4d".to_string()];
    
    println!("flush: {}", is_flush(hand1));
    println!("flush: {}", is_flush(hand2));

    let hand3: Vec<String> = vec!["Ah".to_string(), "Jh".to_string(), "Kh".to_string(), "Qh".to_string(), "Th".to_string()];
    let hand4: Vec<String> = vec!["Ah".to_string(), "Jh".to_string(), "7h".to_string(), "Qh".to_string(), "Th".to_string()];
    let hand5: Vec<String> = vec!["5h".to_string(), "3h".to_string(), "6d".to_string(), "4c".to_string(), "8c".to_string()];
    let hand6: Vec<String> = vec!["2d".to_string(), "3h".to_string(), "4c".to_string(), "5c".to_string(), "Ad".to_string()];
    println!("straight: {}", is_straight(hand3));
    println!("straight: {}", is_straight(hand4));
    println!("straight: {}", is_straight(hand5));
    println!("straight: {}", is_straight(hand6));

    let hand7: Vec<String> = vec!["Tc".to_string(), "Jc".to_string(), "Qc".to_string(), "Kc".to_string(), "Ac".to_string()];
    let hand8: Vec<String> = vec!["Td".to_string(), "Jd".to_string(), "Qd".to_string(), "Qd".to_string(), "Ad".to_string()];

    println!("straight flush: {}", is_rsf(hand7));
    println!("straight flush: {}", is_rsf(hand8));


/*     let cards: Vec<Option<char>> = hand1
        .iter()
        .map(|s| s.chars().next())
        .collect(); 

    let suits: Vec<Option<char>> = hand1
        .iter()
        .map(|s| s.chars().nth(1))
        .collect();  */


    
}
