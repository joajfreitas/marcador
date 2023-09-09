from serde import Model, fields
import json
import pathlib
import os
from typing import *

from .proxy import Proxy, Bookmark


class Tag(Model):
    tag: fields.Str()


class JsonBookmark(Model):
    url: fields.Str()
    description: fields.Str()
    tags: fields.List(Tag)


class JsonBook(Model):
    bookmarks: fields.List(JsonBookmark)

    def get(self, url: str) -> Optional[Bookmark]:
        for bookmark in self.bookmarks:
            if bookmark.url == url:
                return bookmark

        return None

    def get_index(self, url: str) -> Optional[int]:
        for i, bookmark in enumerate(self.bookmarks):
            if bookmark.url == url:
                return i

        return None


class JsonProxy(Proxy):
    def __init__(self, path: pathlib.Path):
        self.path = path
        if not os.path.exists(path):
            self.book = JsonBook(bookmarks=[])
        else:
            with open(path) as f:
                self.book = JsonBook.from_json(f.read())

    def save(self):
        if not os.path.isdir(self.path.parent):
            os.makedirs(self.path.parent, exist_ok=True)

        with open(self.path, "w") as f:
            f.write(json.dumps(self.book.to_dict()))

    def list(self) -> List[Bookmark]:
        return [
            Bookmark(
                bookmark.url, bookmark.description, [tag.tag for tag in bookmark.tags]
            )
            for bookmark in self.book.bookmarks
        ]

    def add(self, url: str, description: str, tags: List[str]):
        if self.book.get(url) is not None:
            return

        tags = [Tag(tag) for tag in tags]
        self.book.bookmarks.append(JsonBookmark(url, description, tags))
        self.save()

    def add_tag(self, url: str, tag: str):
        index = self.book.bookmarks.get_index(url)

        if tag not in self.book.bookmarks[index].tags:
            self.book.bookmarks[index].tags.append(tag)

        self.save()

    def delete(self, url) -> Bookmark:
        index = self.book.get_index(url)
        bookmark = self.book.bookmarks.pop(index)
        self.save()

        return Bookmark(
            bookmark.url, bookmark.description, [tag.tag for tag in bookmark.tags]
        )


def main():
    bookmark = Bookmark(
        url="www.google.com", description="google", tags=[Tag("search")]
    )
    book = Book([bookmark])
    print(book.to_dict())
    print(json.dumps(bookmark.to_dict()))


if __name__ == "__main__":
    main()
