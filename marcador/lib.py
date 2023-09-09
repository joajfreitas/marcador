import os
import sqlite3
from subprocess import call
from pathlib import Path

from appdirs import user_data_dir


def get_user_data_dir():
    appauthor = "joajfreitas"
    appname = "marcador"

    return user_data_dir(appname, appauthor)


def get_db_path():
    return Path(get_user_data_dir()) / "marcador.json"
