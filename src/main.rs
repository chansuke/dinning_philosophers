struct Philosopher {
    name: String,
}

fn Philosopher() {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
    
    fn eat(&self) {
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let Philosophers = vec![
        Philosopher::new("Judith Butler");
        Philosopher::new("Gilles Deleuze");
        Philosopher::new("Karl Max");
        Philosopher::new("Emma Goldman");
        Philosopher::new("Michel Foucault");
    ];

    for p in &Philosophers {
        p.eat();
    }
}
