use std::fs;

fn find_duplicate<'a>(first: &'a str, second: &'a str) -> char {
    let characters = first.chars();

    for character in characters {
        if second.contains(character) {
            return character;
        }
    }

    panic!("oh shit");
}

fn score_char(character_to_score: char) -> i32 {
    let offset: i32;

    if character_to_score.is_lowercase() {
        offset = -1 * ('a' as i32) + 1;
    } else {
        offset = -1 * ('A' as i32) + 27;
    }

    return (character_to_score as i32) + offset;
    // (character_to_score as u32) - 
}

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let inventories = contents.lines();

    let mut buffer = Vec::new();

    for inventory in inventories {
        // println!("{}", inventory.chars().count());
        let inventory_length = inventory.chars().count();

        let (first, second) = inventory.split_at(inventory_length / 2);
        let duplicated_character: char = find_duplicate(first, second);

        // println!("{}", score_char(duplicated_character));

        buffer.push(score_char(duplicated_character));

        // println!("{}, {}", first, second);
    }

    let total_priority: _ = buffer.into_iter().map(|x| x).sum::<i32>();

    println!("{}", total_priority);
}
