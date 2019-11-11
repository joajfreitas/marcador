from setuptools import setup

setup(
    name='bookmarks',
    version='0.1',
    py_modules=['bookmarks'],
    install_requires=[
        'Click',
        'bottle',
        'jinja2',
        'requests',
        'beautifulsoup4'
    ],
    entry_points='''
        [console_scripts]
        bookmarks=bookmarks:main
    ''',
)
