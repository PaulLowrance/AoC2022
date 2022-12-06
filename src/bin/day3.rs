fn main() {
    let input = include_str!("./day3.input.txt");
    let lines = input.split('\n');
    let mut total = 0;

    for line in lines {
        let count = line.chars().count();

        let first_half = &line[..count/2].chars().into_iter();
        let last_half = &line[count/2 .. count];


        for c in line[..count/2].chars() {
            let dup_points = get_duplicate_char(&c, last_half);
            if dup_points > 0 {
                total += dup_points;
                println!("Dup Points: {:?} Char: {:?}, Char as int {:?}", dup_points, c, c as i8);
                break;
            }
        }

    }

    println!("TOTAL: {:?}", total);
}

fn get_duplicate_char(test_char: &char, string_half: &str) -> i32 {
    for c in string_half.char_indices() {
        if c.1.eq(test_char) {
            return points_from_char(c.1);
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
