use std::str::FromStr;

//Nothing is more fun than a barrel of monkeys!
type Barrel = Vec<Monkey>;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u32>,
    op: Operation,
    test: Test,
    inspections: i32,
}

#[derive(Clone, Debug)]
enum Operation {
    Multiply(Value),
    Add(Value),
    Divide(Value),
    Subtract(Value),
}

impl Operation {
    fn apply(&self, old: u32) -> u32 {
        match &self {
            Operation::Add(val) => old + val.number(old),
            Operation::Multiply(val) => old * val.number(old),
            Operation::Subtract(val) => old - val.number(old),
            Operation::Divide(val) => old / val.number(old),
        }
        
    }
}


#[derive(Clone, Debug)]
enum Value {
    Old,
    Num(u32),
}

impl Value {
    fn number(&self, old: u32) -> u32 {
        match self {
            Value::Num(n) => *n,
            Value::Old => old,
        }
    }
}


#[derive(Clone, Debug)]
struct Test {
    divisible: u32,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

fn main() {
    let input = include_str!("../../inputs/day11.test");

    let monkeys = parse_input(input);
    println!("made it here: {:?}", monkeys);
    let part_one_result = part_one(&monkeys);


    println!("Part One: {part_one_result}");

}

fn part_one(barrel: &Option<Barrel>) -> String {
    let mut inspections = vec![0; barrel.as_ref().unwrap().len()];
    let barrel = barrel.clone();
    
    for _ in 0..20 {
        for idx in 0..barrel.as_ref().unwrap().len() {
            let items: Vec<u32> = barrel.as_ref().unwrap()[idx].clone().items.drain(..).collect();
            let monkey = barrel.as_ref().unwrap()[idx].clone();
            for old in items {
                inspections[idx] += 1;
                let new = monkey.op.apply(old);
                let new = new / 3;
                let idx = if new % monkey.test.divisible == 0 {
                    monkey.test.monkey_if_true
                } else {
                    monkey.test.monkey_if_false
                };
                let receiver = &mut barrel.as_ref().unwrap()[idx].clone();
                receiver.items.push(new);
            }
        }
    }
    inspections.sort_unstable();
    return inspections.iter().rev().take(2).product::<u64>().to_string();
}

fn parse_input(input: &str) -> Option<Barrel> {
    let mut barrel = Barrel::new();

    let incoming_monkeys = input.split("\n\n");

    for unparsed_monkey in incoming_monkeys {

        println!("Hello? {:?}", unparsed_monkey);

        // Get the monkey number
        /* let id = unparsed_monkey.lines()
            .next()?
            .chars()
            .rev()
            .nth(unparsed_monkey.len() - 2)?
            .to_digit(10)?; */

        // get the list of items
        let (_, item_chunk) = unparsed_monkey.lines().next()?.split_once(":")?;

        println!("Hello? {:?}", item_chunk);

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
            items,
            op,
            test,
            inspections: 0,
        };

        barrel.push(monkey);

    }

    return Some(barrel);
}
