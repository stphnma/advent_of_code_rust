use hashbrown::HashMap;
use regex::Regex;
use std::fs;

fn parse_stack(stack_str: &Vec<&str>, stack_numbers: &str) -> HashMap<usize, Vec<char>> {
    let mut pos_to_stack: HashMap<usize, usize> = HashMap::new();
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    for (i, x) in stack_numbers.chars().enumerate() {
        if x != ' ' {
            pos_to_stack.insert(i as usize, x.to_digit(10).unwrap() as usize);
        }
    }

    pos_to_stack.values().for_each(|k| {
        stacks.insert(k.clone(), Vec::new());
    });

    stack_str.iter().for_each(|line| {
        for (pos, c) in line.chars().enumerate() {
            match c {
                '[' => continue,
                ']' => continue,
                ' ' => continue,
                other => stacks.get_mut(&pos_to_stack[&pos]).unwrap().push(other),
            }
        }
    });

    pos_to_stack.values().for_each(|k| {
        stacks.get_mut(k).unwrap().reverse();
    });

    stacks
}

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_directions(directions: Vec<&str>) -> Vec<Move> {
    let mut move_list: Vec<Move> = Vec::new();

    let re = Regex::new(r"move (\d{1}+) from (\d{1}) to (\d{1})").unwrap();

    directions.iter().for_each(|line| {
        let mat = re.captures(line).unwrap();

        move_list.push(Move {
            num: mat
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap(),
            from: mat
                .get(2)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap(),
            to: mat
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap(),
        });
    });
    move_list
}

pub fn main() {
    let contents = fs::read_to_string("problem5_input.txt").unwrap();

    let mut stack_str: Vec<&str> = Vec::new();
    let mut stack_numbers: String = String::new();
    let mut directions: Vec<&str> = Vec::new();
    let mut stacking = true;
    for line in contents.lines() {
        if line.len() == 0 {
            continue;
        }
        if (line.len() >= 2)
            & (line.chars().nth(0).unwrap() == ' ')
            & (line.chars().nth(1).unwrap() == '1')
        {
            stacking = false;
            stack_numbers.push_str(line);
        } else if stacking {
            stack_str.push(line);
        } else {
            directions.push(line)
        }
    }

    let mut stacks = parse_stack(&stack_str, &stack_numbers);
    println!("{:?}", stacks);

    let move_list = parse_directions(directions);

    move_list.iter().for_each(|moo| {
        println!("{:?}", moo);
        if let Some([from_stack, to_stack]) = stacks.get_many_mut([&moo.from, &moo.to]) {
            for i in 0..(moo.num) {
                // Part 1
                // let index = from_stack.len() - 1;

                // Part 2
                let index = from_stack.len() - (moo.num - i);
                to_stack.push(from_stack.remove(index));
                println!("{:?}, {:?}", from_stack, to_stack);
            }
        }
    });

    for i in 1..10 {
        println!("{:?}", stacks.get(&i).unwrap().last());
    }
}
