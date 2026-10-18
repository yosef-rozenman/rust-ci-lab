use ci_runner_onboarding_lab::total_with_shipping;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: cargo run -- <price-in-cents> [more-prices-in-cents]");
        std::process::exit(2);
    }
    let prices: Vec<u64> = args.iter().map(|s| s.parse::<u64>()).collect::<Result<_, _>>()
        .unwrap_or_else(|_| {
            eprintln!("All prices must be non-negative whole numbers in cents.");
            std::process::exit(2);
        });
    match total_with_shipping(&prices) {
        Ok(total) => println!("Total: {total} cents"),
        Err(message) => {
            eprintln!("Error: {message}");
            std::process::exit(1);
        }
    }
}
