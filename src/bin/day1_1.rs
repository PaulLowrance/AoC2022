fn main() {
    let input: &str = include_str!("./day1_1_input.txt");

    let lines = input.split("\n\n");

    let linesParse: Option<u32> = lines.map(|line|line.split("\n").flat_map(|num|num.parse::<u32>()).sum::<u32>()).max();

    println!("{:?}", linesParse);
}
