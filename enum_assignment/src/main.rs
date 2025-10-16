enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        println!("Available fruits in the store:");
        for f in &self.fruit {
            match f {
                Fruit::Apple(_) => println!("- Apple"),
                Fruit::Banana(_) => println!("- Banana"),
                Fruit::Tomato(_) => println!("- Tomato"),
            }
        }
        println!();
    }

    fn tell_me_joke(fruit: &Fruit) {
        match fruit {
            Fruit::Apple(j) => println!("Apple joke: {}", j),
            Fruit::Banana(j) => println!("Banana joke: {}", j),
            Fruit::Tomato(j) => println!("Tomato joke: {}", j),
        }
    }
}

fn main() {
    // New jokes (not the provided ones)
    let a = "Why was the apple so grumpy? Because it was a crab apple.".to_string();
    let b = "Why did the banana go to the doctor?Because it wasn't peeling well!".to_string();
    let t = "What did the tomato say to the lettuce? Lettuce be friends.".to_string();

    let fruits = vec![
        Fruit::Apple(a),
        Fruit::Banana(b),
        Fruit::Tomato(t),
    ];

    let grocery_store = Inventory { fruit: fruits };

    grocery_store.available_fruits();

    for f in &grocery_store.fruit {
        Inventory::tell_me_joke(f);
    }
}
