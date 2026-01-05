use std::io;

fn main() {
    println!("Please make a guess");
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");

    println!("You guessed {guess}");
    println!("{}", five())
}

fn five()->u8{
    5
}