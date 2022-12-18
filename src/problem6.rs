use std::collections::HashSet;
use std::fs;

pub fn main() {
    let contents = fs::read_to_string("problem6_input.txt").unwrap();

    let signal_length = 14;
    for i in signal_length..contents.len() {
        println!("{:?}", &contents[i - signal_length..i]);

        let m: HashSet<char> = contents[i - signal_length..i].chars().collect();
        if m.len() == 14 {
            println!("{}", i);
            break;
        }
    }
}
