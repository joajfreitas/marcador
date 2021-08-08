import pytest
import os
from marcador.proxy import LocalProxy
from marcador.lib import get_session, Bookmark

@pytest.fixture
def database():
    test_file = "test_temp"
    session = get_session(test_file)

    session.add(Bookmark(url="reddit.com"))
    session.add(Bookmark(url="news.ycombinator.com"))
    session.add(Bookmark(url="facebook.com"))
    session.add(Bookmark(url="google.com"))
    session.add(Bookmark(url="duckduckgo.com"))
    session.add(Bookmark(url="1337x.com"))

    session.commit()


    yield session
    os.remove(test_file)

@pytest.fixture
def proxy(database):
    return LocalProxy(database)

def test_list(proxy):
    l = list(proxy.list())
    assert(len(l) == 6)

def test_add(proxy):
    assert(len(list(proxy.list())) == 6)
    proxy.add("4chan.org")
    assert(len(list(proxy.list())) == 7)

def test_delete(proxy):
    assert(len(list(proxy.list())) == 6)
    proxy.delete("reddit.com")
    assert(len(list(proxy.list())) == 5)
