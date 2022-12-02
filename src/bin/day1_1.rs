fn main() {
    let input: &str = include_str!("./day1_1_input.txt");

    let lines = input.split("\n\n");

    let mut linesParse: Vec<u32> = lines.map(|line|line.split("\n").flat_map(|num|num.parse::<u32>()).sum::<u32>()).collect();

    linesParse.sort_by(|a,b|b.cmp(a));
    
    println!("{:?}", linesParse.iter().take(3).sum::<u32>());
}
