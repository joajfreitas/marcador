from setuptools import setup, find_packages

# read the contents of your README file
from os import path

from marcador import version

this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(
    name='marcador',
    description='Simple rofi based bookmark manager',
    long_description=long_description,
    long_description_content_type='text/markdown',
    version=version,
    author="João Freitas",
    author_email="joaj.freitas@gmail.com",
    license="GPLv3",
    url='https://github.com/joajfreitas/marcador',
    #download_url = 'https://github.com/joajfreitas/marcador/archive/v0.2.tar.gz',
    packages = find_packages(),
    entry_points={'console_scripts': ["marcador = marcador.__main__:main",],},
    install_requires = [
        'clipboard',
        'python-rofi',
        'click',
        'jinja2',
        'beautifulsoup4',
        'requests',
        'bottle',
        'selenium',
        'sqlalchemy',
    ]
)
