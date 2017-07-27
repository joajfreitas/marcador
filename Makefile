VERSION=0.2
PREFIX=/usr/local

#CFLAGS=-O3 -Wall -Wextra -pedantic
#CFLAGS=-Wall -Wextra -pedantic
#CFLAGS=-g -pg -Wall -Wextra -pedantic
CFLAGS=-g
#CFLAGS=-g -Wall
CPPFLAGS=-MP -MMD
LDFLAGS=
CC=gcc
SRC=$(wildcard src/*.c)
EXEC=bookmarks

all:
	make $(EXEC)
	rm -rf *.d

$(EXEC): $(SRC:%.c=%.o)
	# This will implicity make all the .c files with *FLAGS and with
	# dependencies generated automatically by CPPFLAGS, included below (.d files)
	$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)

-include $(SRC:%.c=%.d)

dist: clean
	@echo creating dist tarball
	@mkdir -p bookmarks-$(VERSION)
	@cp -R LICENSE Makefile src/*.c src/*.h README.md bookmarks-${VERSION}
	@tar -cf bookmarks-${VERSION}.tar bookmarks-${VERSION}
	@gzip bookmarks-${VERSION}.tar
	@rm -rf bookmarks-${VERSION}

install: all
	@echo installing executable file to $(DESTDIR)$(PREFIX)/bin
	@mkdir -p $(DESTDIR)$(PREFIX)/bin
	@cp -f bookmarks $(DESTDIR)$(PREFIX)/bin
	@cp -f rofi-bookmarks $(DESTDIR)$(PREFIX)/bin
	@chmod 755 $(DESTDIR)${PREFIX}/bin/bookmarks $(DESTDIR)${PREFIX}/bin/rofi-bookmarks
	$(MAKE) clean

clean:
	rm -rf **/*.o **/*.d *.out* $(EXEC) .dummy doc tags

# This rebuilds everything if the Makefile was modified
# http://stackoverflow.com/questions/3871444/making-all-rules-depend-on-the-makefile-itself/3892826#3892826
-include .dummy
.dummy: Makefile
	touch $@
	$(MAKE) -s clean
