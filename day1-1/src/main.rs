use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let separator = Regex::new(r"\n\n").unwrap();
    let contents_separated = separator.split(&contents);

    let mut largest = 0;

    for calories_per_elf in contents_separated {
        let mut totaled_calories = 0;

        for caloric_item in calories_per_elf.lines() {
            totaled_calories += caloric_item.parse::<i32>().unwrap();
        }

        if totaled_calories > largest {
            largest = totaled_calories;
        }
    }
        
    println!("{largest}");
}
