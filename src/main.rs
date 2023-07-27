use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess the number!");

        println!("Please input your guess!");

        let mut guess: String = String::new()

        // USING THE STANDARD LIBRARY TO ACCEPT INPUT AS STRINGS
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //PARSING THE INPUT FROM STRING TO A 32-BIT INTEGER
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
                println!("The secret number is: {}", secret_number);
                break;
            }
        }
    }
}
