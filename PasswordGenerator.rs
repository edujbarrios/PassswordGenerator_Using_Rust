extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    println!("Enter the length of the password:");

    let mut length = String::new();
    std::io::stdin().read_line(&mut length)
        .expect("Failed to read line");

    let length: u32 = match length.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid length");
            return;
        }
    };

    println!("Enter the number of special characters:");

    let mut special_chars = String::new();
    std::io::stdin().read_line(&mut special_chars)
        .expect("Failed to read line");

    let special_chars: u32 = match special_chars.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number of special characters");
            return;
        }
    };

    let mut rng = thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let i = rng.gen_range(0, 3);
            match i {
                0 => rng.gen_range(b'a', b'z') as char,
                1 => rng.gen_range(b'A', b'Z') as char,
                2 => rng.gen_range(b'0', b'9') as char,
                _ => unreachable!()
            }
        })
        .collect();

    let special_chars: String = (0..special_chars)
        .map(|_| {
            let i = rng.gen_range(0, special_chars.len());
            special_chars.chars().nth(i).unwrap()
        })
        .collect();

    println!("Your password is: {}", password);
}
