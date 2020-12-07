use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Enter a new guess:");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please type your guess between 1 and 100: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to retrieve value");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };
        if guess < 1 {
            println!("Please input a number equal or higher than 1");
            continue;
        }
        if guess > 100 {
            println!("Please input a number equal or lower than 100");
            continue;
        }
        println!("Your guess is: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
