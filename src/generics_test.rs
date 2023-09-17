struct Point<T> {
    x: T,
    y: T,
}

struct Ram {
    a: i32,
    b: i32,
}

impl Point<i32> {
    fn add(&self) -> i32 {
        self.x + self.y
    }
}



impl Cartisian for Point<i32> {
    fn show_x(&self) -> i32 {
        self.x
    }

    fn show_y(&self) -> i32 {
        self.y
    }

    fn add(&self) -> i32 {
        self.x + self.y
    }
}

impl Cartisian for Ram {
    fn show_x(&self) -> i32 {
        self.a
    }

    fn show_y(&self) -> i32 {
        self.b
    }

    fn add(&self) -> i32 {
        self.a + self.b
    }
}

trait Cartisian {
    fn show_x(&self) -> i32;
    fn show_y(&self) -> i32;
    fn add(&self) -> i32;
}

fn accept_cartisian(c: impl Cartisian) -> impl Cartisian {
    c
}

pub fn test() {
    let p = accept_cartisian(Point { x: 1, y: 2 });
    let ram = accept_cartisian(Ram { a: 1, b: 3 });

    let z = p.add();

    let x = p.show_x();
    let y = p.show_y();

    ram.show_x();
    ram.show_y();

    println!("{x} {y}");
}
