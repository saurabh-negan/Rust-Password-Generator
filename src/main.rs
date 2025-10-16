use clap::Parser;
use rand::{rng, Rng};

/// 🔐 Simple password generator CLI tool
#[derive(Parser, Debug)]
#[command(author, version, about = "Generate random passwords easily")]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Include symbols like !@#$%^&*()
    #[arg(short, long, default_value_t = false)]
    symbols: bool,
}

fn main() {
    let args = Args::parse();

    let mut rng = rng(); // modern replacement for thread_rng()

    // Define base charset
    let mut charset = Vec::new();
    charset.extend('a'..='z');
    charset.extend('A'..='Z');
    charset.extend('0'..='9');

    // Optionally add symbols
    if args.symbols {
        charset.extend("!@#$%^&*()_-+=[]{};:,.<>?/|".chars());
    }

    // Generate random password
    let password: String = (0..args.length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx]
        })
        .collect();

    println!("\n🔐 Generated Password: {}\n", password);
}
