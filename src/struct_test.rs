#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

impl Person {
    fn set_age(&mut self, age: i32) {
        if age >= 100 {
            self.age = 100
        } else {
            self.age = age
        }
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name
    }

    fn set_person(&mut self, other: Person) {
        *self = other
    }
}

pub fn test() {
    let mut p1 = Person {
        name: "mgmg",
        age: 1,
    };

    let heap_person = Box::new(Person {
        name: "heap_person",
        age: 10,
    });

    p1.set_age(500);
    p1.set_name("kyaw kyaw");

    println!("before new person -> {:#?}", p1);

    p1.set_person(Person {
        name: "New Person",
        age: 69,
    });

    println!("after new person -> {:#?}", p1);

    p1.set_person(*heap_person);
}
