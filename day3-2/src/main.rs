use std::{fs, collections::BTreeSet};

fn find_duplicates(first: &str, second: &str) -> String {
    let mut first_btree = BTreeSet::new();
    let mut second_btree = BTreeSet::new();

    for item in first.chars() {
        first_btree.insert(item);
    }

    for item in second.chars() {
        second_btree.insert(item);
    }

    let intersection: Vec<_> = first_btree.intersection(&second_btree).cloned().collect();

    intersection.into_iter().fold("".to_string(), |acc, next| acc + &next.to_string())

    
}

fn score_char(character_to_score: char) -> i32 {
    let offset: i32;

    if character_to_score.is_lowercase() {
        offset = -1 * ('a' as i32) + 1;
    } else {
        offset = -1 * ('A' as i32) + 27;
    }

    return (character_to_score as i32) + offset;
}

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let mut inventories = contents.lines();
    let inventories_count = contents.lines().count();

    let mut buffer = Vec::new();

    for counter in 0..inventories_count / 3 {
        let slice_start_index = counter * 3;
        let slice_end_index = slice_start_index + 3;

        let mut inventory_group: Vec<Option<&str>> = Vec::new();

        for i in slice_start_index..slice_end_index {
            inventory_group.push(inventories.next());
        }

        let first = inventory_group.get(0).expect("dead").expect("dead");
        let second = inventory_group.get(1).expect("dead").expect("dead");
        let third = inventory_group.get(2).expect("dead").expect("dead");

        let first_dupes = find_duplicates(first, second);
        let final_dupes = find_duplicates(&first_dupes, third);

        let duplicated_char = final_dupes.chars().nth(0).expect("character not there");
     
        buffer.push(score_char(duplicated_char));
    }

    let total_priority: _ = buffer.into_iter().map(|x| x).sum::<i32>();

    println!("{}", total_priority);
}
