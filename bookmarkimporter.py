#!/usr/bin/env python

import html.parser
import argparse

class bookmark:
    def __init__(self, link, tag):
        self.link = link
        self.tag = tag

class bookmarkhtmlparser(html.parser.HTMLParser):
    def __init__(self, _list):
        html.parser.HTMLParser.__init__(self)
        self.list = _list
        self.data = ""
        self.link = ""

    def handle_starttag(self, tag, attrs):
        if tag == 'a':
            for attr in attrs:
                if attr[0] == 'href':
                    self.link = attr[1]

    def handle_data(self, data):
        if data.isspace() == False:
            self.data = data

    def handle_endtag(self, tag):
        if tag == 'a':
            self.list.append( bookmark( link = self.link, tag = self.data ) )


def main():
    parser = argparse.ArgumentParser(description='Import bookmarks from standard html file to rofi-bookmarks.')
    parser.add_argument('inputfile', metavar='htmlfile' )
    parser.add_argument('outputfile', metavar='bookmarkfile')

    args = parser.parse_args()

    with open(args.outputfile,'r') as f:
        last = int(f.readlines()[-1].split('.')[0])
        f.close()

    inf= open(args.inputfile,'r')
    outf = open(args.outputfile,'a')

    listofbookmarks = []

    p = bookmarkhtmlparser(listofbookmarks)


    p.feed(inf.read())

    for i in listofbookmarks:
        last = last + 1
        outf.write(str(last) + '. ' + i.link  + ' '+ i.tag.replace(" ",",") + '\n')


if __name__ == "__main__":
    main()
