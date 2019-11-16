import os
import sqlite3


class Database():
    def __init__(self, filename):
        self.filename = filename
        self.conn = self.open_database(self.filename)
        self.cursor = self.conn.cursor()

    def open_db(self, filename):
        return sqlite3.connect(filename)
    
    def set_default_db(self, filename):
        conn = self.open_db(filename)
        c = conn.cursor()

        c.execute("""CREATE TABLE bookmarks (
            identifier INTEGER PRIMARY KEY, 
            url TEXT, 
            description TEXT,
            count INTEGER)
            """
        )

        c.execute("""CREATE TABLE tags (
            identifier INTEGER PRIMARY KEY, 
            tag TEXT)
            """
        )
        c.execute("""CREATE TABLE bookmarks_tags (
            bookmark REFERENCES bookmarks(identifier), 
            tag REFERENCES tags(identifier))
            """
        )
        conn.commit()

        return conn

    def open_database(self, filename):
        if not os.path.isfile(filename):
            return self.set_default_db(filename)

        return self.open_db(filename)

    def get_bookmarks(self):
        self.cursor.execute("select * from bookmarks")
        bookmarks = self.cursor.fetchall()

        for id, url, desc, count in bookmarks:
            self.cursor.execute(f"""select distinct tags.tag from bookmarks join
            bookmarks_tags on bookmarks.identifier = bookmarks_tags.bookmark join
            tags on bookmarks_tags.tag = tags.identifier where
            bookmarks.url='{url}'""")
            
            tags = []
            _tags = self.cursor.fetchall()
            for _tag in _tags:
                tag = _tag[0]
                tags.append(tag)

            yield id, url, tags

    def open_bookmark(self, id):
        self.cursor.execute(f"select * from bookmarks where identifier='{id}'")

        id, url, desc, count = self.cursor.fetchone()

        count+=1
        self.cursor.execute(f"update bookmarks set count = {count} where identifier='{id}'")
        self.conn.commit()

        import webbrowser
        webbrowser.open(url)

    def add_bookmark(self, url, tags):
        self.cursor.execute(f'insert into bookmarks (url,count) values ("{url}",0)')
        book_id = self.cursor.lastrowid
        for tag in tags:
            self.cursor.execute(f'insert into tags (tag) values ("{tag}")')
            tag_id = self.cursor.lastrowid
            self.cursor.execute(f"""insert into bookmarks_tags (bookmark, tag) values ("{book_id}", "{tag_id}")""")

        self.conn.commit()

    def rm_bookmark(self, id):
        self.cursor.execute(f"delete from bookmarks_tags as bt where bt.bookmark = '{id}'")
        self.cursor.execute(f"delete from bookmarks where identifier = '{id}'")
        self.conn.commit()

    def get_url(self, id):
        if id == 0:
            return None

        self.cursor.execute(f"select * from bookmarks where identifier='{id}'")
        _, url, _, _ = self.cursor.fetchone()
        return url

    def get_bookmark(self, id):
        self.cursor.execute(f"select * from bookmarks where identifier='{id}'")

        id, url, desc, count = self.cursor.fetchone()
        return id, url, desc, count

    #def edit_bookmark(self, id):
    #    row = self.conn[id]
    #    tags = row['tags']
    #    url = row['url']

    #    row = conn[id]
    #    tmp_file = "/tmp/bookmarks.tmp"
    #    with open(tmp_file, "w") as tmp:
    #        tmp.write(url)
    #        tmp.write('\n')

    #        for tag in tags:
    #            tmp.write(tag)
    #            tmp.write('\n')

    #    term = os.path.expandvars("$TERM")
    #    editor = os.path.expandvars("$EDITOR")
    #    call([term, "-e", editor, tmp_file])

    #    with open(tmp_file, "r") as tmp:
    #        output = tmp.read().split('\n')

    #    output = [o for o in output if o != '']

    #    url = output[0]
    #    tags = [tag for tag in output[1:]]

    #    books[id]['url'] = url
    #    books[id]['tags'] = tags

    #    write_json(books, file)

    def bookmark_tag_search(self, tag):
        self.cursor.execute(f"select identifier from tags where tag='{tag}'")
        r = self.cursor.fetchone()
        if r == None:
            return []
        id = r[0]

        self.cursor.execute(f"select bt.bookmark from bookmarks_tags as bt where bt.tag = '{id}'")
        bookmarks = self.cursor.fetchall()

        for _book in bookmarks:
            book = _book[0]
            self.cursor.execute(f"select * from bookmarks where identifier = {book}")
            id, url, desc, count = self.cursor.fetchone()
            yield id, url, desc, count

    def bookmark_tag_list(self):
        self.cursor.execute("select tag from tags")
        tags = self.cursor.fetchall()

        for tag in tags:
            yield tag[0]


def bookmark_to_str(bookmark):
    id, url, tags = bookmark
    return f"{id}, {url}, {tags}"
