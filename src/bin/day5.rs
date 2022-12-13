fn main() {
    let input = include_str!("../../inputs/day5.prod");

    let parsed_input = parse_input(input);
    let p1_result = part_one(&parsed_input);
    println!("Result: {}", p1_result);
    let p2_result = part_two(&parsed_input);
    println!("Result: {}", p2_result);
    
}

pub struct Procedure {
    num_of_containers: usize,
    from_stack: usize,
    to_stack: usize,
}

type Stack = Vec<Crate>;
type Crate = char;

pub fn part_two((in_stacks, instructions): &(Vec<Stack>, Vec<Procedure>)) -> String {
    let mut stacks = in_stacks.to_owned();

    for i in instructions {
        let from_stack = &mut stacks[i.from_stack];
        let mut moved_crates = from_stack.split_off(from_stack.len() - i.num_of_containers);
        stacks[i.to_stack].append(&mut moved_crates);
    }
    return stacks.iter().filter_map(|s|s.iter().last()).collect();
}

pub fn part_one((in_stacks, instructions): &(Vec<Stack>, Vec<Procedure>)) -> String {
    let mut stacks = in_stacks.to_owned();

    for i in instructions {
        for _x in 0..i.num_of_containers {
            let supply_crate = stacks[i.from_stack].pop().unwrap();
            stacks[i.to_stack].push(supply_crate);
        }
    }

    return stacks.iter().filter_map(|s| s.iter().last()).collect();
}

pub fn parse_input(input: &'static str) -> (Vec<Stack>, Vec<Procedure>) {
    //break input into two parts
    let (schema, procedures) = input.split_once("\n\n").unwrap();

    //get the size of the cargo area
    let graph: Vec<_> = schema.lines().collect();
    //each container is 4 char long wiht the trailing space e.g. '[X] '
    //This gets the count of stacks from the number of cahr in a row
    let graph_len = graph[0].len() / 4 + 1;
    println!("graph_len: {}", graph_len);

    //load the stacks from input
    let mut stacks: Vec<Stack> = vec![Vec::with_capacity(graph.len() - 1); graph_len + 1];

    for row_number in (0..graph.len() - 1).rev() {
        let row = graph[row_number];
        for (stack_number, stack) in stacks.iter_mut().enumerate().skip(1).take(graph_len) {
            let container = row.chars().nth((stack_number - 1) * 4 + 1).unwrap();

            if !container.is_ascii_whitespace() {
                stack.push(container);
            }
        }
    }

    let instructions = procedures
        .lines()
        .map(|step| {
            let mut numbers = step
                .split_ascii_whitespace()
                .filter_map(|token| token.parse().ok());

            Procedure {
                num_of_containers: numbers.next().unwrap(),
                from_stack: numbers.next().unwrap(),
                to_stack: numbers.next().unwrap(),
            }
        })
        .collect();

    return (stacks, instructions);
}
