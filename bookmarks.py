#!/usr/bin/env python3

"""Bookmarks.

Usage:
    bookmarks add <file> <url> <tags>...
    bookmarks print <file>
    bookmarks rm <file> <id>
    bookmarks url <file> <id>
    bookmarks bookmark <file> <id>
    bookmarks edit <file> <id>
    bookmarks tag-search <file> <tag>
    bookmarks tag-list <file>
    bookmarks update-metadata <file>
    bookmarks html <file> <template> <output>
    bookmarks open <file> <id>

Options:
  -h --help     Show this screen.
  --version     Show version.
"""

#bookmarks add <file> <url> <tags>... 
from docopt import docopt
from subprocess import call
import json, os

import requests
from bs4 import BeautifulSoup
import jinja2
import sqlite3

from pprint import pprint

def open_url_cmd(args, books):
    id = int(args['<id>'])

    c = books.cursor()

    c.execute(f"select * from bookmarks where identifier='{id}'")

    id, url, desc = c.fetchone()

    from webbrowser import open
    open(url)


def add(args, books):
    tags = args.get('<tags>', [])
    url = args['<url>']
    file = args['<file>']

    c = books.cursor()

    c.execute(f'insert into bookmarks (url) values ("{url}")')
    book_id = c.lastrowid
    for tag in tags:
        c.execute(f'insert into tags (tag) values ("{tag}")')
        tag_id = c.lastrowid
        c.execute(f"""insert into bookmarks_tags (bookmark, tag) values
                ("{book_id}", "{tag_id}")""")

    conn.commit()


def get_book(id, url, tags):
    return "{0}. {1} {2}".format(id, url, str(tags).replace(' ', '').replace("'", "")[1:-1])
    return s


def print_books(args, books):
    c = books.cursor()
    
    c.execute("select * from bookmarks")
    bookmarks = c.fetchall()

    for id, url, desc in bookmarks:
        c.execute(f"""select distinct tags.tag from bookmarks join
        bookmarks_tags on bookmarks.identifier = bookmarks_tags.bookmark join
        tags on bookmarks_tags.tag = tags.identifier where
        bookmarks.url='{url}'""")
        
        tags = []
        _tags = c.fetchall()
        for _tag in _tags:
            tag = _tag[0]
            tags.append(tag)

        print(f"{id}, {url} ", end='')

        for tag in tags:
            print(f"{tag},", end='')

        print()

    c.execute("""select distinct bookmarks.url, tags.tag, bookmarks_tags.tag,
    bookmarks_tags.bookmark from bookmarks join bookmarks_tags on
    bookmarks.identifier = bookmarks_tags.bookmark join tags on
    bookmarks_tags.tag = tags.identifier""")

    result = c.fetchall()

    #pprint(result)

def remove(args, books):
    id = int(args['<id>'])
    file = args['<file>']
    
    c = books.cursor()

    c.execute(f"delete from bookmarks_tags as bt where bt.bookmark = '{id}'")
    c.execute(f"delete from bookmarks where identifier = '{id}'")

    books.commit()


def get_url(args, books):
    id = int(args['<id>'])
    c = books.cursor()

    c.execute(f"select * from bookmarks where identifier='{id}'")
    id, url, desc = c.fetchone()
    print(url)


def get_bookmark(args, books):
    id = int(args['<id>'])
    c = books.cursor()

    c.execute(f"select * from bookmarks where identifier='{id}'")
    id, url, desc = c.fetchone()
    print(id, url, desc)

def edit(args, books):
    id = int(args['<id>'])
    file = args['<file>']
    row = books[id]
    tags = row['tags']
    url = row['url']

    row = books[id]
    tmp_file = "/tmp/bookmarks.tmp"
    with open(tmp_file, "w") as tmp:
        tmp.write(url)
        tmp.write('\n')

        for tag in tags:
            tmp.write(tag)
            tmp.write('\n')

    term = os.path.expandvars("$TERM")
    editor = os.path.expandvars("$EDITOR")
    call([term, "-e", editor, tmp_file])

    with open(tmp_file, "r") as tmp:
        output = tmp.read().split('\n')

    output = [o for o in output if o != '']

    url = output[0]
    tags = [tag for tag in output[1:]]

    books[id]['url'] = url
    books[id]['tags'] = tags

    write_json(books, file)


def tag_search(args, books):
    tag = args['<tag>']
    
    c = books.cursor()
    
    c.execute(f"select identifier from tags where tag='{tag}'")
    id = c.fetchone()[0]
    c.execute(f"select bt.bookmark from bookmarks_tags as bt where bt.tag = {id}")
    bookmarks = c.fetchall()
    for book in bookmarks:
        print(book[0])

    #c.execute("select * from bookmarks_tags where 
#    for row in books:
#        for t in row['tags']:
#            if t == tag:
#                print_book(row['id'], row['url'], row['tags'])



def tag_list(args, books):
    c = books.cursor()

    c.execute("select tag from tags")
    tags = c.fetchall()

    for tag in tags:
        print(tag[0])


def update_metadata(args, books):
    file = args['<file>']
    for i, row in enumerate(books):
        name = row.get('name')
        url = row['url']
        if not name:
            print(url)
            try:
                r = requests.get(url)
            except (requests.exceptions.InvalidSchema, requests.exceptions.SSLError, requests.exceptions.MissingSchema) as e:
                continue
            soup = BeautifulSoup(r.text, "lxml")
            if soup.title:
                print(soup.title.string)
                row['name'] = soup.title.string
            if i%20:
                write_json(books, file)

    write_json(books, file)

def html(args, books):
    template = args['<template>']
    file = args['<file>']
    output = args['<output>']

    bookmarks = [(row['id'], row['url'], row['tags'], row.get('name') or row['url']) for row in books]
    with open(template) as t:
        template = jinja2.Template(t.read())

    with open(output, 'w') as file:
        file.write(template.render(bookmarks=bookmarks))

def do_nothing(args, books):
    return

def read_db(file):
    print("read file")
    with open(file) as f:
        return json.loads(f.read())


def set_default_db(books_file):
    conn = open_db(books_file)
    c = conn.cursor()

    c.execute("""CREATE TABLE bookmarks (
        identifier INTEGER PRIMARY KEY, 
        url TEXT, 
        description TEXT)
        """
    )

    c.execute("""CREATE TABLE tags (
        identifier INTEGER PRIMARY KEY, 
        tag TEXT)
        """
    )
    c.execute("""CREATE TABLE bookmarks_tags (
        bookmark REFERENCES bookmarks(identifier), 
        tag REFERENCES tags(identifier))
        """
    )
    conn.commit()

    return conn

def open_db(books_file):
    return sqlite3.connect(books_file)

function_map = {
    'add' : add,
    'print' : print_books,
    'rm' : remove,
    'url' : get_url,
    'bookmark' : get_bookmark,
    'edit' : edit,
    'tag-search' : tag_search,
    'tag-list' : tag_list,
    'update-metadata' : update_metadata,
    'html' : html,
    'open': open_url_cmd,
}

if __name__ == "__main__":
    arguments = docopt(__doc__, version='Bookmarks 0.1')
    books_file = arguments['<file>']

    conn = None

    if not os.path.isfile(books_file):
        print("create default")
        conn = set_default_db(books_file)
    
    if not conn:
        conn = open_db(books_file)
   
    for name, fn in function_map.items():
        if arguments[name]:
            function_map[name](arguments,conn)

    conn.close()
