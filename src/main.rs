use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess the number!");

        println!("Please input your guess!");

        let mut guess: String = String::new();

        println!("The secret number is: {}", secret_number);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
