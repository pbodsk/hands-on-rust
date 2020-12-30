use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("No alcohol for you {}", self.name)
                }
            }
            VisitorAction::Accept => println!("Welcome to the treehouse {}", self.name),
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Sorry {} no access for you", self.name)
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read input");
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Ask", VisitorAction::Probation, 12),
        Visitor::new("Freja", VisitorAction::AcceptWithNote { note: String::from("Note here") }, 16),
        Visitor::new("Pia", VisitorAction::Refuse, 45),
        Visitor::new("Peter", VisitorAction::Accept, 46)
    ];

    loop {
        println!("Hello, what's your name? (leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);
    
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("Hey {}, no access for you buddy", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }           
    }   
    println!("The final list of visitors");
    println!("{:#?}", visitor_list);
}