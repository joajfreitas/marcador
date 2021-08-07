from .rofi import Rofi
from .marcador_lib import Database, bookmark_to_str, Bookmark, Tag, BookmarkTag
import clipboard


class RofiMarcador():
    def __init__(self, session):
        self.rofi = Rofi()
        self.session = session

    def disp_bookmarks(self):
        return [bookmark.url for bookmark in self.session.query(Bookmark).order_by(Bookmark.score.desc()).all()]

    def select(self, index):
        from webbrowser import open

        bookmarks = self.session.query(Bookmark).order_by(Bookmark.score.desc()).all()

        open(bookmarks[index].url)

    def add(self):
        text = clipboard.paste()
        url = self.rofi.text_entry("Bookmark url", stdin_str=text)
        if url == None or len(url) == 0:
            return

        tags = self.rofi.text_entry("Bookmark tags")
        if tags == None or len(tags) == 0:
            return

        tags = tags.split(",")

        bookmark = Bookmark(url=url)
        self.session.add(bookmark)

        for tag in tags:
            tag = Tag(tag=tag)
            self.session.add(tag)

            bookmark_tag = BookmarkTag(url=url, tag=tag.tag)
            self.session.add(bookmark_tag)
        
        self.session.commit()
        return

    def delete(self, index):
        bookmarks = self.session.query(Bookmark).all()
        bookmark = bookmarks[index]

        self.session.query(Bookmark).filter(Bookmark.url == bookmark.url).delete()
        self.session.commit()
        return

    def edit(self, index):
        i = self.bookmarks[index].split(',')[0]
        self.db.edit_bookmark(i)
        self.launch()
        return

    def dispatch(self, index, key):
        if key == 0: 
            return self.select(index)
        elif key == 1:
            return self.add()
        elif key == 2:
            return self.delete(index)
        elif key == 3:
            return self.edit(index)

    def launch(self):
        self.bookmarks = self.disp_bookmarks()
        ret = self.rofi.select("> ", 
                          self.bookmarks, 
                          key1=('Alt+n', "Add new bookmark"), 
                          key2=('Alt+d', "Delete the selected bookmark"),
                          key3=('Alt+e', "Edit the selected bookmark"))
        index, key = ret
        self.dispatch(index, key)
