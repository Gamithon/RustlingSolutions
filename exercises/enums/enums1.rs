// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
	Quit,//: String::from("QUITING PROGRAM"),
	Echo,//: String::from("PETE REPETE"),
	Move,//: String::from("I'm outa here"),
	ChangeColor,//: String::from("This color is better"),    

// TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
