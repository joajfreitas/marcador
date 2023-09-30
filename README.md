# Marcador

Minimal bookmark manager

## Install

	$ pip install marcador

## Usage

CLI interface
	

    Usage: marcador [OPTIONS] COMMAND [ARGS]...

    Options:
      --version
      --help     Show this message and exit.

    Commands:
      add
      bookmarks
      delete
      rofi
      server

### Rofi interface

	$ marcador rofi

### Configuration

Rofi server supports reading configuration options from a config file. The file
should be located according to the [appdirs](https://pypi.org/project/appdirs/)
specification. In particular, for linux follow the
[XDG](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
spec.

Available config options with default values:

``` toml
hostname="127.0.0.1"
port=6003
root="marcador"
```

## Thanks
This project is heavily inspired by [buku](https://github.com/jarun/Buku)

## Screenshots
![bookmarks screenshot](https://raw.githubusercontent.com/joajfreitas/marcador/master/showcase_pretty.png)
