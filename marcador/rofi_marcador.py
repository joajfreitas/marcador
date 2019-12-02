from .rofi import Rofi
from .marcador_lib import Database, bookmark_to_str
import clipboard


class Bookmark():
    def __init__(self):
        return 

class RofiMarcador():
    def __init__(self, filename):
        self.rofi = Rofi()
        self.db = Database(filename)
    
    def disp_bookmarks(self):
        return [bookmark_to_str(bookmark) for bookmark in self.db.get_bookmarks()]

    def select(self, index):
        self.db.open_bookmark(self.bookmarks[index].split(',')[0])

    def add(self):
        text = clipboard.paste()
        url = self.rofi.text_entry("Bookmark url", stdin_str=text)
        if url == None or len(url) == 0:
            return

        tags = self.rofi.text_entry("Bookmark tags")
        if tags == None or len(tags) == 0:
            return

        tags = tags.split(",")
        self.db.add_bookmark(url, tags)
        return
    
    def delete(self, index):
        i = self.bookmarks[index][0]
        self.db.rm_bookmark(i)
        self.launch()
        return

    def edit(self, index):
        i = self.bookmarks[index][0]
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
        print(self.bookmarks)
        index, key = self.rofi.select("> ", 
                          self.bookmarks, 
                          key1=('Alt+n', "Add new bookmark"), 
                          key2=('Alt+d', "Delete the selected bookmark"),
                          key3=('Alt+e', "Edit the selected bookmark"))
        self.dispatch(index, key)
