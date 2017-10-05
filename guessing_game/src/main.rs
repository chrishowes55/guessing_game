use std::io;

fn main() {
    println!("Guess the number please");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {}", guess);

    //got up to Using a Crate to Get More Functionality\\
}
