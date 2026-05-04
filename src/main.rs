use clap::Parser;
use weather_api::cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Location(args) => {
            println!("Location module: {:?}", args)
        }
        Commands::Weather(args) => {
            println!("Weather module: {:?}", args)
        }
    }
}
