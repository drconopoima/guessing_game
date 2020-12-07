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
        let guess: u32 = guess.trim().parse().expect("Please type a number");
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
