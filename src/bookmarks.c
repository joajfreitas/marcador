/*bookmarks - simple bookmark manager
 * Copyright © 2017 João Freitas
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */


#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "bookmarks.h"

void
print_bookmark(Bookmark *bk, FILE *stream) {
	fprintf(stream, "%d. %s ", bk->index, bk->url);
	if (bk->tags[0]) {
		fprintf(stream, "%s", bk->tags[0]);
	}
	for (int i=1; bk->tags[i]; i++)
		fprintf(stream, ",%s", bk->tags[i]);
	fputc('\n', stream);
}

void
write_bookmarks(Bookmarks *bookmarks, FILE *stream) {
	int index=0;
	Bookmark *aux;
	for (int i=0; i<bookmarks->size-1; i++) {
		aux = bookmarks->bookmarks[i];
		if (aux) {
			fprintf(stream, "%d. %s ", index, aux->url);
			if (aux->tags[0]) {
				fprintf(stream, "%s", aux->tags[0]);
			}
			for (int j=1; aux->tags[j]; j++)
				fprintf(stream, ",%s", aux->tags[j]);
			fputc('\n', stream);

			index++;
		}
	}
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
	Bookmark *bookmark = (Bookmark *) malloc(sizeof(Bookmark));

	bookmark->url = (char *) malloc(sizeof(char) * STRING_LEN);
	for (int i=0; i<20; i++) {
		bookmark->tags[i] = NULL;
	}

	bookmark->index = atoi(strtok_r(buffer, ".", &tok));
	strcpy(bookmark->url, strtok_r(NULL, " ", &tok));


	char *tag_tok = strtok_r(NULL, " ", &tok);
	if (!tag_tok) {
		return bookmark;
	}
	bookmark->tags[0] = (char *) malloc(sizeof(char) * STRING_LEN);
	strcpy(bookmark->tags[0], strtok_r(tag_tok, ",", &tok));

	for (int i=1; ; i++) {
		aux =  strtok_r(NULL, ",", &tok);
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
		if (bookmarks->bookmarks[i]) {
			free_bookmark(bookmarks->bookmarks[i]);
		}
	}

	free(bookmarks->bookmarks);
	free(bookmarks);
}
