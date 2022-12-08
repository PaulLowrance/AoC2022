fn main() {
    let input = include_str!("./day4_input.prod");
    let part_one = input.lines().map(|line| {
        let (left, right) = line.split_once(',').unwrap();
        let ((a,b),(c,d)) = (left.split_once('-').unwrap(), right.split_once('-').unwrap());
        (
        a.parse::<u8>().unwrap(),
        b.parse::<u8>().unwrap(),
        c.parse::<u8>().unwrap(),
        d.parse::<u8>().unwrap(),
    )
    })
        .filter(|(a,b,c,d)|(a >= c && b <= d)|| (a <= c && b >= d))
        .count();

    let part_two = input.lines().map(|line| {
        let (left, right) = line.split_once(',').unwrap();
        let ((a,b),(c,d)) = (left.split_once('-').unwrap(), right.split_once('-').unwrap());
        (
        a.parse::<u8>().unwrap(),
        b.parse::<u8>().unwrap(),
        c.parse::<u8>().unwrap(),
        d.parse::<u8>().unwrap(),
    )
    })
        .filter(|(a,b,c,d)|( a <= d && b >= c))
        .count();

    println!("Part one result: {}", part_one); 
    println!("Part two result: {}", part_two); 
}
