use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "geocli", about = "Look up location and weather data")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Location(InputArgs),
    Weather(InputArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct InputArgs {
    #[command(flatten)]
    coords: Option<CoordInput>,

    #[command(flatten)]
    place: Option<PlaceInput>,
}

#[derive(Debug, Args)]
struct CoordInput {
    /// Latitiude
    #[arg(long, requires = "lon")]
    lat: Option<f64>,

    /// Longitude
    #[arg(long, requires = "lat")]
    lon: Option<f64>,
}

#[derive(Debug, Args)]
struct PlaceInput {
    /// City name
    #[arg(long, requires = "country")]
    city: Option<String>,

    /// Country name or ISO 3166-1 A-2 Code
    #[arg(long, requires = "city")]
    country: Option<String>,

    /// State or region (optional)
    state: Option<String>,
}
