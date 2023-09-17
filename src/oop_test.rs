pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

pub fn test() {
    let selected_box: Box<dyn Draw> = Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    });
    let components: Vec<Box<dyn Draw>> = vec![
        selected_box,
        Box::new(Button {
            //button is Box<dyn Draw> too
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];
    let screen = Screen {
        components: components,
    };

    screen.run();
}
