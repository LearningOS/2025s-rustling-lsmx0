// enums1.rs
//
// No hints this time! Watch and learn how to track down and fix a compiler error
// yourself. Try to solve the exercise without looking at the hints in the code.

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
