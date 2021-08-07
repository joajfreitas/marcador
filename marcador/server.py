import click
import json
import socket

from marcador.marcador_lib import get_session, get_db_path, Bookmark

def marcador_list(session, args):
    bookmarks = session.query(Bookmark).all()
    return [{
        'url': bookmark.url,
        'description': bookmark.description,
        'count': bookmark.count,
        'score': bookmark.score
    } for bookmark in bookmarks]

def marcador_add(session, args):
    bookmark = Bookmark(url=args.get('url'))
    session.add(bookmark)
    session.commit()

def marcador_tag(session, args):
    tag = Tag(tag=args.get('tag'))
    self.session.add(tag)
    book_tag = BookmarkTag(url=args.get('url'), tag=args.get('tag'))
    session.add(book_tag)
    session.commit()

def marcador_delete(session, args):
    session.query(Bookmark).filter(Bookmark.url == args.get('url')).delete()
    session.commit()

@click.command()
def server():
    session = get_session(get_db_path())

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(("127.0.0.1", 6003))

    cmds = {
        'list': marcador_list,
        'add': marcador_add,
        'tag': marcador_tag,
        'delete': marcador_delete,
    }

    while True:
        try:
            msg, addr = sock.recvfrom(1024)
            msg = json.loads(msg)
            ret = cmds[msg['cmd']](session, msg['args'])
            sock.sendto(bytes(json.dumps(ret), 'utf-8'), addr)
        except Exception as e:
            continue

