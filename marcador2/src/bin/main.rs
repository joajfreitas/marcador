use std::fs;

use rofi;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].clone())
        .expect("Should have been able to read the file");

    
    //let bookmarks = Bookmarks::from_str(&contents);
    //for bookmark in bookmarks.bookmarks() {
    //    println!("{}", bookmark.url());
    //}

    //let books = bookmarks.bookmarks().iter().map(|x| format!("{}", x.url())).collect::<Vec<String>>();

    //let url = dbg!(rofi::Rofi::new(&books).run());
    //
    ////open::that(url.unwrap()).unwrap();
    //open::with(url.unwrap(), "firefox").unwrap();

}
