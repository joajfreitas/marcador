import os
import sqlite3
from subprocess import call
from pathlib import Path

from sqlalchemy import Column, ForeignKey, Integer, Float, String, func, create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship
from sqlalchemy.orm import sessionmaker
from sqlalchemy.orm.session import Session

Base = declarative_base()

class Bookmark(Base):
    __tablename__ = 'bookmark'
    #identifier = Column(Integer, primary_key=True)
    url = Column(String, primary_key=True)
    description = Column(String)
    count = Column(Integer)
    thumbnail = Column(String)
    score = Column(Float)

    def __repr__(self):
        return f"Bookmark {{{self.url=}, {self.thumbnail=}}}"

class Tag(Base):
    __tablename__ = 'tag'

    #identifier = Column(Integer, primary_key = True)
    tag = Column(String, primary_key=True)

    def __repr__(self):
        return f"Tag {{{self.tag=}}}"

class BookmarkTag(Base):
    __tablename__ = 'bookmark_tag'

    url = Column(String, ForeignKey('bookmark.url'), primary_key=True)
    tag = Column(String, ForeignKey('tag.tag'), primary_key=True)

    def __repr__(self):
        return f"BookmarkTag {{bookmark={self.bookmark},tag={self.tag}}}"


def get_session(db_path: Path) -> Session:
    engine = create_engine("sqlite:///"+str(db_path))
    Base.metadata.create_all(engine)
    Base.metadata.bind = engine
    DBSession = sessionmaker(bind=engine)
    session: Session = DBSession()

    return session

