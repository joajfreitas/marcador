#!/usr/bin/env python3

from pprint import pprint

import click
import sys
import json
import logging

from marcador.version import version as marcador_version
from marcador.rofi_marcador import RofiMarcador
from marcador.server import server
from marcador.proxy import RemoteProxy
from marcador.lib import get_db_path
from marcador.json_backend import JsonProxy


def get_proxy(hostname, port):
    if hostname is not None and port is not None:
        return RemoteProxy((hostname, port))
    elif hostname is None and port is None:
        return JsonProxy(get_db_path())
    else:
        logging.error(
            "Using only of hostname and port, both required for remote operation"
        )


@click.command()
@click.argument("url")
@click.argument("description")
@click.argument("tags")
@click.option("--hostname", default=None, help="hostname of the marcador server")
@click.option("--port", default=None, type=int, help="post of the marcador server")
def add(url, description, tags, hostname, port):
    proxy = get_proxy(hostname, port)
    proxy.add(url, description, tags.split(","))


@click.command(name="bookmarks")
@click.option("--hostname", default=None, help="hostname of the marcador server")
@click.option("--port", default=None, type=int, help="post of the marcador server")
@click.option("-j", is_flag=True, default=False, type=bool, help="output json")
def print_bookmarks(hostname, port, j):
    proxy = get_proxy(hostname, port)

    if j:
        print(json.dumps(proxy.list()))
    else:
        for i, bookmark in enumerate(proxy.list()):
            print(bookmark)


@click.command()
@click.argument("url")
@click.option("--hostname", default=None, help="hostname of the marcador server")
@click.option("--port", default=None, type=int, help="post of the marcador server")
def delete(url, hostname, port):
    proxy = get_proxy(hostname, port)
    proxy.delete(url)


@click.command(name="rofi")
@click.option("--hostname", default=None, help="hostname of the marcador server")
@click.option("--port", default=None, type=int, help="post of the marcador server")
def rofi_launch(hostname=None, port=None):
    proxy = get_proxy(hostname, port)

    rm = RofiMarcador(proxy)
    rm.launch()


@click.group(invoke_without_command=True)
@click.option("--version", is_flag=True, default=False)
def main(version):
    if len(sys.argv) == 1:
        print(
            "marcador.\nVersion:", marcador_version, "\nFor usage see marcador --help"
        )
    if version:
        click.echo(marcador_version)

    return


main.add_command(print_bookmarks)
main.add_command(add)
main.add_command(delete)
main.add_command(rofi_launch)
main.add_command(server)

if __name__ == "__main__":
    main()
