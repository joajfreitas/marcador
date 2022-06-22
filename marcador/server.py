import click
import json
import socket
import logging

from marcador.lib import get_db_path
from marcador.json_backend import JsonProxy


class Ok:
    def __init__(self, x):
        self.x = x

    def ok(self):
        return self.x

    def json(self):
        return bytes(json.dumps({"type": "ok", "payload": self.x}), "utf-8")

class Error:
    def __init__(self, error):
        self.error = error

    def error(self):
        return self.error

    def json(self):
        return bytes(json.dumps({"type": "error", "payload": self.error}), "utf-8")

def marcador_list(session, args):
    bookmarks = session.list()
    return Ok([
        {
            "url": bookmark.url,
            "description": bookmark.description,
            "tags": bookmark.tags,
        }
        for bookmark in bookmarks
    ])


def marcador_add(session, args):
    session.add(args.get("url"), args.get("description"), args.get("tags"))
    return Ok(())


def marcador_delete(session, args):
    bookmark = session.delete(arg.get("url"))
    return Ok({
        "url": bookmark.url,
        "description": bookmark.description,
        "tags": bookmark.tags,
    })


@click.command()
@click.option("--hostname", default="0.0.0.0")
@click.option("--port", type=int, default=6003)
def server(hostname, port):
    logger = logging.getLogger()
    logger.setLevel(logging.INFO)

    print(get_db_path())
    session = JsonProxy(get_db_path())

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((hostname, port))

    cmds = {
        "list": marcador_list,
        "add": marcador_add,
        "delete": marcador_delete,
    }

    while True:
        try:
            msg, addr = sock.recvfrom(1024)
            print(msg, addr)
            msg = json.loads(msg)
            ret = cmds[msg["cmd"]](session, msg["args"])
            sock.sendto(ret.json(), addr)
        except Exception as e:
            logging.error(repr(e))
            sock.sendto(Error(repr(e)).json(), addr)
            continue
