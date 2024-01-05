use clap::Parser;
use marcador::config::{Config, ServerConfig};
use marcador::server::server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    db: Option<String>,
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    root: Option<String>,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let config = Config::read().ok_or("Failed to read config".to_string())?;

    let mut server_config = if let Some(server_config) = config.server {
        server_config
    } else {
        ServerConfig::default()
    };

    server_config.set_db(&cli.db);
    server_config.set_host(&cli.host);
    server_config.set_port(&cli.port);
    server_config.set_root(&cli.root);

    server(server_config)?;

    Ok(())
}
