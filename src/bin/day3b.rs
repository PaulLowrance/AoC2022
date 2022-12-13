fn main() {
    let input = include_str!("../../inputs/day3.prod");
    let lines= input.split('\n');
    let mut total = 0;

    let mut x = lines.peekable();
    while x.peek().is_some() {
        let grp: Vec<&str> = x.by_ref().take(3).collect();
        if grp.len() < 3 {
            println!("{:?}", total);
            return;
        }
        total += find_common_char(grp[0], grp[1], grp[2]);
    }
}

fn find_common_char(c1: &str, c2: &str, c3: &str) -> i32 {
    for c in c1.chars() {
        if c2.contains(c) && c3.contains(c) {
            return points_from_char(c);
        }
    }
    return 0;
}
fn points_from_char(c: char) -> i32 {
    let c_point = c as i8;
    if c.is_lowercase() {
        return (c_point - 96) as i32;
    }
    return (c_point - 38) as i32;
}
