import click
import json
import socket

from marcador.lib import get_db_path
from marcador.json_backend import JsonProxy


def marcador_list(session, args):
    bookmarks = session.list()
    return [
        {
            "url": bookmark.url,
            "description": bookmark.description,
            "tags": bookmark.tags,
        }
        for bookmark in bookmarks
    ]


def marcador_add(session, args):
    session.add(args.get("url"), args.get("description"), args.get("tags"))


def marcador_delete(session, args):
    session.delete(arg.get("url"))


@click.command()
@click.option("--hostname", default="127.0.0.1")
@click.option("--port", type=int, default=6003)
def server(hostname, port):
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
            msg = json.loads(msg)
            ret = cmds[msg["cmd"]](session, msg["args"])
            sock.sendto(bytes(json.dumps(ret), "utf-8"), addr)
        except Exception as e:
            continue
