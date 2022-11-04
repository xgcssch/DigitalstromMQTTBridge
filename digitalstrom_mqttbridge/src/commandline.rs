use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    /// Url of dSS Server including protocol an optional port
    pub url: String,

    /// Format of log output
    #[arg(
        short,
        long,
        value_enum,
        value_name = "FMT",
        default_value_t = Messageformat::Simple
    )]
    pub message_format: Messageformat,

    /// Sets a custom config file
    //#[arg(short, long, value_name = "FILE")]
    //config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Messageformat {
    /// Output as JSON
    Json,
    /// Simple Message Line
    Simple,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Requests an Authentication Token from the dSS
    RequestApplicationToken {
        /// Application name. Will show up in the dSS UI
        #[arg(short, long, default_value_t = String::from("MQTT Bridge"))]
        application_name: String,
    },
    /// Requests an Authentication Token from the dSS
    Run {
        #[arg(short, long)]
        /// Url of dSS Server including protocol an optional port
        mqttserver: String,

        #[arg(short, long)]
        /// Application Token to authorize to the dSS Server
        application_token: String,
    },
}
