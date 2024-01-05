use clap::Parser;
use marcador::server::server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    db: String,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    server(cli.db)?;

    Ok(())
}
