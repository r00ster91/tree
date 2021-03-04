use std::{env, process};

const DARK_LEAVES: [char; 2] = ['░', '▒'];
const LIGHT_LEAF: char = '▓';
const STEM: char = '█';
const BLOCK_ELEMENT_BYTE_SIZE: usize = 3;

struct Tree {
    string: String,
}

impl Tree {
    fn new(size: usize) -> Tree {
        let size_half = size / 2;

        //
        // Precompute the required string capacity
        //
        let mut capacity = 0;

        // This is basically like below but with the string pushes replaced by addition
        for index in 0..size_half {
            // Spaces
            capacity += size_half - index;

            // Leaves
            capacity += (index * 2) * BLOCK_ELEMENT_BYTE_SIZE;

            // Newline
            capacity += 1;
        }

        // Spaces
        let stem_thickness = size / 6;
        let stem_height = size / 10;
        capacity += (size_half - stem_thickness / 2) * stem_height;

        // Stem
        capacity += stem_thickness * BLOCK_ELEMENT_BYTE_SIZE * stem_height;

        // Newline
        capacity += stem_height;

        //
        // Build the tree
        //
        let mut string = String::with_capacity(capacity);

        let mut next_leaf = 0;
        for index in 0..size_half {
            for _ in 0..size_half - index {
                string.push(' ');
            }
            for _ in 0..index * 2 {
                let leaf = match next_leaf {
                    0 => DARK_LEAVES[0],
                    1 => DARK_LEAVES[1],
                    _ => LIGHT_LEAF,
                };
                string.push(leaf);
                next_leaf = (next_leaf + 1) % 3;
            }
            string.push('\n');
        }

        for _ in 0..stem_height {
            for _ in 0..size_half - stem_thickness / 2 {
                string.push(' ');
            }
            for _ in 0..stem_thickness {
                string.push(STEM);
            }
            string.push('\n');
        }

        // println!(
        //     "capacity: {}, len: {}, calculated capacity: {}",
        //     string.capacity(),
        //     string.len(),
        //     capacity
        // );
        Tree { string }
    }
}

fn main() {
    let mut args = env::args();
    args.next().expect("No executable path");

    let tree_size = if let Some(arg) = args.next() {
        arg.parse::<usize>().unwrap_or_else(|_| {
            eprintln!("Argument must be a valid unsigned integer");
            process::exit(1);
        })
    } else {
        eprintln!("Using 50 as the tree size");
        50
    };

    let tree = Tree::new(tree_size);

    println!("{}", tree.string);
}
