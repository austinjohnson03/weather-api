use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "geocli",
    about = "Look up location and weather data",
    allow_negative_numbers = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Location(InputArgs),
    Weather(InputArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InputArgs {
    #[command(flatten)]
    pub coords: Option<CoordInput>,

    #[command(flatten)]
    pub place: Option<PlaceInput>,
}

#[derive(Debug, Args)]
pub struct CoordInput {
    /// Latitiude
    #[arg(long, requires = "lon")]
    pub lat: Option<f64>,

    /// Longitude
    #[arg(long, requires = "lat")]
    pub lon: Option<f64>,
}

#[derive(Debug, Args)]
pub struct PlaceInput {
    /// City name
    #[arg(long, requires = "country")]
    pub city: Option<String>,

    /// Country name or ISO 3166-1 A-2 Code
    #[arg(long, requires = "city")]
    pub country: Option<String>,

    /// State or region (optional)
    pub state: Option<String>,
}
