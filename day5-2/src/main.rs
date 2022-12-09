use std::{fs, collections::{HashMap}};
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let separator = Regex::new(r"\n\n").unwrap();
    let mut contents_separated = separator.split(&contents);

    let stack_drawing_raw = contents_separated.next()
        .expect("Stack drawing not found");

    let move_instructions_raw = contents_separated.next()
        .expect("Move instructions not found");

    println!("{}", remove_brackets_and_spaces_from_stack_drawing_raw(&stack_drawing_raw));
    let cleaned_stack_drawing_raw = remove_brackets_and_spaces_from_stack_drawing_raw(&stack_drawing_raw);

    let stack_map = initialize_stacks(&cleaned_stack_drawing_raw);

    // println!("{:?}", stack_map);
    // println!("{}", parse_instructions(&move_instructions_raw));
    let parsed_instructions = parse_instructions(&move_instructions_raw);

    let mut processed_stacks = perform_instructions(&parsed_instructions, &stack_map);

    println!("{:?}", processed_stacks);

    let processed_stacks_size = processed_stacks.keys().len();

    let mut top_crates = "".to_string();

    for i in 1..processed_stacks_size + 1 {
        let stack = processed_stacks.get_mut(&i.to_string()).expect("nope");

        // println!("{}", stack.pop().expect("").to_string());
        top_crates += &stack.pop().expect("").to_string();
    }

    // for (key, value) in processed_stacks {
    //     println!("{},{}", key, value.pop().unwrap());
    // }
    println!("{}", top_crates);
}

fn remove_brackets_and_spaces_from_stack_drawing_raw(stack_drawing_raw: &str) -> String {
    // let s = stack_drawing_raw.replace("    [", "[*] [");
    let s = stack_drawing_raw.replace("    ", "[*]");
    let s = s.replace("][", "] [");
    let s = s.replace("    [", "[*] [");
    let s = s.replace("]    ", "] [*]");
    // let s = stack_drawing_raw.replace("[", " ");
    let s = s.replace("[", " ");
    let s = s.replace("]", " ");
    let s = s.replace(" ", "");
    let s = s.replace("*", " ");

    s
}

fn initialize_stacks(sanitized_stack_drawing: &str) -> HashMap<String, Vec<char>> {
    let line_count = sanitized_stack_drawing.lines().count();
    let stack_count = sanitized_stack_drawing.lines()
        .nth(line_count - 1)
        .expect("oops")
        .chars()
        .count();

    let lines = sanitized_stack_drawing.lines();
    
    // let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut stacks: HashMap<String, Vec<char>> = HashMap::new();
    
    for stack_index in 0..stack_count {
        let mut stack: Vec<char> = Vec::new();

        stacks.insert((stack_index + 1).to_string(), Vec::new());

        // stacks.push(stack);

        // stacks.push(Vec::new());
    }

    let mut is_first_iteration = true;

    for line in lines.rev() {
        // println!("{}", line);
        if !is_first_iteration {
            let stack_item_count = line.chars().count();
            let mut stack_items = line.chars();

            // println!("{:?}", stack_items);

            for i in 0..stack_item_count {
                // stacks[i].push(stack_items.nth(i).expect("oops"));
                // let a = stacks.get(&i.to_string()).expect("dammit").push(stack_items.nth(i).unwrap());
                let stack_item = stack_items.next()
                    .expect("oops");

                // println!("{}", stack_item);
                let stack = stacks.get_mut(&(&i + 1).to_string()).unwrap();

                if stack_item != ' ' {
                    stack.push(stack_item);
                }
            }
        }

        is_first_iteration = false;
    }

    stacks    
}

fn parse_instructions(raw_instructions: &str) -> String {
    let instructions = raw_instructions.lines();

    let mut reassembled_instructions = "".to_string();

    for instruction in instructions {
        let s = instruction.replace("move ", "");
        let s = s.replace(" from ", ",");
        let s = s.replace(" to ", ",");

        reassembled_instructions += &(s + "\r\n").to_string();
    }

    reassembled_instructions.to_string()
}

fn perform_instructions(instructions: &str, stacks: &HashMap<String, Vec<char>>) -> HashMap<String, Vec<char>> {
    let mut stacks_clone = stacks.clone();

    for instruction in instructions.lines() {
        let mut instruction_parts = instruction.split(",");
        
        let move_amount = instruction_parts.next()
            .expect("No amount")
            .parse::<i32>()
            .expect("not parseable to int");

        let from = instruction_parts.next()
            .expect("No from");

        let to = instruction_parts.next()
            .expect("no to");

        let mut popped = Vec::new();

        for _i in 0..move_amount {
            let popped_item = stacks_clone.get_mut(from)
                .expect(&("from stack not found: ".to_string() + from))
                .pop()
                .expect("nothing popped");

                // stacks_clone.get_mut(to)
                // .expect(&("to stack not found: ".to_string() + to))
                // .push(popped)
            popped.push(popped_item);
        }

        for popped_item in popped.into_iter().rev() {
            stacks_clone.get_mut(to)
                .expect(&("to stack not found: ".to_string() + to))
                .push(popped_item);
        }
    }

    // stacks.clone()
    stacks_clone
}