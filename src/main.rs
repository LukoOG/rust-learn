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
    }

    let example = "Helpokklo World".to_string();

    println!("{}", {first_word(&example)});
}

fn five() -> u16 {
    5
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
