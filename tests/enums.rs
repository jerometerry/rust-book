enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x : i32, y: i32 },
    Write(String),
}

enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

#[test]
fn test_enum() {
    let x: Message = Message::Move { x: 3, y: 4 };
    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
}

// won't compile. cannot convert Message to Message:ChangeColor
//fn process_color_change(msg: Message) {
//    let Message::ChangeColor(r, g, b) = msg;
//}

#[test]
fn test_constructor() {
    let m = Message::Write("Hello, world!".to_string());
}

fn foo(x: String) -> Message {
    Message::Write(x)
}

#[test]
fn test_constructor_2() {
    let x = foo("Hello, world".to_string());
}

// illustrates how to use enum Constructors as functions to map a vector of Strings
// into a vector of Messages.
#[test]
fn test_iterators() {
    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}

