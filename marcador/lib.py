import os
import sqlite3
from subprocess import call
from pathlib import Path

from sqlalchemy import Column, ForeignKey, Integer, Float, String, func, create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship
from sqlalchemy.orm import sessionmaker
from sqlalchemy.orm.session import Session

from appdirs import user_data_dir


def get_user_data_dir():
    appauthor = "joajfreitas"
    appname = "marcador"

    return user_data_dir(appname, appauthor)


def get_db_path():
    print(get_user_data_dir())
    return Path(get_user_data_dir()) / "marcador.json"
