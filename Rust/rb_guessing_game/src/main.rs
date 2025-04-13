
use std::io;
fn main() {
    println!("🎮 Guessing game!");
    println!("Input your guess:");

    let mut guess = String::new(); // mutable variable of type String ::new is a function fo the String type

    //from the io crate, use the stdin and the read_line() from it
    io::stdin()
     // &mut guess is a reference to the guess variable and since reference are also immutable by default, we use &(reference) mut(mutable) guess(reference variable)
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    println!("You guessed: {}", guess);
}
