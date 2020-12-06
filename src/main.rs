use rand::Rng;
use std::io;
fn main() {
    println!("Enter a new guess:");
    let secret_number: usize = rand::thread_rng().gen_range(1, 101);
    println!("Please type your desired number between 1 and 100: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to retrieve value");
    println!("Your guess is: {}", guess);
}
