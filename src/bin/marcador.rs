use clap::{Parser, Subcommand};

use marcador::config::Config;
use marcador::rofi_interface::command_rofi;
use marcador::{BookmarkProxy, LocalProxy, RemoteProxy};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    db: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Rofi,
    Add {
        url: String,
        description: String,
        tags: Vec<String>,
    },
    List,
}

fn get_proxy(host: Option<String>, db: Option<String>) -> Result<Box<dyn BookmarkProxy>, String> {
    if let Some(db) = db {
        Ok(Box::new(LocalProxy::new(&db)))
    } else if let Some(host) = host {
        Ok(Box::new(RemoteProxy::new(&host)))
    } else {
        Err("You must provide either a --host or --db flag".to_string())
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let mut config = Config::read().ok_or("Failed to read config".to_string())?;

    config.set_host(&cli.host);
    config.set_db(&cli.db);

    let proxy = get_proxy(config.host, config.db)?;
    match cli.command {
        Commands::Rofi => command_rofi(&*proxy),
        Commands::Add {
            url,
            description,
            tags,
        } => proxy.add(&url, &description, tags),
        Commands::List => {
            for bookmark in proxy.bookmarks()? {
                println!("{}\n", bookmark);
            }
            Ok(())
        }
    }?;

    Ok(())
}
