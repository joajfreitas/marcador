#!/usr/bin/env python3

import click
import os

import jinja2

import bottle
from bottle import route, run, post, request, redirect, static_file

from selenium import webdriver
import pathlib

from .marcador_lib import Database, bookmark_to_str
from .rofi_marcador import RofiMarcador

from . import version

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


@click.command(name='html')
@click.argument('filename')
@click.argument('template')
def html(filename, template):
    db = Database(filename)

    with open(template) as t:
        template = jinja2.Template(t.read())

    db.cursor.execute(
        "select url, description from bookmarks order by count desc")
    bookmarks = db.cursor.fetchall()

    bookmarks = [(("/redirect?url=" + url, desc) if desc else
                  ("/redirect?url=" + url, url)) for (url, desc) in bookmarks]

    bottle.TEMPLATE_PATH.insert(0, ".marcador_thumbnail/")

    @route('/')
    def index():
        db.cursor.execute(
            """select url, description, thumbnail 
            from bookmarks order by score desc"""
        )
        bookmarks = db.cursor.fetchall()

        bookmarks = [(("/redirect?url=" + url, desc, thumb) if desc else ("/redirect?url="+url, url, thumb))
                     for (url, desc, thumb) in bookmarks]

        return template.render(bookmarks=bookmarks)

    @route('/redirect')
    def _redirect():
        url = request.query.url
        db.hit_url(url)
        redirect(url)

    @post('/search')
    def search():
        nonlocal bookmarks
        query = request.forms.query
        db.cursor.execute(
            f"select url, description from bookmarks where description like '%{query}%' or url like '%{query}%' order by count desc"
        )
        bookmarks = db.cursor.fetchall()
        bookmarks = [("/redirect?url=" + url, desc) if desc else
                     ("/redirect?url=" + url, url)
                     for (url, desc) in bookmarks]
        redirect("/")

    @route('/.marcador_thumbnail/<filepath:path>')
    def file_stac(filepath):
        return static_file(filepath, root="./.marcador_thumbnail")

    run(host='localhost', port=9080)


@click.command(name="gen_thumbnails")
@click.argument('filename')
def gen_thumbnails(filename):
    db = Database(filename)
    bookmarks = db.get_bookmarks()

    path = ".marcador_thumbnail/"
    pathlib.Path(path).mkdir(parents=True, exist_ok=True)

    files = os.listdir(path)

    driver = webdriver.Chrome("chromedriver")
    for bookmark in bookmarks:
        id, url, thumbnail, tags = bookmark
        if f"{id}" + ".png" in files:
            print("skiped", id)
            continue
        print("getting thumbnail for:", id)
        try:
            driver.get(url)
            thumbnail_path = path + f"{id}" + ".png"
            driver.save_screenshot(thumbnail_path)
            db.set_thumbnail(id, thumbnail_path)
        except Exception as e:
            print("Error: " + e)
            continue


@click.command(name="rofi")
@click.argument('filename')
def rofi_launch(filename):
    rm = RofiMarcador(filename)
    rm.launch()


@click.group(invoke_without_command=True)
@click.version_option(version)
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
main.add_command(html)
main.add_command(gen_thumbnails)
main.add_command(rofi_launch)

if __name__ == "__main__":
    main()
