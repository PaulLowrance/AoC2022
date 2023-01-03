use std::str::FromStr;

//Nothing is more fun than a barrel of monkeys!
type Barrel = Vec<Monkey>;

struct Monkey {
    id: u32,
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

fn parse_input(input: &str) -> Option<Barrel> {
    let barrel = Barrel::new();

    let incoming_monkeys = input.split("\n\n");

    for unparsed_monkey in incoming_monkeys {

        // Get the monkey number
        let id = unparsed_monkey.lines()
            .next()?
            .chars()
            .rev()
            .nth(unparsed_monkey.len() - 2)?
            .to_digit(10)?;

        // get the list of items
        let (_, item_chunk) = unparsed_monkey.lines().next()?.split_once(":")?;

        let items = item_chunk
            .split_terminator(", ")
            .filter_map(|str| str.parse().ok())
            .collect();

        // Get the test equation parts 
        // The operand ...
        let (_, op) = unparsed_monkey.lines().next()?.split_once("= old")?;
        let (operator, operand) = op.split_once(" ")?;
        let operand = match operand {
            "old" => Value::Old,
            _ => {
                let x = operand.parse().ok()?;
                Value::Num(x)
            }
        };

        let op = match operator {
            "+" => Operation::Add(operand),
            "-" => Operation::Subtract(operand),
            "\\" => Operation::Divide(operand),
            "*" => Operation::Multiply(operand),
            _ => panic!("Invalid Operator"),
        };

        // The division value...
        let (_, divisible) = unparsed_monkey.lines().next()?.rsplit_once(" ")?;
        let divisible = divisible.parse().ok()?;

        // The true monkey value...
        let (_, true_monkey) = unparsed_monkey.lines().next()?.rsplit_once(" ")?;
        let true_monkey = true_monkey.parse().ok()?;
        
        // The false value...
        let (_, false_monkey) = unparsed_monkey.lines().next()?.rsplit_once(" ")?;
        let false_monkey = false_monkey.parse().ok()?;

        let test = Test {
            divisible,
            monkey_if_true: true_monkey,
            monkey_if_false: false_monkey,
        };


        let monkey = Monkey {
            id,
            items,
            op,
            test,
            inspections: 0,
        };

        barrel.push(monkey);

    }

    return Some(barrel);
}
