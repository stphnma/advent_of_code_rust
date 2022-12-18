mod problem1;
use problem1::problem_1;

mod problem2;
use problem2::problem_2;

mod problem3;
use problem3::problem_3;

mod problem4;
use problem4::problem_4;

mod problem5;
use problem5::main as problem_5;

mod problem6;
use problem6::main as problem_6;

mod problem8;
use problem8::main as problem_8;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let problem_num = &args[1].parse::<i32>().unwrap();

    match problem_num {
        1 => problem_1(),
        2 => problem_2().unwrap(),
        3 => problem_3(),
        4 => problem_4(),
        5 => problem_5(),
        6 => problem_6(),
        8 => problem_8(),
        _other => println!("No matching problems!"),
    };
}
