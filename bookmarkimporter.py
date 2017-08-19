#!/usr/bin/env python

import html.parser
import argparse
import os
from pprint import pprint

class BookmarkHTMLParser(html.parser.HTMLParser):
    def __init__(self):
        self.parser = html.parser.HTMLParser.__init__(self)
        self.bookmarks = {'None': []}
        self.link = ""
        self.h3 = False
        self.tag = None

    def handle_starttag(self, tag, attrs):
        if tag == 'a':
            for attr in attrs:
                if attr[0] == 'href':
                    self.link = attr[1]
        if tag == 'h3':
            self.h3 = True

    def handle_data(self, data):
        if self.h3:
            self.tag = data
            self.bookmarks[data] = self.bookmarks.get(data) or []
            self.h3 = False

    def handle_endtag(self, tag):
        if tag == 'a':
            self.bookmarks[str(self.tag)].append(self.link) 

def parse_args():
    parser = argparse.ArgumentParser(description='Import bookmarks from standard html file to rofi-bookmarks.')
    parser.add_argument('inputfile', metavar='htmlfile' )
    parser.add_argument('outputfile', metavar='bookmarkfile')

    return parser.parse_args()


def main(args):
    if os.path.isfile(args.outputfile):
        with open(args.outputfile,'r') as f:
            last = int(f.readlines()[-1].split('.')[0]) + 1
    else:
        last = 0 

    inf= open(args.inputfile,'r')
    outf = open(args.outputfile,'a')

    p = BookmarkHTMLParser()
    
    p.feed(inf.read())
    
    for k, v in p.bookmarks.items():
        for link in v:
            outf.write(f"{str(last)}. {link} {k}\n")
            last += 1


if __name__ == "__main__":
    main(parse_args())
