use std::io;
use std::io::Write;

// 1. Data Structure (The Greeter)
struct Greeter {
    language: String,
    emoji:    String,
}

impl Greeter {
    fn new(lang: &str, icon: &str) -> Greeter {
        Greeter {
            language: lang.to_string(),
            emoji:    icon.to_string(),
        }
    }

    fn greet(&self, name: &str) {
        println!("\n{} Hello, {}! Welcome to {} {}", self.emoji, name, self.language, self.emoji);
    }
}

// 2. Helper function for headers
fn print_header(title: &str) {
    println!("\n--- {} ---", title.to_uppercase());
}

// 3. The Main Program
fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Moringa AI Capstone · Rust Toolkit         ║");
    println!("╚══════════════════════════════════════════════╝");

    // Concept: Structs & Methods
    let greeter = Greeter::new("Rust", "🦀");

    // Concept: Ownership & Borrowing
    print_header("Ownership");
    let original = String::from("Rust Rules");
    let borrowed = &original; // This is a 'borrow'
    println!("Original: {} | Borrowed: {}", original, borrowed);

    // Concept: Pattern Matching
    print_header("Match Expression");
    let grade_score = 85;
   let feedback = match grade_score {
        90..=100 => "Excellent",
        80..=89  => "Great Job",
        _        => "Keep Learning", // This underscore handles all other numbers
    };
    println!("Score: {} -> {}", grade_score, feedback);

    // Concept: Interactive Input
    print_header("Interactive Input");
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read");
    
    greeter.greet(user_name.trim());
}