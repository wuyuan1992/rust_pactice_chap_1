use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let target = rand::thread_rng().gen_range(1..101);
    println!("Target is {:?}. Let's go!", target);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("you guess: {:?}", guess);

        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number");
                continue;
            }
        };

        match guess_num.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congrats. Answer is {:?}", target);
                break;
            }
        }
    }
}
