from webbrowser import open
from .rofi import Rofi
from .marcador_lib import Bookmark, Tag, BookmarkTag
from .proxy import RemoteProxy
import clipboard


class RofiMarcador():
    def __init__(self, session):
        self.rofi = Rofi()
        self.session = session

        self.proxy = RemoteProxy(("127.0.0.1", 6003))

    def list(self):
        return [bookmark.url for bookmark in self.proxy.list()]

    def select(self, index):
        bookmarks = list(self.proxy.list())
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

        self.proxy.add(url)

        for tag in tags:
            self.proxy.add_tag(url, tag)

        return

    def delete(self, index):
        bookmark = list(self.proxy.list())[index]
        self.proxy.delete(bookmark.url)

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
        self.bookmarks = self.list()
        ret = self.rofi.select("> ", 
                          self.bookmarks, 
                          key1=('Alt+n', "Add new bookmark"), 
                          key2=('Alt+d', "Delete the selected bookmark"),
                          key3=('Alt+e', "Edit the selected bookmark"))
        index, key = ret
        self.dispatch(index, key)
