use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("oops");

    let signals = contents.lines();

    for signal in signals {
        let mut signal_items = Vec::new();

        for item in signal.chars() {
            signal_items.push(item);
        }

        // signal_items.windows(4)
        //     .inspect(|w| println!("{}, {}, {}, {}", w[0], w[1], w[2], w[3]))
        //     .collect::<Vec<_>>();

        let mut signal_chars_count = 4;

        for window in signal_items.windows(4) {
            let mut map:HashSet<&char> = HashSet::new();

            for i in window {
                map.insert(&i);
            }

            if map.len() == 4 {
                break;
            }

            signal_chars_count += 1;
        }

        println!("{}", signal_chars_count);
    }
}