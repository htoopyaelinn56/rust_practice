enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn make_exciting(s: &str) -> String {
    let s2 = s.replace(".", "!");
    let s3 = s2.replace("?", "‽");
    s3
}

pub fn test() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let z = match y {
        Some(v) => v,
        None => 0,
    };

    let m = Message::Quit;

    match m {
        Message::Quit => todo!(),
        Message::Move { x, y } => todo!(),
        Message::Write(_) => todo!(),
        Message::ChangeColor(_, _, _) => todo!(),
    }

    let dice_roll = 9;
    
    let gg = match dice_roll {
        3 => 3,
        7 => 7,
        other => other,
        _ => 3,
    };

    let opt: Option<String> = Some(String::from("Hello world"));

    // opt became &opt
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}
