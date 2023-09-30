import click
import toml
import json
import socket
import logging
import appdirs
from pathlib import Path

from .bottle import route, run, request

from marcador.lib import get_db_path
from marcador.json_backend import JsonProxy


class Ok:
    def __init__(self, x):
        self.x = x

    def ok(self):
        return self.x

    def dict(self):
        return {"type": "ok", "payload": self.x}

    def json(self):
        return bytes(json.dumps({"type": "ok", "payload": self.x}), "utf-8")


class Error:
    def __init__(self, error):
        self.error = error

    def error(self):
        return self.error

    def dict(self):
        return {"type": "error", "payload": self.error}

    def json(self):
        return bytes(json.dumps({"type": "error", "payload": self.error}), "utf-8")


def marcador_list(session):
    bookmarks = session.list()
    return Ok(
        [
            {
                "url": bookmark.url,
                "description": bookmark.description,
                "tags": bookmark.tags,
            }
            for bookmark in bookmarks
        ]
    )


def marcador_add(session, url, description, tags):
    session.add(url, description, tags)
    return Ok(())


def marcador_delete(session, url):
    bookmark = session.delete(url)
    return Ok(
        {
            "url": bookmark.url,
            "description": bookmark.description,
            "tags": bookmark.tags,
        }
    )


def load_config_file():
    config_path = Path(appdirs.user_config_dir(appname="marcador")) / "marcador.toml"
    if config_path.is_file():
        with config_path.open() as f:
            config = toml.loads(f.read())

    else:
        config = {}

    return config


def config_cli_overloads(config, cli):
    for key, value in cli.items():
        if value is not None:
            config[key] = value

    return config


def config_defaults(config, defaults):
    for key, value in defaults.items():
        if config.get(key) is None:
            config[key] = value

    return config


@click.command()
@click.option("--hostname")
@click.option("--port", type=int)
@click.option("--root", type=str)
def server(hostname, port):
    config = config_defaults(
        config_cli_overloads(
            load_config_file(), {"hostname": hostname, "port": port, "root": root}
        ),
        {"hostname": "127.0.0.1", "port": 6003, "root": "marcador"},
    )

    session = JsonProxy(get_db_path())

    @route("/")
    def hello():
        return "hello"

    @route("/list")
    def list():
        return marcador_list(session).dict()

    @route("/add", method="POST")
    def add():
        url = request.params.get("url")
        description = request.params.get("description")
        tags = request.params.get("tags")

        if url is None or description is None or tags is None:
            return Error("Expected params were: url, description and tags").dict()
        return marcador_add(
            session, request.params.url, request.params.description, request.params.tags
        ).dict()

    @route("/delete", method="POST")
    def delete():
        url = request.params.get("url")

        if url is None:
            return Error("Expected params were: url").dict()

        return marcador_delete(session, url).dict()

    app = default_app()
    app.mount(config.get("root"), app)
    app.run(host=config.get("hostname"), port=config.get("port"))
