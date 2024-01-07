// Copyright 2024 João FreitasA
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
use crate::bookmark_proxy::BookmarkProxy;

use crate::server::{AddParams, DeleteParams};

pub struct RemoteProxy {
    bookmark_endpoint: String,
    list_endpoint: String,
    add_endpoint: String,
    delete_endpoint: String,
    update_description_endpoint: String,
    update_url_endpoint: String,
    update_tags_endpoint: String,
}

impl RemoteProxy {
    pub fn new(url: &str) -> Self {
        Self {
            bookmark_endpoint: url.to_string() + "/bookmark",
            list_endpoint: url.to_string() + "/list",
            add_endpoint: url.to_string() + "/add",
            delete_endpoint: url.to_string() + "/delete",
            update_description_endpoint: url.to_string() + "/update_description",
            update_url_endpoint: url.to_string() + "/update_url",
            update_tags_endpoint: url.to_string() + "/update_tags",
        }
    }
}

impl BookmarkProxy for RemoteProxy {
    fn bookmark(&self, id: i32) -> Result<Bookmark, String> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&self.bookmark_endpoint)
            .json(&id)
            .send()
            .map_err(|_| "Failed to send get request")?;

        response
            .json::<Bookmark>()
            .map_err(|err| format!("{}", err))
    }

    fn bookmarks(&self) -> Result<Vec<Bookmark>, String> {
        Ok(reqwest::blocking::get(&self.list_endpoint)
            .map_err(|_| "Failed to get web resource")?
            .json::<Vec<Bookmark>>()
            .map_err(|_| "Failed to parse json")?)
    }

    fn add(&self, link: &str, desc: &str, _tags: Vec<String>) -> Result<(), String> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.add_endpoint)
            .json(&AddParams {
                url: link.to_string(),
                description: desc.to_string(),
            })
            .send()
            .map_err(|_| "Failed to send post request")?;

        Ok(())
    }
    fn delete(&self, identifier: i32) -> Result<(), String> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.delete_endpoint)
            .json(&DeleteParams { id: identifier })
            .send()
            .map_err(|_| "Failed to send post request")?;

        Ok(())
    }

    fn update_description(&self, id: i32, description: &str) -> Result<(), String> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.update_description_endpoint)
            .json(&(id, description))
            .send()
            .map_err(|_| "Failed to send post request")?;

        Ok(())
    }

    fn update_url(&self, id: i32, url: &str) -> Result<(), String> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.update_url_endpoint)
            .json(&(id, url))
            .send()
            .map_err(|_| "Failed to send post request")?;

        Ok(())
    }
    fn update_tags(&self, id: i32, tags: &[String]) -> Result<(), String> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.update_tags_endpoint)
            .json(&(id, tags))
            .send()
            .map_err(|_| "Failed to send post request")?;

        Ok(())
    }
}
