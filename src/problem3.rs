use std::collections::HashSet;
use std::fs;

fn get_priority(c: char) -> u16 {
    let priority = c as u16;
    // magic numbers
    if priority >= 97 {
        return (priority - 97) + 1;
    } else {
        return (priority - 65) + 27;
    }
}

pub fn problem_3() {
    let contents = fs::read_to_string("problem3_input.txt").unwrap();

    let mut ssum1: u16 = 0;
    contents.lines().for_each(|l| {
        let cc: Vec<char> = l.chars().collect();

        let (first, second): (HashSet<&char>, HashSet<&char>) = (
            HashSet::from_iter(&cc[..(cc.len() / 2)]),
            HashSet::from_iter(&cc[(cc.len() / 2)..]),
        );

        let intersection = first.intersection(&second).nth(0).unwrap();
        ssum1 += get_priority(**intersection);
    });
    println!("Part 1: total is {:?}", ssum1);

    let mut ssum2: u16 = 0;
    contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .for_each(|lines| {
            let (first, second, third): (HashSet<char>, HashSet<char>, HashSet<char>) = (
                HashSet::from_iter(lines[0].chars()),
                HashSet::from_iter(lines[1].chars()),
                HashSet::from_iter(lines[2].chars()),
            );

            let sets = [first, second, third];
            let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });

            ssum2 += get_priority(*intersection.iter().nth(0).unwrap());
        });
    println!("Part 2total is {:?}", ssum2);
}
