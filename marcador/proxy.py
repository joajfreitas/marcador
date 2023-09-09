import socket
import json
import sys
import requests

from typing import *


class ProxyError(Exception):
    pass


def cmd(name, args):
    return bytes(json.dumps({"cmd": name, "args": args}), "utf-8")


class Bookmark(dict):
    def __init__(self, url, description, tags):
        self.url = url
        self.description = description
        self.tags = tags
        dict.__init__(self, {"url": url, "description": description, "tags": tags})

    def __repr__(self) -> str:
        return f"{self.url}"


class Proxy:
    def list(self) -> List[Bookmark]:
        pass

    def add(self, url: str, description: str, tags: List[str]):
        pass

    def add_tag(self, url: str, tag: str):
        pass

    def delete(self, url: str) -> Bookmark:
        pass


class RemoteProxy(Proxy):
    def __init__(self, addr):
        self.addr = addr

    def get_url(self):
        return "http://" + self.addr[0] + ":" + str(self.addr[1])

    def list(self) -> List[Bookmark]:
        r = requests.get(self.get_url() + "/list")
        msg = r.json()

        if msg["type"] == "error":
            raise ProxyError(msg["payload"])

        return [
            Bookmark(
                bookmark.get("url"), bookmark.get("description"), bookmark.get("tags")
            )
            for bookmark in msg["payload"]
        ]

    def add(self, url: str, description: str, tags: List[str]):
        r = requests.post(
            self.get_url() + "/add",
            data={"url": url, "description": description, "tags": tags},
        )
        ret = r.json()

        if ret["type"] == "error":
            raise ProxyError(ret["payload"])

    def delete(self, url: str) -> Bookmark:
        r = requests.post(self.get_url() + "/delete", data={"url": url})
        ret = r.json()
        if ret["type"] == "error":
            raise ProxyError(ret["payload"])
