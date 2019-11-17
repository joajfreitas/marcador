import pytest
import os

from .marcador_lib import Database

@pytest.fixture
def database():
    test_file = "test_temp"
    db = Database(test_file)

    queries = [
        'insert into bookmarks (url,count) values ("reddit.com", 0);',
        'insert into bookmarks (url,count) values ("new.ycombinator.com", 0);',
        'insert into bookmarks (url,count) values ("facebook.com", 0);',
        'insert into bookmarks (url,count) values ("google.com", 0);',
        'insert into bookmarks (url,count) values ("duckduckgo.com", 0);',
        'insert into bookmarks (url,count) values ("1337x.to", 0);',
        'insert into tags (tag) values ("social media");',
        'insert into tags (tag) values ("personal");',
        'insert into tags (tag) values ("work");',
        'insert into tags (tag) values ("programming");',
        'insert into bookmarks_tags (bookmark, tag) values (1,1);',
        'insert into bookmarks_tags (bookmark, tag) values (1,2);',
        'insert into bookmarks_tags (bookmark, tag) values (2,1);',
        'insert into bookmarks_tags (bookmark, tag) values (2,4);',
        'insert into bookmarks_tags (bookmark, tag) values (3,1);',
        'insert into bookmarks_tags (bookmark, tag) values (4,2);'
    ]

    for querie in queries:
        db.cursor.execute(querie)

    yield db
    os.remove(test_file)


def test_get_bookmarks(database):
    bookmarks = database.get_bookmarks()
    assert(len(list(bookmarks)) == 6)

@pytest.mark.skip(reason="annoying since it opens a new browser tab every time it runs")
def test_open_bookmark(database):
    id, url, desc, count = database.get_bookmark(1)
    assert(count==0)
    database.open_bookmark(1)
    id, url, desc, count = database.get_bookmark(1)
    assert(count==1)

def test_add_bookmark(database):
    database.add_bookmark("example.com", ["example", "test"])
    bookmarks = database.get_bookmarks()
    assert(len(list(bookmarks)) == 7)

def test_rm_bookmark(database):
    database.rm_bookmark(1)
    bookmarks = database.get_bookmarks()
    assert(len(list(bookmarks)) == 5)

def test_rm_bookmark_wrong_index(database):
    database.rm_bookmark(0)
    bookmarks = database.get_bookmarks()
    assert(len(list(bookmarks)) == 6)

def test_get_url(database):
    assert(database.get_url(1)=="reddit.com")

def test_get_url_0_index(database):
    assert(database.get_url(0)==None)

def test_get_bookmark(database):
    id, url, desc, count = database.get_bookmark(1)
    assert(id==1)
    assert(url=="reddit.com")
    assert(desc==None)
    assert(count==0)

def test_bookmark_tag_search_empty(database):
    bookmarks = database.bookmark_tag_search("test_tag")
    assert(len(list(bookmarks)) == 0)

def test_bookmark_tag_search(database):
    bookmarks = database.bookmark_tag_search("social media")
    assert(len(list(bookmarks)) == 3)

def test_get_bookmark_tags(database):
    assert(len(database.get_bookmark_tags(1)) == 2)

def test_get_tag_id(database):
    assert(database.get_tag_id("personal") == 2)

def test_set_bookmark(database):
    database.set_bookmark(1, "example.com", ["example", "test"])
    id, url, desc, count = database.get_bookmark(1)
    tags = database.get_bookmark_tags(1)
    assert(url == "example.com")
    assert(tags[0][0] == "example")
    assert(tags[1][0] == "test")


