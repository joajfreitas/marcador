#!/usr/bin/env python3

import click
import json, os

import requests
from bs4 import BeautifulSoup
import jinja2
import sqlite3

from bottle import route, run, template, post, request, redirect

from pprint import pprint

from .marcador_lib import Database, bookmark_to_str
from .rofi_marcador import RofiMarcador


@click.command(name='open')
@click.argument('filename')
@click.argument('id')
def open_bookmark_cmd(filename, id):
    db = Database(filename)
    db.open_bookmark(id)

@click.command()
@click.argument('filename')
@click.argument('url')
@click.argument('tags')
def add(filename, url, tags):
    db = Database(filename)
    db.add_bookmark(url, tags)

@click.command(name='print')
@click.argument('filename')
def print_bookmarks(filename):
    db = Database(filename)
    bookmarks = db.get_bookmarks()
    
    output = ""
    for bookmark in bookmarks:
        output += bookmark_to_str(bookmark)
    
    print(output[:-1])


@click.command()
@click.argument('filename')
@click.argument('id')
def remove(filename, id):
    db = Database(filename)
    db.rm_bookmark(id)


@click.command(name='url')
@click.argument('filename')
@click.argument('id')
def get_url(filename, id):
    db = Database(filename)
    print(db.rm_bookmark(id))

@click.command(name='bookmark')
@click.argument('filename')
@click.argument('id')
def get_bookmark(filename, id):
    db = Database(filename)
    print(db.get_bookmark(id))


@click.command()
@click.argument('filename')
@click.argument('id')
def edit(filename, id):
    db = Database(filename)
    db.edit_bookmark(id)

@click.command(name='tag-search')
@click.argument('filename')
@click.argument('tag')
def tag_search(filename, tag):
    db = Database(filename)
    for id, url, desc in db.bookmark_tag_search(tag):
        print(id, url, desc)

@click.command(name='tag-search')
@click.argument('filename')
def tag_list(filename):
    db = Database(filename)
    for tag in db.bookmark_tag_list():
        print(tag)

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
    db = Database(filename)

    with open(template) as t:
        template = jinja2.Template(t.read())

    db.cursor.execute("select url, description from bookmarks order by count desc");
    bookmarks = db.cursor.fetchall()
    
    bookmarks = [(("/redirect?url="+url,desc) if desc else ("/redirect?url="+url, url))  for (url, desc) in bookmarks]

    
    @route('/')
    def index():
        db.cursor.execute("select url, description from bookmarks order by count desc");
        bookmarks = db.cursor.fetchall()
        
        bookmarks = [(("/redirect?url="+url,desc) if desc else ("/redirect?url="+url, url))  for (url, desc) in bookmarks]

        return template.render(bookmarks=bookmarks)

    @route('/redirect')
    def _redirect():
        url = request.query.url

        db.cursor.execute(f"select * from bookmarks where url='{url}'")
        id, url, desc, count = db.cursor.fetchone()
        count+=1

        db.cursor.execute(f"update bookmarks set count = {count} where identifier='{id}'")
        db.conn.commit()

        print("redirected")
        redirect(url)
    
    @post('/search')
    def search():
        nonlocal bookmarks
        query = request.forms.query
        db.cursor.execute(f"select url, description from bookmarks where description like '%{query}%' or url like '%{query}%' order by count desc") 
        bookmarks = db.cursor.fetchall()
        bookmarks = [("/redirect?url="+url,desc) if desc else ("/redirect?url="+url, url) for (url, desc) in bookmarks]
        redirect("/")
    
    run(host='localhost', port=9080)



@click.command(name="rofi")
@click.argument('filename')
def rofi_launch(filename):
    rm = RofiMarcador(filename)
    rm.launch()

@click.group()
def main():
    pass

main.add_command(open_bookmark_cmd)
main.add_command(add)
main.add_command(print_bookmarks)
main.add_command(remove)
main.add_command(get_url)
main.add_command(get_bookmark)
main.add_command(edit)
main.add_command(tag_search)
#main.add_command(update_metadata)
main.add_command(html)
main.add_command(rofi_launch)

if __name__ == "__main__":
    main()
