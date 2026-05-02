use std::io::{self, Write};

// =============================================
// 1. STRUCTS & IMPLEMENTATION
// =============================================

#[derive(Debug)]
struct Greeter {
    language: String,
    emoji: String,
}

impl Greeter {
    /// Creates a new Greeter instance
    fn new(lang: &str, icon: &str) -> Self {
        Greeter {
            language: lang.to_string(),
            emoji: icon.to_string(),
        }
    }

    /// Prints a greeting message with emoji
    fn greet(&self, name: &str) {
        println!(
            "\n{} Hello, {}! Welcome to {} {}",
            self.emoji, name, self.language, self.emoji
        );
    }
}

// =============================================
// 2. STUDENT STRUCT
// =============================================

#[derive(Debug)]
struct Student {
    name: String,   // Student's name
    score: u8,      // Score between 0-100
}

// =============================================
// 3. HELPER FUNCTION
// =============================================

/// Prints a nicely formatted header for sections/menus
fn print_header(title: &str) {
    println!("\n{}", "─".repeat(55));
    println!("{:^55}", title.to_uppercase());
    println!("{}", "─".repeat(55));
}

// =============================================
// 4. MAIN FUNCTION
// =============================================

fn main() {
    // Display program banner
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║     Moringa AI Capstone · Rust Toolkit v2.0         ║");
    println!("╚══════════════════════════════════════════════════════╝");

    // Initialize components
    let greeter = Greeter::new("Rust", "🦀");
    let mut students: Vec<Student> = Vec::new(); // Empty list of students

    println!("\nWelcome to the Rust Learning Toolkit!");

    // Main program loop - keeps running until user quits
    loop {
        print_header("Main Menu");

        // Display menu options
        println!("1. Greet Someone");
        println!("2. Add Student Score");
        println!("3. List All Students");
        println!("4. Show Rust Fact");
        println!("5. Quit");

        // Get user input safely
        print!("\nEnter your choice (1-5): ");
        io::stdout().flush().unwrap(); // Ensures prompt appears before input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim(); // Remove newline character

        // Match expression - Rust's powerful switch statement
        match choice {
            "1" => {
                // Option 1: Greeting
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                greeter.greet(name.trim());
            }

            "2" => {
                // Option 2: Add student with score
                print!("Enter student name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                print!("Enter score (0-100): ");
                io::stdout().flush().unwrap();
                let mut score_str = String::new();
                io::stdin().read_line(&mut score_str).unwrap();

                // Convert string input to number with fallback
                let score: u8 = score_str.trim().parse().unwrap_or(0);

                // Add new student to the vector
                students.push(Student {
                    name: name.trim().to_string(),
                    score,
                });

                println!("✅ Student added successfully!");
            }

            "3" => {
                // Option 3: Display all students
                print_header("Student List");
                if students.is_empty() {
                    println!("No students yet. Add some first!");
                } else {
                    for (i, student) in students.iter().enumerate() {
                        println!("{}. {} → Score: {}", i + 1, student.name, student.score);
                    }
                }
            }

            "4" => {
                // Option 4: Educational fact
                print_header("Rust Random Fact");
                println!("Did you know? Rust's ownership system prevents");
                println!("data races at compile time! 🛡️");
            }

            "5" => {
                // Option 5: Exit the program
                println!("\n👋 Goodbye! Happy Rusting! 🦀");
                break; // Exit the loop
            }

            _ => {
                // Default case for invalid input
                println!("❌ Invalid choice! Please enter a number between 1 and 5.");
            }
        }
    }
}