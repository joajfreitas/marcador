from setuptools import setup, find_packages

setup(
    name='marcador',
    description='Simple rofi based bookmark manager',
    version='0.2',
    author="João Freitas",
    author_email="joaj.freitas@gmail.com",
    license="GPLv3",
    url='https://github.com/joajfreitas/marcador',
    download_url = 'https://github.com/joajfreitas/marcador/archive/v0.2.tar.gz',
    packages = find_packages(),
    entry_points={
            'console_scripts': [
                'marcador = marcador.__main__:main',
            ],
        },
    install_requires = [
        'clipboard',
        'python-rofi',
        'click',
        'jinja2',
        'beautifulsoup4',
        'request',
        'bottle'
    ]
)
