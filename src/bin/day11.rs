use std::str::FromStr;

//Nothing is more fun than a barrel of monkeys!
type Barrel = Vec<Monkey>;

struct Monkey {
    id: i32,
    items: Vec<i32>,
    op: Operation,
    test: Test,
    inspections: i32,
}

enum Operation {
    Multiply(Value),
    Add(Value),
    Divide(Value),
    Subtract(Value),
}

enum Value {
    Old,
    Num(i32),
}

struct Test {
    divisible: i32,
    monkey_if_true: i32,
    monkey_if_false: i32,
}

fn main() {
    let input = include_str!("../../inputs/day11.test");

    let part_one_result = part_one(input);


    println!("Part One: {part_one_result}");

}

fn parse_input(input: &str) -> Barrel {
    let incoming_monkeys = input.split("\n\n");

    for unparsed_monkey in incoming_monkeys {

        let id = unparsed_monkey.lines().next().unwrap().chars().rev().nth(n - 2).unwrap();
        let (_, items) = unparsed_monkey.lines().next()?.split_once(":")?;
        let items = items.split_terminator(", ").filter_map(|str| str.parse().ok()).collect();



        let monkey = Monkey {
            id: id,
            items: items,
            op: (),
            test: Test {},
            inspections: 0,
        };

    }

    return Barrel::new();
}