use std::fs;

fn match_code(code: &str) -> Result<&str, &'static str> {
    return match code {
        "A" => Ok("Rock"),
        "B" => Ok("Paper"),
        "C" => Ok("Scissors"),
        "X" => Ok("Rock"),
        "Y" => Ok("Paper"),
        "Z" => Ok("Scissors"),
        _other => Err("Unaccounted move"),
    };
}

fn get_shape_score(shape: &str) -> Result<u32, &'static str> {
    return match shape {
        "Rock" => Ok(1),
        "Paper" => Ok(2),
        "Scissors" => Ok(3),
        _other => Err("Unaccounted move"),
    };
}

fn win_or_not_score(opp_move: &str, my_move: &str) -> Result<u32, &'static str> {
    return match (opp_move, my_move) {
        ("Rock", "Scissors") => Ok(0),
        ("Rock", "Paper") => Ok(6),
        ("Rock", "Rock") => Ok(3),
        ("Paper", "Scissors") => Ok(6),
        ("Paper", "Paper") => Ok(3),
        ("Paper", "Rock") => Ok(0),
        ("Scissors", "Scissors") => Ok(3),
        ("Scissors", "Paper") => Ok(0),
        ("Scissors", "Rock") => Ok(6),
        _other => Err("Unaccounted move"),
    };
}

fn calculate_score(opp_move: &str, my_move: &str) -> u32 {
    return win_or_not_score(opp_move, my_move).unwrap() + get_shape_score(my_move).unwrap();
}

fn get_my_move<'a>(opp_move: &'a str, outcome: &'a str) -> Result<&'a str, &'static str> {
    match (opp_move, outcome) {
        ("Rock", "X") => Ok("Scissors"),
        ("Rock", "Y") => Ok("Rock"),
        ("Rock", "Z") => Ok("Paper"),
        ("Paper", "X") => Ok("Rock"),
        ("Paper", "Y") => Ok("Paper"),
        ("Paper", "Z") => Ok("Scissors"),
        ("Scissors", "X") => Ok("Paper"),
        ("Scissors", "Y") => Ok("Scissors"),
        ("Scissors", "Z") => Ok("Rock"),
        _other => {
            println!("{}, {}", opp_move, outcome);
            return Err("Unaccounted move");
        }
    }
}

pub fn problem_2() -> Result<(), &'static str> {
    let contents = fs::read_to_string("problem2_input.txt").unwrap();

    let mut total_score: u32 = 0;

    for line in contents.lines() {
        let v = line.split_whitespace().take(2).collect::<Vec<&str>>();
        // Related: https://stackoverflow.com/questions/32324645/how-can-i-unpack-destructure-elements-from-a-vector

        if let [first, second] = &v[..] {
            let opp_move = match_code(first)?;
            //let my_move = match_code(second);
            let outcome = second;
            let my_move = get_my_move(opp_move, outcome)?;
            total_score += calculate_score(opp_move, my_move);
        }
    }
    println!("Total score is {}", total_score);
    Ok(())
}
