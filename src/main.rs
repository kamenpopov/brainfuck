use std::io;
use std::io::Read;

fn find_closing(idx: usize, chars: &Vec<char>) -> usize {
    let mut open_brackets = 0;
    for i in idx..chars.len() {
        match chars[i] {
            '[' => open_brackets += 1,
            ']' => {
                if open_brackets == 0 {
                    return idx;
                }
                open_brackets -= 1;
            },
            _ => {}
        }
    }
    0
}

fn find_opening(idx: usize, chars: &Vec<char>) -> usize {
    let mut close_brackets = 0;
    for i in (0..idx).rev() {
        match chars[i] {
            ']' => close_brackets += 1,
            '[' => {
                if close_brackets == 0 {
                    return i;
                }
                close_brackets -= 1;
            },
            _ => {}
        }
    }
    0
}

fn main() {
    let input = include_str!("input.txt");
    let chars = input.chars().collect::<Vec<char>>();

    let mut memory_position = 0;
    let mut memory: Vec<u32> = vec![0; 1000];

    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '+' => {
                memory[memory_position] += 1;
            },
            '-' => {
                memory[memory_position] -= 1;
            },
            '>' => {
                memory_position += 1;
            },
            '<' => {
                memory_position -= 1;
            },
            '[' => {
                let idx_of_closing = find_closing(i, &chars);
                if memory[memory_position] == 0 {
                    i = idx_of_closing + 1;
                }
            },
            ']' => {
                if memory[memory_position] != 0 {
                    let idx_of_open = find_opening(i, &chars);
                    i = idx_of_open;
                }
            },
            '.' => {
                print!("{}", char::from_u32(memory[memory_position].into()).unwrap());
            },
            ',' => {
                let input = io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u32)
                    .unwrap();

                memory[memory_position] = input;
            },
            _ => {}
        }
        i += 1;
    }
}
