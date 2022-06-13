fn main() {
    // Create a String for the bird name
    let bird_name = String::from("Johnny");
    // create a new object with class Bird
    let bird = Bird {name: bird_name, attack: 5};
    // Use print_name method of Bird class
    println!("{} {}", bird.can_fly(), bird.is_animal());
}

// Create struct for Bird
struct Bird {
    name: String,
    attack: u64
}

// Create methods for bird
impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

// Creating a trait for Animal (like an interface on JS)
trait Animal {
    // This function is a prototype which gets completed below in the method implementation
    fn can_fly(&self) -> bool;
    // fnction below just returns if it's an animal
    fn is_animal(&self) -> bool { true }
}

// Implementing an Animal trait on a Bird struct (like JS interface)
impl Animal for Bird {
    // Complete function prototype declared inside Animal trait
    fn can_fly(&self) -> bool { 
        true 
    }
}