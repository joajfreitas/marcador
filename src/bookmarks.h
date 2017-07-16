#ifndef BOOKMARKS_H
#define BOOKMARKS_H

#include <stdlib.h>

#define STRING_LEN 256

typedef struct Bookmark {
	unsigned int index;
	char *url;
	char *tags[20];
} Bookmark;

typedef struct Bookmarks {
	Bookmark **bookmarks;
	unsigned int occupied;
	unsigned int size;
} Bookmarks;

void print_bookmark(Bookmark *bk, FILE *stream);
void write_bookmarks(Bookmarks *bookmarks, FILE *stream);
Bookmarks *read_bookmarks(FILE *db);
Bookmark *read_bookmark(char *buffer, char *tok);
void insert_bookmark(Bookmarks *bookmarks, Bookmark *bookmark);
void free_bookmark(Bookmark *bk);
void free_bookmarks(Bookmarks *bookmarks);

#endif
