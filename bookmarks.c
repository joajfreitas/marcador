#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "bookmarks.h"

void
print_bookmark(Bookmark *bk, FILE *stream) {
	fprintf(stream, "%d. %s - ", bk->index, bk->url);
	if (bk->tags[0]) {
		fprintf(stream, "%s", bk->tags[0]);
	}
	for (int i=1; bk->tags[i]; i++) fprintf(stream, ",%s", bk->tags[i]);
	fputc('\n', stream);
}


Bookmarks *
read_bookmarks(FILE *db) {
	Bookmarks *bookmarks = (Bookmarks *) malloc(sizeof(Bookmarks));
	bookmarks->occupied = 0;
	bookmarks->size = 8;
	bookmarks->bookmarks = (Bookmark **) malloc(sizeof(Bookmark)*bookmarks->size);

	char buffer[STRING_LEN];
	char *tok = (char *) malloc(sizeof(char) * STRING_LEN);

	while(fgets(buffer, STRING_LEN, db)) {
		buffer[strlen(buffer)-1] = 0;
		insert_bookmark(bookmarks, read_bookmark(buffer, tok));
	}

	free(tok);

	return bookmarks;
}

Bookmark *
read_bookmark(char *buffer, char *tok) {
	char *aux;
	char *tmp;
	Bookmark *bookmark = (Bookmark *) malloc(sizeof(Bookmark));

	bookmark->url = (char *) malloc(sizeof(char) * STRING_LEN);
	for (int i=0; i<20; i++) {
		bookmark->tags[i] = NULL;
	}

	bookmark->index = atoi(strtok_r(buffer, ".", &tok));
	strcpy(bookmark->url, strtok_r(NULL, " - ", &tok));

	tmp = tok+2;
	
	bookmark->tags[0] = (char *) malloc(sizeof(char) * STRING_LEN);

	char *tag_tok = strtok_r(NULL, "", &tmp);
	if (!tag_tok) {
		return bookmark;
	}
	strcpy(bookmark->tags[0], strtok_r(tag_tok, ",", &tmp));

	for (int i=1; ; i++) {
		aux =  strtok_r(NULL, ",", &tmp);
		if (aux == NULL)
			break;

		bookmark->tags[i] = (char *) malloc(sizeof(char) * STRING_LEN);
		strcpy(bookmark->tags[i], aux);
	}
	return bookmark;
}

void 
insert_bookmark(Bookmarks *bookmarks, Bookmark *bookmark) {
	if (bookmarks->occupied == bookmarks->size) {
		bookmarks->size*=2;
		bookmarks->bookmarks = realloc(bookmarks->bookmarks, 
				sizeof(bookmarks)*bookmarks->size);
	}
	bookmarks->bookmarks[bookmarks->occupied++] = bookmark;
}

void free_bookmark(Bookmark *bk) {

	free(bk->url);
	for (int i=0; bk->tags[i]; i++) {
		free(bk->tags[i]);
	}

	free(bk);
}

void 
free_bookmarks(Bookmarks *bookmarks) {
	for (int i=0; i<bookmarks->occupied; i++) {
		free_bookmark(bookmarks->bookmarks[i]);
	}

	free(bookmarks->bookmarks);
	free(bookmarks);
}





