//use rofi;

use marcador2::{BookmarkProxy, RemoteProxy};

fn main() {
    let remote_proxy = RemoteProxy::new("http://127.0.0.1:8080");
    let bookmarks = remote_proxy.bookmarks();

    //let bookmarks = Bookmarks::from_str(&contents);
    //for bookmark in bookmarks.bookmarks() {
    //    println!("{}", bookmark.url());
    //}

    let books = bookmarks.iter().map(|x| format!("{}", x.url)).collect::<Vec<String>>();

    let url = dbg!(rofi::Rofi::new(&books).run());
    //
    ////open::that(url.unwrap()).unwrap();
    //open::with(url.unwrap(), "firefox").unwrap();
}
