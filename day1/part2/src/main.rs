use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let separator = Regex::new(r"\n\n").unwrap();
    let contents_separated = separator.split(&contents);

    let mut largest = 0;
    let mut second_largest = 0;
    let mut third_largest = 0;

    for calories_per_elf in contents_separated {
        let mut totaled_calories = 0;

        for caloric_item in calories_per_elf.lines() {
            totaled_calories += caloric_item.parse::<i32>().unwrap();
        }

        if totaled_calories > largest {
            third_largest = second_largest;
            second_largest = largest;
            largest = totaled_calories;
        } else if totaled_calories > second_largest {
            third_largest = second_largest;
            second_largest = totaled_calories;
        } else if totaled_calories > third_largest {
            third_largest = totaled_calories;
        }
    }
        
    println!("{largest}");
    println!("{second_largest}");
    println!("{third_largest}");

    println!("{}", largest + second_largest + third_largest);
}
