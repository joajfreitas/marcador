#!/usr/bin/env python3

import click
from subprocess import call
import json, os

import requests
from bs4 import BeautifulSoup
import jinja2
import sqlite3

from bottle import route, run, template, post, request, redirect

from pprint import pprint

@click.command(name='open')
@click.argument('filename')
@click.argument('id')
def open_bookmark(filename, id):
    conn = open_database(filename)
    c = conn.cursor()

    c.execute(f"select * from bookmarks where identifier='{id}'")

    id, url, desc, count = c.fetchone()

    count+=1
    c.execute(f"update bookmarks set count = {count} where identifier='{id}'")
    conn.commit()

    from webbrowser import open
    open(url)

@click.command()
@click.argument('filename')
@click.argument('url')
@click.argument('tags')
def add(filename, url, tags):
    conn = open_database(filename)
    c = conn.cursor()

    c.execute(f'insert into bookmarks (url) values ("{url}")')
    book_id = c.lastrowid
    for tag in tags:
        c.execute(f'insert into tags (tag) values ("{tag}")')
        tag_id = c.lastrowid
        c.execute(f"""insert into bookmarks_tags (bookmark, tag, count) values
                ("{book_id}", "{tag_id}", 0)""")

    conn.commit()

@click.command(name='print')
@click.argument('filename')
def print_bookmarks(filename):
    conn = open_database(filename)
    c = conn.cursor()
    
    c.execute("select * from bookmarks")
    bookmarks = c.fetchall()
    
    for id, url, desc, count in bookmarks:
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

@click.command()
@click.argument('filename')
@click.argument('id')
def remove(filename, id):
    conn = open_database(filename)

    c = conn.cursor()

    c.execute(f"delete from bookmarks_tags as bt where bt.bookmark = '{id}'")
    c.execute(f"delete from bookmarks where identifier = '{id}'")

    conn.commit()

@click.command(name='url')
@click.argument('filename')
@click.argument('id')
def get_url(filename, id):
    conn = open_database(filename)
    c = conn.cursor()

    c.execute(f"select * from bookmarks where identifier='{id}'")
    id, url, desc, count = c.fetchone()
    print(url)

@click.command(name='bookmark')
@click.argument('filename')
@click.argument('id')
def get_bookmark(filename, id):
    conn = open_database(filename)
    c = conn.cursor()

    c.execute(f"select * from bookmarks where identifier='{id}'")
    id, url, desc = c.fetchone()
    print(id, url, desc)

@click.command()
@click.argument('filename')
@click.argument('id')
def edit(filename, id):
    conn = open_database(filename)
    row = conn[id]
    tags = row['tags']
    url = row['url']

    row = conn[id]
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

@click.command(name='tag-search')
@click.argument('filename')
@click.argument('tag')
def tag_search(filename, tag):
    conn = open_database(filename)
    c = conn.cursor()
    
    c.execute(f"select identifier from tags where tag='{tag}'")
    id = c.fetchone()[0]
    c.execute(f"select bt.bookmark from bookmarks_tags as bt where bt.tag = '{id}'")
    bookmarks = c.fetchall()

    for _book in bookmarks:
        book = _book[0]
        c.execute(f"select * from bookmarks where identifier = {book}")
        r = c.fetchone()
        print(r[0], r[1], r[2])


@click.command(name='tag-search')
@click.argument('filename')
def tag_list(filename):
    conn = open_database(filename)
    c = conn.cursor()

    c.execute("select tag from tags")
    tags = c.fetchall()

    for tag in tags:
        print(tag[0])

#@click.command(name='update-metadata')
#@click.argument('filename')
#def update_metadata(filename):
#    conn = open_database(filename)
#    for i, row in enumerate(conn):
#        name = row.get('name')
#        url = row['url']
#        if not name:
#            print(url)
#            try:
#                r = requests.get(url)
#            except (requests.exceptions.InvalidSchema, requests.exceptions.SSLError, requests.exceptions.MissingSchema) as e:
#                continue
#            soup = BeautifulSoup(r.text, "lxml")
#            if soup.title:
#                print(soup.title.string)
#                row['name'] = soup.title.string
#            if i%20:
#                write_json(conn, file)
#
#    write_json(conn, file)


@click.command(name='html')
@click.argument('filename')
@click.argument('template')
def html(filename, template):
    conn = open_database(filename)
    c = conn.cursor()
    with open(template) as t:
        template = jinja2.Template(t.read())

    c.execute("select url, description from bookmarks order by count desc");
    bookmarks = c.fetchall()
    
    bookmarks = [(("/redirect?url="+url,desc) if desc else ("/redirect?url="+url, url))  for (url, desc) in bookmarks]

    
    @route('/')
    def index():
        c.execute("select url, description from bookmarks order by count desc");
        bookmarks = c.fetchall()
        
        bookmarks = [(("/redirect?url="+url,desc) if desc else ("/redirect?url="+url, url))  for (url, desc) in bookmarks]

        return template.render(bookmarks=bookmarks)

    @route('/redirect')
    def _redirect():
        url = request.query.url

        c.execute(f"select * from bookmarks where url='{url}'")
        id, url, desc, count = c.fetchone()
        count+=1

        c.execute(f"update bookmarks set count = {count} where identifier='{id}'")
        conn.commit()

        print("redirected")
        redirect(url)
    
    @post('/search')
    def search():
        nonlocal bookmarks
        query = request.forms.query
        c.execute(f"select url, description from bookmarks where description like '%{query}%' or url like '%{query}%' order by count desc") 
        bookmarks = c.fetchall()
        bookmarks = [("/redirect?url="+url,desc) if desc else ("/redirect?url="+url, url) for (url, desc) in bookmarks]
        redirect("/")
    
    run(host='localhost', port=9080)


def set_default_db(books_file):
    conn = open_db(books_file)
    c = conn.cursor()

    c.execute("""CREATE TABLE bookmarks (
        identifier INTEGER PRIMARY KEY, 
        url TEXT, 
        description TEXT,
        count INTEGER)
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

def open_database(filename):
    conn = None

    if not os.path.isfile(filename):
        print("create default")
        conn = set_default_db(filename)

    if not conn:
        conn = open_db(filename)

    return conn

@click.group()
def main():
    pass
    #for name, fn in function_map.items():
    #    if arguments[name]:
    #        function_map[name](arguments,conn)

    #conn.close()


main.add_command(open_bookmark)
main.add_command(add)
main.add_command(print_bookmarks)
main.add_command(remove)
main.add_command(get_url)
main.add_command(get_bookmark)
main.add_command(edit)
main.add_command(tag_search)
#main.add_command(update_metadata)
main.add_command(html)

if __name__ == "__main__":
    main()
