use std::io;

fn main() {
    println!("Guess the number!");

    println!("please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("faild to read line");

    println!("you guesed: {}", guess);
}
