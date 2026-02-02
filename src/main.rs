#![allow(dead_code)]
pub(crate) mod strings;

use rand::Rng;
use std::{cmp::Ordering, io};
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    guess_game();

    println!("{}", five());
    for number in (1..10).rev() {
        println!("The number is {}", number)
    }

    let example = "Helpokklo World";

    println!("{}", { first_word(example) });

    println!("area of the rectangle is {}", calculate_area(8, 9));

    let rect = Rectangle {
        width: 20,
        length: 20,
    };
    println!("Area of the rectangular struct is {}", {
        rect_calculate_area(&rect)
    });
    println!("Area of the rectangular struct using trait is {}", {
        rect.area()
    })
}

fn five() -> u16 {
    5
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn calculate_area(w: u8, l: u8) -> u8 {
    w * l
}

fn rect_calculate_area(r: &Rectangle) -> u32 {
    let Rectangle { width, length } = r;

    width * length
    // OR
    // r.width * r.length
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            length: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

fn guess_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //parsing guess
    //println!("The secret number is {secret_number}");
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is too small"),
            Ordering::Equal => {
                println!("You guessed right");
                break;
            }
            Ordering::Greater => println!("Guess is too big"),
        };
    }
}
