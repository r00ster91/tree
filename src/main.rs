use rand::Rng;
use std::convert::TryFrom;

const DARK_LEAVES: [char; 2] = ['░', '▒'];
const LIGHT_LEAF: char = '▓';
const LEAF_BYTE_SIZE: usize = 3;
const STEM: char = '█';

struct Tree {
    size: usize,
    string: String,
}

impl Tree {
    fn new(size: usize) -> Tree {
        let size_half = size / 2;
        let mut capacity = 0;
        for index in 0..size_half {
            let space_count = size_half - index;
            capacity += space_count;
            capacity += (index * 2) * LEAF_BYTE_SIZE;
            capacity += 1;
        }

        let mut string = String::with_capacity(capacity);

        for index in 0..size_half {
            let space_count = size_half - index;
            for i in 0..space_count {
                string.push(' ');
            }
            for _ in 0..index * 2 {
                let leaf;
                let chosen_leaf = rand::thread_rng().gen_range(0..=6);
                if chosen_leaf == 6 {
                    leaf = LIGHT_LEAF;
                } else {
                    leaf = DARK_LEAVES[chosen_leaf % DARK_LEAVES.len()];
                }
                string.push(leaf);
            }
            string.push('\n');
        }

        let stem_thickness = size / 6;
        let stem_height = size / 10;

        for _ in 0..stem_height {
            for index in 0..size_half - stem_thickness / 2 {
                string.push(' ');
            }
            for index in 0..stem_thickness {
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
        Tree { size, string }
    }
}

fn main() {
    let tree = Tree::new(50);

    println!("{}", tree.string);
}
