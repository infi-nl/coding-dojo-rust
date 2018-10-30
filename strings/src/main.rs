fn main() {
    let greeting = greet("John Doe");
    println!("{}", greeting);
    // Prints: Good morning John Doe
}

fn greet(name: &str) -> String {
    format!("Good morning {}", name)
}
