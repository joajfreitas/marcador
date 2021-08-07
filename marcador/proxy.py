from marcador.marcador_lib import Bookmark
import socket
import json

def cmd(name, args):
    return bytes(json.dumps({'cmd': name, 'args': args}), 'utf-8')

class RemoteProxy():
    def __init__(self, addr):
        self.addr = addr
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    def list(self):
        self.sock.sendto(cmd('list', {}), self.addr)
        msg, addr = self.sock.recvfrom(1024)
        for bookmark in json.loads(msg):
            yield Bookmark.load(bookmark)

    def add(self, url):
        self.sock.sendto(cmd('add', {'url': url}), self.addr)

    def add_tag(self, url, tag):
        self.sock.sendto(cmd('tag', {'url': url, 'tag': tag}), self.addr)

    def delete(self, url):
        self.sock.sendto(cmd('delete', {'url': url}), self.addr)







