use std::fs;

pub fn problem_1() {
    let contents = fs::read_to_string("problem1_input.txt").unwrap();

    let mut calories_per_elf: Vec<i64> = Vec::new();
    let mut total_calories: i64 = 0;

    for line in contents.lines() {
        // println!("{}", line);
        match line {
            "" => {
                calories_per_elf.push(total_calories.clone());
                total_calories = 0;
            }
            num => total_calories += num.parse::<i64>().unwrap(),
        }
    }
    calories_per_elf.sort();
    // println!("{:?}", &calories_per_elf[calories_per_elf.len() - 3..]);
    println!(
        "Max calorie is {:?}",
        calories_per_elf.iter().max().unwrap()
    );
    // println!("{:?}", top_calories);
    println!(
        "Top 3 calories is {:?}",
        &calories_per_elf[calories_per_elf.len() - 3..]
            .iter()
            .sum::<i64>()
    );
}
