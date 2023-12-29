use clap::{CommandFactory, Parser, Subcommand};
use copypasta::{ClipboardContext, ClipboardProvider};

use marcador::models::Bookmarks;
use marcador::rofi;
use marcador::server::server;
use marcador::{BookmarkProxy, LocalProxy, RemoteProxy};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Rofi {
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        db: Option<String>,
    },
    Server {
        db: String,
    },
    Add {
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        db: Option<String>,
        url: String,
        description: String,
    },
}

fn rofi_add(proxy: &dyn BookmarkProxy) -> Result<(), String> {
    let mut ctx = ClipboardContext::new().map_err(|_| "Failed to create clipboard context")?;
    let content = ctx
        .get_contents()
        .map_err(|_| "Failed to get clipboard contents")?;
    let s = rofi::Rofi::new(&[content])
        .prompt("URL")
        .run()
        .map_err(|_| "Adding bookmark aborted")?
        .1
        .unwrap();

    let v: Vec<String> = vec![];
    let description = rofi::Rofi::new(&v)
        .prompt("Description")
        .run()
        .map_err(|_| "Adding description aborted")?
        .1
        .unwrap();
    proxy.add(&s, &description, vec![])
}

fn rofi_delete(
    proxy: &dyn BookmarkProxy,
    index: usize,
    books: Vec<Bookmarks>,
) -> Result<(), String> {
    proxy.delete(books[index].id)
}

fn rofi_open(url: &str) -> Result<(), String> {
    open::with(url, "firefox").map_err(|_| "Failed to open url")?;
    Ok(())
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

fn command_rofi(proxy: &dyn BookmarkProxy) -> Result<(), String> {
    let bookmarks = proxy.bookmarks()?;

    let books = bookmarks
        .iter()
        .map(|x| x.url.to_string())
        .collect::<Vec<String>>();

    let ret = rofi::Rofi::new(&books)
        .kb_custom(1, "Alt+n")
        .kb_custom(2, "Alt+d")
        .prompt("> ")
        .message("<b>Alt+n</b>: Add new bookmark <b>Alt+d</b>: Delete bookmark")
        .run_index();

    match ret {
        Ok((10, _)) => rofi_add(proxy),
        Ok((11, Some(index))) => rofi_delete(proxy, index, bookmarks),
        Ok((0, Some(index))) => rofi_open(&bookmarks[index].url),
        Err(_) => Ok(()),
        _ => panic!(),
    }?;

    Ok(())
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    if let Some(command) = cli.command {
        match command {
            Commands::Rofi { host, db } => command_rofi(&*get_proxy(host, db)?),
            Commands::Server { db } => server(db),
            Commands::Add {
                host,
                db,
                url,
                description,
            } => get_proxy(host, db)?.add(&url, &description, vec![]),
        }?;
    } else {
        cmd.print_help().unwrap();
        return Ok(());
    }

    Ok(())
}
