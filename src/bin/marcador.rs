use clap::{CommandFactory, Parser, Subcommand};
use copypasta::{ClipboardContext, ClipboardProvider};

use marcador::models::Bookmarks;
use marcador::rofi;
use marcador::server::server;
use marcador::{BookmarkProxy, RemoteProxy};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Rofi { host: String },
    Server {},
}

fn rofi_add(proxy: &RemoteProxy) -> Result<(), String> {
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

fn rofi_delete(proxy: &RemoteProxy, index: usize, books: Vec<Bookmarks>) -> Result<(), String> {
    proxy.delete(books[index].id)
}

fn rofi_open(url: &str) -> Result<(), String> {
    open::with(url, "firefox").map_err(|_| "Failed to open url")?;
    Ok(())
}

fn command_rofi(host: String) -> Result<(), String> {
    let remote_proxy = RemoteProxy::new(&host);
    let bookmarks = remote_proxy.bookmarks()?;

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
        Ok((10, _)) => rofi_add(&remote_proxy),
        Ok((11, Some(index))) => rofi_delete(&remote_proxy, index, bookmarks),
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
            Commands::Rofi { host } => command_rofi(host),
            Commands::Server {} => server(),
        }?;
    } else {
        cmd.print_help().unwrap();
        return Ok(());
    }

    Ok(())
}
