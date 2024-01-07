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

use crate::bookmark::Bookmark;

pub trait BookmarkProxy {
    fn bookmarks(&self) -> Result<Vec<Bookmark>, String>;
    fn bookmark(&self, id: i32) -> Result<Bookmark, String>;
    fn add(&self, url: &str, description: &str, tags: Vec<String>) -> Result<(), String>;
    fn delete(&self, id: i32) -> Result<(), String>;
    fn update_description(&self, id: i32, descritption: &str) -> Result<(), String>;
    fn update_url(&self, id: i32, url: &str) -> Result<(), String>;
    fn update_tags(&self, id: i32, tags: &[String]) -> Result<(), String>;
}

use itertools::intersperse;

use std::{
    env::{temp_dir, var},
    fs::File,
    io::{Read, Write},
    process::Command,
};

pub fn edit_bookmark(proxy: &dyn BookmarkProxy, id: i32, visual: Option<bool>) {
    let editor = if visual.is_none() || visual == Some(false) {
        var("EDITOR").unwrap()
    } else {
        var("VISUAL").unwrap()
    };

    let split: Vec<String> = editor.split(' ').map(|x| x.to_string()).collect();
    let editor = &split[0];
    let mut args: Vec<String> = split[1..].iter().map(|x| x.to_string()).collect();

    let mut file_path = temp_dir();
    file_path.push("editable");
    let mut file = File::create(&file_path).expect("Could not create file");

    let bookmark = proxy.bookmark(id).unwrap();

    let editable_content = format!(
        "# Description:\n{}\n\n# Url:\n{}\n\n# Tags:\n{}",
        bookmark.bookmark.description,
        bookmark.bookmark.url,
        intersperse(
            bookmark.tags.iter().map(|tag| tag.tag.to_string()),
            ",".to_string()
        )
        .collect::<String>()
    );

    file.write_all(editable_content.as_bytes()).unwrap();

    args.push(file_path.to_str().unwrap().to_string());
    Command::new(editor)
        .args(args)
        .status()
        .expect("Something went wrong");

    let mut editable = String::new();

    File::open(file_path)
        .expect("Could not open file")
        .read_to_string(&mut editable)
        .unwrap();

    let lines = editable.lines().collect::<Vec<&str>>();

    let description = lines[1];
    let url = lines[4];

    proxy.update_url(id, url).unwrap();
    proxy.update_description(id, description).unwrap();
    if lines.len() == 8 {
        let tags = lines[7];
        proxy
            .update_tags(
                id,
                &tags
                    .split(',')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )
            .unwrap();
    }
}
