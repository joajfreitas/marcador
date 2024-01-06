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

use copypasta::{ClipboardContext, ClipboardProvider};

use crate::bookmark::Bookmark;
use crate::rofi;
use crate::BookmarkProxy;

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
    books: Vec<Bookmark>,
) -> Result<(), String> {
    proxy.delete(books[index].bookmark.id)
}

fn rofi_open(url: &str) -> Result<(), String> {
    open::with(url, "firefox").map_err(|_| "Failed to open url")?;
    Ok(())
}

pub fn command_rofi(proxy: &dyn BookmarkProxy) -> Result<(), String> {
    let bookmarks = proxy.bookmarks()?;

    let books = bookmarks
        .iter()
        .map(|bookmark| bookmark.bookmark.url.to_string())
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
        Ok((0, Some(index))) => rofi_open(&bookmarks[index].bookmark.url),
        Err(_) => Ok(()),
        _ => panic!(),
    }?;

    Ok(())
}
