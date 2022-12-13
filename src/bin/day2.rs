use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(PartialEq, Debug)]
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
    let input: &str = include_str!("../../inputs/day2.prod");
    let lines = input.split('\n');
    let mut total = 0;

    let mut points = HashMap::new();

    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);
    
    points.insert("A", 1);
    points.insert("B", 2);
    points.insert("C", 3);

    for line in lines {

        println!("{:?}", line);
        if line.is_empty() {
            break;
        }
        
        let round: Vec<&str> = line.split(' ').collect();
        let hand_points = points.get(round[1]);
        total += hand_points.unwrap(); 

        println!("hand_points {:?} Total {:?}", hand_points, total);
        let elf_hand: Result<Hand, _> = points.get(round[0]).cloned().unwrap().try_into();
        let player_hand: Result<Hand, _> = points.get(round[1]).cloned().unwrap().try_into();

        println!("elf_hand: {:?}", elf_hand.as_ref().unwrap());
        println!("player_hand: {:?}", player_hand.as_ref().unwrap());


        let result = win_lose(elf_hand.unwrap(), player_hand.unwrap()) as i32;
        total += result;
    }

    println!("Total : {:?}", total);
    
}

fn win_lose(elf_hand: Hand, player_hand: Hand) -> HandResult {
    match player_hand {
        _ if elf_hand == hand_beats(&player_hand) => HandResult::Win,
        _ if player_hand == hand_beats(&elf_hand) => HandResult::Lose,
        _ => HandResult::Draw
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
// fn win_lose_or_draw(map: HashMap<&str, i32>, elf_hand: &str, player_hand: &str) -> i32 {
//     let mut elf = map.get(elf_hand).unwrap();
//     let mut player = map.get(player_hand).unwrap();
//
//     let round = elf - player;
//     
//     if elf_hand == player_hand {
//         return 3;
//     } else if elf_hand == "A" {
//        if player_hand == "X" {
//            return 6;
//        } 
//         return 0;
//     }
// }
