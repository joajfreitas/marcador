// Copyright 2024 João Freitas
//
// This program is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this
// program. If not, see <https://www.gnu.org/licenses/>.

pub mod bookmark;
pub mod bookmark_proxy;
pub mod config;
pub mod local_proxy;
pub mod models;
pub mod remote_proxy;
pub mod rofi;
pub mod rofi_interface;
pub mod schema;
pub mod server;

use clap::{Parser, Subcommand};

use bookmark_proxy::edit_bookmark;
use bookmark_proxy::BookmarkProxy;
use config::Config;
use local_proxy::LocalProxy;
use remote_proxy::RemoteProxy;
use rofi_interface::command_rofi;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Hostname of marcador server
    #[arg(long)]
    host: Option<String>,
    /// Bookmark batabase path
    #[arg(long)]
    db: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Rofi interface
    Rofi,
    /// Add a new bookmark
    Add {
        /// Bookmark url
        url: String,
        /// Bookmark description
        description: String,
        /// List of bookmark tags
        tags: Vec<String>,
    },
    /// List bookmarks
    List,
    /// Delete bookmark by id
    Delete { index: i32 },
    /// Edit bookmark by id
    Edit { index: i32 },
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

pub fn marcador(cli: Cli) -> Result<(), String> {
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
        Commands::Delete { index } => proxy.delete(index),
        Commands::Edit { index } => {
            edit_bookmark(&*proxy, index, None);
            Ok(())
        }
    }?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(1, 1);
    }
}
