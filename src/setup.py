from setuptools import setup

setup(
    name='marcador',
    version='0.1',
    py_modules=['marcador','rofi_marcador'],
    install_requires=[
        'Click',
        'bottle',
        'jinja2',
        'requests',
        'beautifulsoup4'
    ],
    entry_points='''
        [console_scripts]
        marcador=marcador:main
    ''',
)
