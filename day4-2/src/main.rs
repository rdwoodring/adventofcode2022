use std::{fs, collections::BTreeSet};

fn transform_pair_section_assignment_to_vector(section_assignment: &str) -> Vec<u32> {
    let mut split_assignment = section_assignment.split("-");

    let start = split_assignment.next()
        .expect("Invalid starting section")
        .parse::<u32>()
        .expect("Unable to convert start to int");

    let end = split_assignment.next()
        .expect("Invalid ending section")
        .parse::<u32>()
        .expect("Unable to parse end to int");

    let mut assignment_range = Vec::new();

    for i in start..(end + 1) {
        assignment_range.push(i);
    }

    assignment_range
}

fn check_assignment_ranges_for_overlap(assignment1: &Vec<u32>, assignment2: &Vec<u32>) -> bool {
    let mut assignment1_tree = BTreeSet::new();
    let mut assignment2_tree = BTreeSet::new();

    for assignment in assignment1 {
        assignment1_tree.insert(assignment);
    }

    for assignment in assignment2 {
        assignment2_tree.insert(assignment);
    }

    let intersection: Vec<_> = assignment1_tree.intersection(&assignment2_tree)
        .cloned()
        .collect();

    let intersection_count = intersection.len();

    // let mut overlaps_fully = false;

    // if intersection_count == assignment1.len() || intersection_count == assignment2.len() {
    //     overlaps_fully = true;
    // }

    // overlaps_fully

    intersection_count > 0
}

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let pair_section_assignments = contents.lines();

    let mut overlaps_count = 0;

    for pair_section_assignment in pair_section_assignments {
        let individual_assigments = pair_section_assignment.split(",");

        let mut individual_assigments_ranges = Vec::new();

        for assignment in individual_assigments {
            individual_assigments_ranges.push(transform_pair_section_assignment_to_vector(assignment));
        }

        let first_range = individual_assigments_ranges.first()
            .expect("First range doesn't exist");
        
        let second_range = individual_assigments_ranges.last()
            .expect("Last range doesn't exist");

        let overlaps = check_assignment_ranges_for_overlap(first_range, second_range);

        if overlaps {
            overlaps_count += 1;
        }
    }

    println!("{}", overlaps_count);

}
