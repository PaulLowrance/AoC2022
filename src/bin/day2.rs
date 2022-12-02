use std::{collections::{HashMap, HashSet}, vec};

fn main() {
    let input: &str = include_str!("./day2_input.txt");
    let lines = input.split("\n");
    let mut total = 0;

    let points = HashMap::new();

    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);
    
    points.insert("A", 1);
    points.insert("B", 2);
    points.insert("C", 3);

    for line in lines {
        let round = line.split(" ");
        let mut handPoints = points.get(round.last());
        total += handPoints; 


    }
    
}

fn win_lose_or_draw(map: HashMap<&str, i32>, elf_hand: &str, player_hand: &str) -> i32 {
    let mut elf = map.get(elf_hand);
    let mut player = map.get(player_hand);
    
    let mut v = vec![elf, player];

    let round = elf - player;

    match round::abs::<i32>() {
        0 => -1,
        1 => v.iter().max::<i32>(),
        _ => v.iter().min::<i32>(),
    }
}
