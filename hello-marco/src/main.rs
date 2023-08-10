use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jithish", about = "Marco Polo game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Jithish", about = "Play Marco Polo game")]
    MarcoPolo {
        #[clap(short, long)] // Removing 'about' attribute here
        name: String,
    },
}

// Function to play Marco Polo game
pub fn marco_polo(name: &str) -> String {
    if name == "marco" {
        "polo".to_string()
    } else {
        format!("Your name is {}", name)
    }
}
// Main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::MarcoPolo { name }) => {
            let result = marco_polo(&name);
            println!("{}", result);
        }
        None => {
            println!("No command was used");
        }
    }
}
