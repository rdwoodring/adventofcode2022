use std::fs;

// fn is_draw(opponent_choice, choice) {

// }
// struct ChoiceScoreKey {
//     A: i16,
//     B: i16,
//     C: i16
// }

fn translate_choice(choice: &str) -> &str {
    let mut translated_choice;

    if choice == "X" {
        translated_choice = "A";
    } else if choice == "Y" {
        translated_choice = "B";
    } else {
        translated_choice = "C";
    }

    return translated_choice;
}

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

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    // let score_key = ChoiceScoreKey {
    //     A: 1,
    //     B: 2,
    //     C: 3
    // };

    // let separator = Regex::new(r"\n\n").unwrap();
    // let contents_separated = separator.split(&contents);
    let rounds = contents.lines();

    let mut score = 0;

    for round in rounds {
        // println!("{}", round);
        let mut choices = round.split_whitespace();
        
        let opponent_choice = choices.next().unwrap();
        let my_choice = choices.next().unwrap();

        let my_choice_translated = translate_choice(my_choice);

        let choice_score = score_my_choice(my_choice_translated);
        let victory_score = score_vs_opponent_choice(opponent_choice, my_choice_translated);

        // println!("{}, {}", opponent_choice, my_choice_translated);
        // println!("{}, {}", choice_score, victory_score);

        score += choice_score + victory_score;

        // println!("{}, {}", choices.nth(0).unwrap(), choices.nth(1).unwrap());
        // let mut score_for_choice: i32;


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