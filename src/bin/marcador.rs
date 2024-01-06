use clap::Parser;
use marcador::{marcador, Cli};

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    marcador(cli)
}
