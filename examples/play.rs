#[derive(Debug)]
struct Example<T> {
    open_counter: usize,
    data: Vec<T>,
}

impl<T> Example<T> {
    pub fn new() -> Self {
        Self {
            open_counter: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, x: T) {
        self.open_counter += 1;
        self.data.push(x);
    }

    pub fn open_counter(&self) -> usize {
        self.open_counter
    }

    // pub fn eat_self(self) -> bool {
    //     println!("later on the lecture");
    //     true
    // }
}

enum _OneMoreEnum {
    Ein(i32),
    Zwei(u64, Example<u8>),
}

fn main() {
    {
        let mut example = Example::<u8>::new();
        example.push(2);
        example.push(3);
        example.push(5);
        example.push(8);

        assert_eq!(example.open_counter(), 4);

        println!("{:#?}", example);

        let mut x = 2;
        if x == 2 {
            x += 2;
        }

        while x > 0 {
            x -= 1;
            println!("{x}");
        }

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {}
            break counter;
        };

        println!("counter: {result}");

        let age = 21;

        match age {
            0 => println!("I haven't celebrated my birthday yet"),
            n @ 1..=12 => println!("I am a child of age {n}"),
            n @ 13..=19 => println!("I am a teen of age {n}"),
            n => println!("I am an old person of age {n}"),
        }
    }
    let optional = Some(7);
    if let Some(i) = optional {
        println!("{i}");
    }

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    dbg!(slice);
    let slice1 = &slice[..2];
    dbg!(slice1);
}
