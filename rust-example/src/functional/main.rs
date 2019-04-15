struct Counter {
    count: u32,
}

impl Counter {
    fn new(id: u16) -> Counter {
        Counter { count: 0 }
    }
}

enum Animals {
    Dog,
    Cat(i8),
    Lion { age: i8, name: &'static str },
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 12 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    let sum: u32 = Counter::new(7)
        .zip(Counter::new(8).skip(2))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        // .fold(0, |acc, x| acc + x);
        .sum();

    println!("sum: {}", sum);
    println!("\n");

    use Animals::*;

    // let animal: Animals = Dog;
    // let animal = Cat(4);
    let animal = Lion{name: "Jon", age: 27};

    if let Cat(x) = animal {
        println!("Ramain live: {}", x);
    }

    match animal {
        Dog => println!("Hau"),
        Cat(n) => println!("Ramain live: {}", n),
        Lion { age, name } => println!("The king {} is {} years old", name, age),
    }
}
