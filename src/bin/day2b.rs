use std::collections::HashMap;


#[derive(PartialEq, Eq, Debug)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Debug)]
pub enum HandResult {
    Win = 6,
    Lose = 0,
    Draw = 3
}


fn main() {
    let input = include_str!("../../inputs/day2.prod");
    let lines = input.split('\n');
    let mut total = 0;

    let mut hand_points = HashMap::new();
    hand_points.insert("A", 1);
    hand_points.insert("B", 2);
    hand_points.insert("C", 3);

    let mut result_map = HashMap::new();
    result_map.insert("X", HandResult::Lose);
    result_map.insert("Y", HandResult::Draw);
    result_map.insert("Z", HandResult::Win);

    for line in lines {
        println!("{:?}", line);
        if line.is_empty() {
            continue;
        }

        let round: Vec<&str> = line.split(' ').collect();
        let elf_hand: Result<Hand, _> = hand_points.get(round[0]).cloned().unwrap().try_into();
        let expected_result = result_map.get(round[1]).unwrap();
        let player_hand = player_hand(elf_hand.unwrap(), expected_result);
        
        let player_hand_points = player_hand as i32;
        let expected_result_points = result_points(expected_result);        

        total += player_hand_points + expected_result_points;

        println!("Expected Result: {:?} + Hand Points: {:?} --- Total: {:?}", expected_result_points, player_hand_points, total);
    }

    println!("FINAL TOTAL {:?}", total)
    
}

fn result_points(result: &HandResult) -> i32 {
    match result {
        HandResult::Lose => 0,
        HandResult::Draw => 3,
        HandResult::Win => 6,
    }
}

fn player_hand(elf_hand: Hand, expected_result: &HandResult) -> Hand {
    match expected_result {
        HandResult::Draw => elf_hand,
        HandResult::Win => hand_loses(&elf_hand),
        HandResult::Lose => hand_beats(&elf_hand),
    }
}

fn hand_loses(h1: &Hand) -> Hand {
    match h1 {
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
        Hand::Rock => Hand::Paper,
    }
}

fn hand_beats(h1: &Hand) -> Hand {
    match h1 {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    }
}

impl TryFrom<i32> for Hand {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Hand::Rock as i32 => Ok(Hand::Rock),
            x if x == Hand::Paper as i32 => Ok(Hand::Paper),
            x if x == Hand::Scissors as i32 => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            x if x == "Rock" => Hand::Rock,
            x if x == "Paper" => Hand::Paper,
            x if x == "Scissors" => Hand::Scissors,
            _ => Hand::Rock,
        }
    }
}
