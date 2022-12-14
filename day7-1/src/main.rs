use std::{fs, collections::{HashMap}};

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
            // fn parse(a: &str) -> Results<i32, ParseIntError> {
            //     a.parse::<i32>()
            // }

            let filesize = line.split(" ")
                .next()
                // .and_then(parse)
                .expect("womp")
                .parse::<i32>()
                .expect("can't parse");

            // println!("{}", from_str::<i32>(line));
            // println!("{}", num);
            let mut dirs_to_drop = 0;
            let directory_stack_len = directory_stack.len();

            for _dir in directory_stack.clone().into_iter().rev() {
                
                // let reversed_dirs:Vec<&str> = directory_stack.clone().into_iter().rev().collect();
                // let dirs_joined = reversed_dirs[dirs_to_drop..].join("/");
                let dirs_joined = directory_stack.clone()[..directory_stack_len - dirs_to_drop].to_vec().join("/");

                // println!("{}", dirs_joined);

                *directory_sizes.get_mut(&dirs_joined)
                    .expect("directory path not found") += filesize;

                // directory_sizes.insert(dirs_joined.to_string(), *current_size + filesize);

                dirs_to_drop += 1;
            }
        }

        // println!("{:?}", directory_stack);
    }

    println!("{:?}", directory_sizes);

    let a = directory_sizes.values().filter(|&x| *x <= 100000).sum::<i32>();
    // let a = directory_sizes.values().filter
    println!("{}", a);

}
