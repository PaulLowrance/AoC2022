fn main() {
    let input = include_str!("./day3.test");
    let lines = input.split('\n');

    for line in lines {
        let count = line.chars().count();
        

    }
}
 fn get_duplicate_char(test_char: char, string_half: &str) -> i32 {
    for c in string_half.char_indices() {
        if c.eq(test_char) {
            return points_from_char(c);
        }
    }
 }

fn points_from_char(c: char) -> i32 {
    
}
