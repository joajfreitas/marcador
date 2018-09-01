#!/usr/bin/env python3

"""Bookmarks.

Usage:
    bookmarks add <file> <url> [tags]...
    bookmarks print <file>
    bookmarks rm <file> <id>
    bookmarks find <file> <id>
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

def write_json(dictionary, file):
    with open(file, 'w') as file:
        file.write(json.dumps(dictionary, indent=2))


def open_url_cmd(args, books):
    open_url(books, int(args['<id>']))

def open_url(books, id):
    from webbrowser import open
    open(books[id]['url'])


def add(args, books):
    tags = args.get('<tags>', [])
    url = args['<url>']
    file = args['<file>']


    id = len(books)
    row = {'id' : id, 'url' : url, 'tags' : tags}

    books.append(row)

    write_json(books, file)

def get_book(id, url, tags):
    return "{0}. {1} {2}".format(id, url, str(tags).replace(' ', '').replace("'", "")[1:-1])
    return s

def get_books(books):
    for row in books:
        yield get_book(row['id'], row['url'], row['tags'])

def print_books(args, books):
    for line in get_books(books):
        print(line)

def remove(args, books):
    id = int(args['<id>'])
    file = args['<file>']

    books.pop(id)
    for i, row in enumerate(books[id:]):
        row['id'] = id+i

    write_json(books, file)

def find(args, books):
    id = int(args['<id>'])
    print(books[id]['url'])

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
    for row in books:
        for t in row['tags']:
            if t == tag:
                print_book(row['id'], row['url'], row['tags'])

def tag_list(args, books):
    tags = [tag for row in books for tag in row['tags']]
    for tag in set(tags):
        print(tag)

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

def read_db(file):
    with open(file) as f:
        return json.loads(f.read())

function_map = {
    'add' : add,
    'print' : print_books,
    'rm' : remove,
    'find' : find,
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

    if not os.path.isfile(books_file):
        with open(books_file, "w") as file:
            file.write("[]")

    books = read_db(books_file)

    for name, function in function_map.items():
        if arguments[name]:
            function(arguments, books)