class Database:
    def __init__(self, filename):
        self.filename = filename
        self.conn = self.open_database(self.filename)
        self.cursor = self.conn.cursor()

    def open_db(self, filename):
        return sqlite3.connect(filename)

    def set_default_db(self, filename):
        conn = self.open_db(filename)
        c = conn.cursor()

        c.execute(
            """CREATE TABLE bookmarks (
            identifier INTEGER PRIMARY KEY, 
            url TEXT, 
            description TEXT,
            count INTEGER,
            thumbnail TEXT,
            score REAL)
            """
        )

        c.execute(
            """CREATE TABLE tags (
            identifier INTEGER PRIMARY KEY, 
            tag TEXT)
            """
        )
        c.execute(
            """CREATE TABLE bookmarks_tags (
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

    def get_bookmarks(self, sorted=False):
        self.cursor.execute(
            """select identifier, url, description, thumbnail, count from bookmarks""" + (" order by score DESC" if sorted else "")
        )

        bookmarks = self.cursor.fetchall()

        for id, url, desc, thumbnail, count in bookmarks:
            tags = self.get_bookmark_tags(id)
            tags = [tag for tag, id in tags]

            yield id, url, thumbnail, tags

    def open_bookmark(self, id):
        self.cursor.execute(f"select url, count from bookmarks where identifier='{id}'")

        url, count = self.cursor.fetchone()

        self.hit_url(url)

        import webbrowser

        webbrowser.open(url)

    def add_bookmark(self, url, tags):
        self.cursor.execute(f'insert into bookmarks (url,count,score) values ("{url}",0,1)')
        book_id = self.cursor.lastrowid
        for tag in tags:
            self.cursor.execute(f'insert into tags (tag) values ("{tag}")')
            tag_id = self.cursor.lastrowid
            self.cursor.execute(
                f"insert into bookmarks_tags (bookmark, tag) values ({book_id}, {tag_id})"
            )

        self.conn.commit()

    def rm_bookmark(self, id):
        self.cursor.execute(
            f"delete from bookmarks_tags as bt where bt.bookmark = {id}"
        )
        self.cursor.execute(f"delete from bookmarks where identifier = {id}")
        self.conn.commit()

    def get_url(self, id):
        if id == 0:
            return None

        self.cursor.execute(f"select url from bookmarks where identifier={id}")
        url = self.cursor.fetchone()
        return url

    def get_bookmark(self, id):
        self.cursor.execute(
            f"""select identifier, url, description, thumbnail, count
                from bookmarks where identifier={id}"""
        )

        id, url, desc, thumbnail, count = self.cursor.fetchone()
        return id, url, desc, thumbnail, count

    def set_bookmark(self, id, url, tags):
        self.cursor.execute(f"update bookmarks set url='{url}' where identifier={id}")

        tag_set = self.bookmark_tag_list()
        _tags = [tag for tag in tags if tag not in tag_set]
        for tag in _tags:
            self.cursor.execute(f"insert into tags (tag) values ('{tag}')")

        self.cursor.execute(f"delete from bookmarks_tags as bt where bt.bookmark={id}")

        for tag in tags:
            tag_id = self.get_tag_id(tag)
            self.cursor.execute(
                f"insert into bookmarks_tags as bt values ({id},{tag_id})"
            )

        self.conn.commit()

    def set_thumbnail(self, id, thumbnail):
        self.cursor.execute(
            f"update bookmarks set thumbnail='{thumbnail}' where identifier={id}"
        )
        self.conn.commit()

    def edit_bookmark(self, id):
        id, url, desc, thumbnail, count = self.get_bookmark(id)
        tags = self.get_bookmark_tags(id)

        tmp_file = "/tmp/bookmarks.tmp"
        with open(tmp_file, "w") as tmp:
            tmp.write(url + "\n")

            for tag, tag_id in tags:
                tmp.write(tag + "\n")

        term = os.path.expandvars("$TERM")
        editor = os.path.expandvars("$EDITOR")
        call([term, "-e", editor, tmp_file])

        with open(tmp_file, "r") as tmp:
            lines = tmp.readlines()

        lines = [l.strip("\n") for l in lines if l != ""]

        url = lines[0]
        tags = [tag for tag in lines[1:]]

        self.set_bookmark(id, url, tags)

    def get_bookmark_tags(self, id):
        self.cursor.execute(
            f"""select tags.tag, tags.identifier from
            bookmarks_tags as bt, tags where bt.bookmark={id} and bt.tag = tags.identifier"""
        )
        return list(self.cursor.fetchall())

    def bookmark_tag_search(self, tag):
        self.cursor.execute(f"select identifier from tags where tag='{tag}'")
        r = self.cursor.fetchone()
        if r == None:
            return []
        id = r[0]

        self.cursor.execute(
            f"select bt.bookmark from bookmarks_tags as bt where bt.tag = {id}"
        )
        bookmarks = self.cursor.fetchall()

        for _book in bookmarks:
            book = _book[0]
            self.cursor.execute(
                    f"""select identifier, url, description, count
                    from bookmarks where identifier = {book}""")

            id, url, desc, count = self.cursor.fetchone()
            yield id, url, desc, count

    def bookmark_tag_list(self):
        self.cursor.execute("select tag from tags")
        tags = self.cursor.fetchall()

        for tag in tags:
            yield tag[0]

    def get_tag_id(self, tag):
        self.cursor.execute(f"select identifier from tags where tag='{tag}'")
        r = self.cursor.fetchone()
        return None if r == None else r[0]

    def hit_url(self, url):
        self.cursor.execute(f"select identifier, count, score from bookmarks where url='{url}'")
        id, count, score = self.cursor.fetchone()
        count = int(count)
        count += 1
        score += 1

        self.cursor.execute(f"update bookmarks set score = score*0.95 where identifier<>'{id}'")
        self.cursor.execute(
            f"update bookmarks set count = {count}, score = {score} where identifier='{id}'")

        self.conn.commit()


def bookmark_to_str(bookmark):
    id, url, thumbnail, tags = bookmark
    output = f"{id}, {url} "
    for tag in tags:
        output += f"{tag},"

    output = output[:-1] + "\n"
    return output
