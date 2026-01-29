use std::io;

fn main() {
    println!("Please make a guess");
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");

    println!("You guessed {guess}");
    println!("{}", five());
    for number in (1..10).rev() {
        println!("The number is {}", number)
    };
}

fn five()->u16{
    5
}