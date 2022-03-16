use std::io; // std, like c++

fn main() {
    println!("Guess the number"); // console output

    let mut guess = String::new(); // mutable string 

    let apples = 5; // immutable, for example

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
}
