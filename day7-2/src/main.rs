use std::{fs, collections::{HashMap, BinaryHeap}};

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let output_lines = contents.lines();

    let mut directory_stack: Vec<&str> = Vec::new();

    let mut directory_sizes: HashMap<String, i32> = HashMap::new();

    for line in output_lines {
        if line.starts_with("$ cd ..") {
            directory_stack.pop();
        } else if line.starts_with("$ cd") {
            let dir = line.split("$ cd ")
                .last()
                .expect("Invalid Path");

            if dir == "/" {
                directory_stack.clear();
            }

            directory_stack.push(dir);

            directory_sizes.insert(directory_stack.join("/"), 0);
        } else if !line.starts_with("$") && !line.starts_with("dir") {
            let filesize = line.split(" ")
                .next()
                .expect("womp")
                .parse::<i32>()
                .expect("can't parse");

            let mut dirs_to_drop = 0;
            let directory_stack_len = directory_stack.len();

            for _dir in directory_stack.clone().into_iter().rev() {
                
                let dirs_joined = directory_stack.clone()[..directory_stack_len - dirs_to_drop].to_vec().join("/");

                *directory_sizes.get_mut(&dirs_joined)
                    .expect("directory path not found") += filesize;

                dirs_to_drop += 1;
            }
        }
    }

    println!("{:?}", directory_sizes);

    let total_file_system_size = 70000000;
    let available_needed = 30000000;

    let root_size = directory_sizes.get("/")
        .expect("that's impossible");

    let current_available = total_file_system_size - root_size;

    let need_to_clear = available_needed - current_available;

    let mut heap: BinaryHeap<&i32> = BinaryHeap::new();

    directory_sizes.values().filter(|&x| *x >= need_to_clear).for_each(|f| heap.push(f));

    println!("{:?}", heap.into_sorted_vec().first());
}
