use std::io::stdin;

struct Visitor {
    name: String,
    greeting:  String,
}

impl Visitor {
    fn new(name: &str,greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greeting_visitor(&self) {
        println!("{}",self.greeting);
    }
}


fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("failed to read line");
    your_name.trim().to_lowercase()
}


fn main() {
    println!("what is your name?");
    let name = what_is_your_name();
    let visitor_list = [Visitor::new("bert", "hello bert"),Visitor::new("steve", "hello steve"),
    Visitor::new("fred", "hello fred")
    ];
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    
    // 找到玩家是否可以进入 P51
    match known_visitor {
        Some(visitor) => visitor.greeting_visitor(),
        None => println!("not found")
    }
}

