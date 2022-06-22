import socket
import json
import sys

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
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    def list(self) -> List[Bookmark]:
        self.sock.sendto(cmd("list", {}), self.addr)
        msg, addr = self.sock.recvfrom(1024)
        
        msg = json.loads(msg)
        if msg["type"] == "error":
            raise ProxyError(msg["payload"])

        return [
            Bookmark(
                bookmark.get("url"), bookmark.get("description"), bookmark.get("tags")
            )
            for bookmark in msg["payload"]
        ]

    def add(self, url: str, description: str, tags: List[str]):
        ret = self.sock.sendto(
            cmd("add", {"url": url, "description": description, "tags": tags}),
            self.addr,
        )

        if ret["type"] == "error":
            raise ProxyError(ret["payload"])

    def add_tag(self, url: str, tag: str):
        ret = self.sock.sendto(cmd("tag", {"url": url, "tag": tag}), self.addr)
        if ret["type"] == "error":
            raise ProxyError(ret["payload"])

    def delete(self, url: str) -> Bookmark:
        ret = self.sock.sendto(cmd("delete", {"url": url}), self.addr)
        if ret["type"] == "error":
            raise ProxyError(ret["payload"])
