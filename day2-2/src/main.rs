use std::fs;

fn score_my_choice(choice: &str) -> i16 {
    let mut score;

    if choice == "A" {
        score = 1;    
    } else if choice == "B" {
        score = 2;
    } else {
        score = 3;
    }

    return score;
}

fn score_vs_opponent_choice(opponent_choice: &str, my_choice: &str) -> i16 {
    let mut score;

    if opponent_choice == my_choice {
        score = 3;
    } else {
        if my_choice == "A" && opponent_choice == "C" {
            score = 6;
        } else if my_choice == "B" && opponent_choice == "A" {
            score = 6;
        } else if my_choice == "C" && opponent_choice == "B" {
            score = 6;
        } else {
            score = 0;
        }
    }

    return score;
}

fn calculate_choice<'a>(opponent_choice: &'a str, desired_result: &'a str) -> &'a str {
    let rock = "A";
    let paper = "B";
    let scissors = "C";

    let lose = "X";
    let draw = "Y";
    let win = "Z";

    let selection;

    if desired_result == draw {
        selection = &opponent_choice;
    } else if desired_result == lose {
        if opponent_choice == rock {
            selection = &scissors;
        } else if opponent_choice == scissors {
            selection = &paper;
        } else {
            selection = &rock;
        }
    } else {
        if opponent_choice == rock {
            selection = &paper;
        } else if opponent_choice == scissors {
            selection = &rock;
        } else {
            selection = &scissors;
        }
    }

    selection
}

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let rounds = contents.lines();

    let mut score = 0;

    for round in rounds {
        // println!("{}", round);
        let mut choices = round.split_whitespace();

        
        
        let opponent_choice = choices.next().unwrap();
        let desired_result = choices.next().unwrap();

        let my_choice = calculate_choice(&opponent_choice, desired_result);

        // println!("{}", my_choice);

        // // let my_choice_translated = translate_choice(my_choice);

        let choice_score = score_my_choice(my_choice);
        let victory_score = score_vs_opponent_choice(opponent_choice, my_choice);

        // println!("{}, {}", opponent_choice, my_choice);
        // println!("{}, {}", choice_score, victory_score);

        score += choice_score + victory_score;

        // // println!("{}, {}", choices.nth(0).unwrap(), choices.nth(1).unwrap());
        // // let mut score_for_choice: i32;


    }

    println!("{}", score);

    // for calories_per_elf in contents_separated {
    //     let mut totaled_calories = 0;

    //     for caloric_item in calories_per_elf.lines() {
    //         totaled_calories += caloric_item.parse::<i32>().unwrap();
    //     }

    //     if totaled_calories > largest {
    //         largest = totaled_calories;
    //     }
    // }
        
    // println!("{largest}");
}