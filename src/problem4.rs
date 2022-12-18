use std::fs;

fn parse_assignments(assignment: &str) -> Vec<u16> {
    assignment
        .split("-")
        .take(2)
        .map(|d| d.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
}

fn check_contains(assignment1: &Vec<u16>, assignment2: &Vec<u16>) -> bool {
    if (assignment1[0] <= assignment2[0]) & (assignment1[1] >= assignment2[1]) {
        return true;
    }
    return false;
}

fn check_overlap(assignment1: &Vec<u16>, assignment2: &Vec<u16>) -> bool {
    if (assignment1[0] >= assignment2[0]) & (assignment1[0] <= assignment2[1]) {
        return true;
    } else if (assignment1[1] >= assignment2[0]) & (assignment1[1] <= assignment2[1]) {
        return true;
    }
    return false;
}

pub fn problem_4() {
    let contents = fs::read_to_string("problem4_input.txt").unwrap();

    let mut num_contains = 0;
    let mut num_overlaps = 0;

    contents.lines().for_each(|line| {
        let sections = line.split(",").take(2).collect::<Vec<&str>>();

        let assignment1 = parse_assignments(sections[0]);
        let assignment2 = parse_assignments(sections[1]);
        println!("{:?}, {:?}, {:?}", sections, assignment1, assignment2);

        if check_contains(&assignment1, &assignment2) | check_contains(&assignment2, &assignment1) {
            num_contains += 1
        }

        if check_overlap(&assignment1, &assignment2) | check_overlap(&assignment2, &assignment1) {
            num_overlaps += 1
        }
    });
    println!(
        "Number of contains {}, overlaps {}",
        num_contains, num_overlaps
    );
}
