#!/usr/bin/env python3

from pathlib import Path
from pprint import pprint

import click
import jinja2
from appdirs import user_data_dir
from selenium import webdriver

from marcador.version import version
from marcador.marcador_lib import (Bookmark, BookmarkTag, Database, Tag,
                                   bookmark_to_str, get_session)
from marcador.rofi_marcador import RofiMarcador


from flask import Flask, jsonify, Response, send_from_directory, render_template, request, redirect
from flask_cors import CORS


def get_user_data_dir():
    appauthor = "joajfreitas"
    appname = "marcador"

    return user_data_dir(appname, appauthor)

def get_db_path():
    return Path(get_user_data_dir()) / Path("marcador.sqlite")

@click.command(name='open')
@click.argument('url')
def open_bookmark_cmd(url):
    session = get_session(get_db_path())
    bookmark = session.query(Bookmark).filter(Bookmark.url == url).one()
    print(bookmark)

    import webbrowser
    webbrowser.open(bookmark.url)

@click.command()
@click.argument('url')
@click.argument('tags', nargs=-1)
def add(url, tags):
    session = get_session(get_db_path())

    bookmark = Bookmark(
            url = url,
            description = "",
            count = 0,
            thumbnail = "",
            score = 0)

    session.add(bookmark)

    for tag in tags:
        tag = Tag(tag=tag)
        session.add(tag)

        bookmark_tag = BookmarkTag(url=url, tag=tag.tag)
        session.add(bookmark_tag)

    session.commit()


@click.command(name='bookmarks')
def print_bookmarks():
    session = get_session(get_db_path())

    bookmarks = session.query(Bookmark).all()
    pprint(bookmarks)

@click.command(name='bookmark')
@click.argument('url')
def print_bookmark(url):
    session = get_session(get_db_path())

    bookmark = session.query(Bookmark).filter(Bookmark.url == url).one()

    pprint(bookmark)
    pprint([bt.tag for bt in session.query(BookmarkTag).filter(BookmarkTag.url == url).all()])


@click.command(name='tags')
def print_tags():
    session = get_session(get_db_path())

    tags = session.query(Tag).all()
    pprint(tags)


@click.command(name='tag')
@click.argument('tag')
def print_tag(tag):
    session = get_session(get_db_path())

    tag = session.query(Tag).filter(Tag.tag == tag).one()
    pprint(tag)

    pprint([bt.url for bt in session.query(BookmarkTag).filter(BookmarkTag.tag == tag.tag).all()])

@click.command()
@click.argument('url')
def delete(url):
    session = get_session(get_db_path())
    session.query(Bookmark).filter(Bookmark.url == url).delete()
    session.query(BookmarkTag).filter(BookmarkTag.url == url).delete()

    session.commit()


@click.command(name='url')
@click.argument('id')
def get_url(id):
    session = get_session(get_db_path())
    print(session.query(Bookmark).filter(Bookmark.identifier==id).one().url)


@click.command(name='bookmark')
@click.argument('id')
def get_bookmark(id):
    session = get_session(get_db_path())
    print(session.query(Bookmark).filter(Bookmark.identifier == id).one())

@click.command()
@click.argument('filename')
@click.argument('id')
def edit(filename, id):
    db = Database(filename)
    db.edit_bookmark(id)

@click.command(name='html')
@click.argument('template')
def html(template):
    app = Flask(__name__)
    app.config.from_object(__name__)

    CORS(app, resources={r'/*': {'origins': '*'}})

    with open(template) as t:
        template = jinja2.Template(t.read())

    @app.route('/')
    def index():
        session = get_session(get_db_path())
        bookmarks = session.query(Bookmark).order_by(Bookmark.score.desc()).all()

        bookmarks = [("/redirect?url=" + book.url, book.thumbnail) for book in bookmarks]

        return template.render(bookmarks=bookmarks)


    @app.route('/bookmarks')
    def bookmarks():
        session = get_session(get_db_path())
        bookmarks = session.query(Bookmark).order_by(Bookmark.score.desc()).all()

        bookmarks_list = []
        for bookmark in bookmarks:
            bookmark = {
                "url": bookmark.url,
                "thumb": "127.0.0.1:5000/" + bookmark.thumbnail,
                "score": bookmark.score,
            }

            bookmarks_list.append(bookmark)

        return jsonify(bookmarks_list)


    # sanity check route
    @app.route('/ping', methods=['GET'])
    def ping_pong():
        return jsonify('pong!')


    @app.route('/thumbnails/<filepath>', methods=['GET'])
    def thumbnails(filepath):  # pragma: no cover
        return send_from_directory(str(get_user_data_dir() / Path(".thumbnails")), filepath)

    @app.route('/redirect')
    def _redirect():
        session = get_session(get_db_path())
        url = request.args['url']

        bookmark = session.query(Bookmark).filter(Bookmark.url == url).one()

        if bookmark.score == None:
            bookmark.score = 0

        if bookmark.count == None:
            bookmark.count = 0

        bookmark.score += 1
        bookmark.count += 1

        session.commit()

        return redirect(url)

    app.run()


@click.command(name="gen_thumbnails")
def gen_thumbnails():
    session = get_session(get_db_path())
    user_data_dir = Path(get_user_data_dir())

    bookmarks = session.query(Bookmark).all()

    thumbnail_dir  = user_data_dir / Path(".thumbnails/")
    thumbnail_dir.mkdir(parents=True, exist_ok=True)

    thumbnails = thumbnail_dir.glob("*")

    driver = webdriver.Firefox()
    for bookmark in bookmarks:
        image_path = str(hash(bookmark.url)) + ".png"
        if image_path in thumbnails:
            print("skiped", bookmark.url)
            continue
        print("getting thumbnail for:", bookmark.url)
        try:
            driver.get(bookmark.url)
            thumbnail_path = image_path
            driver.save_screenshot(str(thumbnail_dir / thumbnail_path))
            bookmark.thumbnail = str(Path("thumbnails") / thumbnail_path)
        except Exception as e:
            print("Error: " + str(e))
            continue

    session.commit()

@click.command(name="rofi")
def rofi_launch():
    session = get_session(get_db_path())
    rm = RofiMarcador(session)
    rm.launch()


@click.group(invoke_without_command=True)
@click.version_option(version)
def main():
    db_path = get_db_path()
    if not db_path.is_file():
        print(db_path)
        db_path.parent.mkdir(exist_ok=True)
        db_path.touch()

    print("marcador.\nVersion:", version, "\nFor usage see marcador --help")

    return


main.add_command(print_bookmarks)
main.add_command(print_bookmark)
main.add_command(print_tags)
main.add_command(print_tag)

main.add_command(open_bookmark_cmd)
main.add_command(add)
main.add_command(delete)
main.add_command(get_url)
main.add_command(get_bookmark)
main.add_command(edit)
main.add_command(html)
main.add_command(gen_thumbnails)
main.add_command(rofi_launch)

if __name__ == "__main__":
    main()
