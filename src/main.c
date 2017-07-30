/* bookmarks - simple bookmark manager
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

#define VERSION "0.3"

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <ctype.h>

#include "util.h"
#include "bookmarks.h"
#include "trees.h"

#define TAGS_MAX 32

char *helps[] = {"\tprint [modifier] file\n\t\tPrint bookmarks",	
				"\tadd url [tag0, tag1, ...] file\n\t\tAdd a new bookmark",		
				"\trm id file\n\t\tRemove a bookmark by its id",
				"\tfind id file\n\t\tGet the url of a bookmark by its id",
				"\tedit id file\n\t\tEdit a bookmark with $EDITOR",
				"\ttag-search tag book\n\t\tFind all bookmarks with a tag",
				"\ttag-list file\n\t\tList all tags in a file",
};

void print_help(unsigned int index, FILE *output) {
	fprintf(output, "%s\n", helps[index]);
}
	
typedef struct arguments {
	bool add, remove, search, tag_search, print, find, tags_list, edit;
	char *query;
	char *url;
	char *tag_list[TAGS_MAX];
	char *path;
} arguments;

void
usage(void) {
	printf("\n\033[1mbookmarks\033[0m: simple linux bookmark manager\n$ bookmarks -h for help\n");
}

void
copyright(void) {
	printf("bookmarks %s - (C) 2017 João Freitas\n", VERSION);
	printf("Released under the GNU GPL\n");
}


void
help(void) {
	printf("\033[1mbookmarks\033[0m: simple linux bookmark manager\n");
	putchar('\n');
	printf("Usage:\n");
	for (int i=0; i<7; i++) {
		print_help(i, stdout);
	}
}

/* bookmarks add url [tag0, tag1, ...] file
 *
 * register a url and a TODO: random number of tags
 * TODO: check URL
 */

void
add(Bookmarks *bookmarks, char **argv, int argc) {
	if (argc < 4) {
		fprintf(stderr, "Error: Invalid number of arguments for command add!\n");
		fprintf(stderr, "Usage: $ bookmarks add url [tag0, tag1, ...] file\n");
		exit(1);
	}
	Bookmark *bookmark = (Bookmark *) malloc(sizeof(Bookmark));
	bookmark->url = (char *) malloc(sizeof(char) * STRING_LEN);
	strcpy(bookmark->url, argv[2]);

	if (bookmarks->bookmarks[0])
		bookmark->index = bookmarks->bookmarks[bookmarks->occupied-1]->index+1;
	else
		bookmark->index = 0;
	for (int i=0; i < argc-4; i++) {
		bookmark->tags[i] = (char *) malloc(sizeof(char) * STRING_LEN);
		strcpy(bookmark->tags[i], argv[i+3]);
	}

	insert_bookmark(bookmarks, bookmark);
}

/* bookmarks rm id file
 * Remove bookmarks by id
 */
void rm(Bookmarks *bookmarks, char **argv, int argc) {
	if (argc != 4) {
		fprintf(stderr, "Error: Invalid number of arguments for command rm!\n");
		fprintf(stderr, "Usage: $ bookmarks rm id file\n");
		exit(1);
	}

	unsigned int index = atoi(argv[2]);
	if (index >= bookmarks->occupied) {
		fprintf(stderr, "Error: Bookmark %d not in range!\n", index);
		exit(1);
	}
	free_bookmark(bookmarks->bookmarks[index]);
	bookmarks->bookmarks[index] = NULL;
}

/* bookmarks find id file
 * find bookmark url by id
 */
void
find(Bookmarks *bookmarks, char **argv, int argc) {
	if (argc != 4) {
		fprintf(stderr, "Error: Invalid number of arguments for command find\n");
		fprintf(stderr, "Usage: $ bookmarks find id file\n");
		exit(1);
	}

	unsigned index = atoi(argv[2]);
	if (index >= bookmarks->occupied) {
		fprintf(stderr, "Error: Bookmark %d out of range!\n", index);
		exit(1);
	}

	printf("%s\n", bookmarks->bookmarks[index]->url);
}

/* bookmarks tag-search tag book
 * Find urls in a tag
 */
void
tag_search(Bookmarks *bookmarks, char **argv, int argc) {
	if (argc != 4) {
		fprintf(stderr, "Error: Invalid number of arguments for command tag-search\n");
		fprintf(stderr, "Usage: $ bookmarks tag-search tag book\n");
		exit(1);
	}

	Bookmark *aux;
	for (int i=0; i<bookmarks->occupied; i++) {
		aux = bookmarks->bookmarks[i];
		for (int j=0; aux->tags[j]; j++) {
			if (strstr(aux->tags[j], argv[2])) {
				print_bookmark(aux, stdout);
			}
		}
	}
}

/* bookmarks print [modifier] file
 * modifiers: 0 - pretty print
 *			  1 - only urls
 */
void
print(Bookmarks *bookmarks, char **argv, int argc) {
	if (argc > 4) {
		fprintf(stderr, "Error: Invalid number of arguments for command print!\n");
		fprintf(stderr, "Usage: $ bookmarks print [modifier] file\n");
		exit(1);
	}
	
	unsigned int option;
		
	if (argc == 3) {
		option = 0;
	}
	else if (argc == 4) {
		option = atoi(argv[2]);
	}


	switch (option) {
		case 0:
			for (int i=0; i<bookmarks->occupied; i++)
				print_bookmark(bookmarks->bookmarks[i], stdout);
			break;
		case 1:
			for (int i=0; i<bookmarks->occupied; i++)
				printf("%s\n", bookmarks->bookmarks[i]->url);
			break;
		default:
			fprintf(stderr, "Error: Unrecognized option for command print!\n");
			exit(1);
	}
}


/* bookmarks tag-list file
 * List all tags in a file
 */
void 
list_tags(Bookmarks *bookmarks, char **argv, int argc) {
	if (argc != 3) {
		fprintf(stderr, "Error: Invalid number of arguments!\n");
		fprintf(stderr, "Usage: bookmarks tag-list file\n");
		exit(1);
	}

	tree *t = init_tree();
	for (int i=0; i<bookmarks->occupied; i++) {
		Bookmark *aux = bookmarks->bookmarks[i];
		for (int j=0; aux->tags[j]; j++) {
			insert_tree(&t, aux->tags[j]);
		}
	}

	print_tree(t);
}

/* bookmarks edit id file
 * Edit a bookmark
 */
void 
edit(Bookmarks *bookmarks, char **argv, int argc) {
	if (argc != 4) {
		fprintf(stderr, "Error: Invalid number of arguments for command edit!\n");
		fprintf(stderr, "Usage: bookmarks edit id file\n");
		exit(1);
	}

	unsigned int id = atoi(argv[2]);

	FILE *tmp = efopen("/tmp/bookmarks.tmp", "w");
	print_bookmark(bookmarks->bookmarks[id], tmp);
	fclose(tmp);

	system("$TERM -e $EDITOR /tmp/bookmarks.tmp");

	tmp = efopen("/tmp/bookmarks.tmp", "r");
	char *buffer = malloc(sizeof(char)*256);
	char *tok = malloc(sizeof(char)*256);
	fgets(buffer, 256, tmp);
	buffer[strlen(buffer)-1] = '\0';
	Bookmark *bookmark = read_bookmark(buffer, tok);
	
	free(bookmarks->bookmarks[id]);
	bookmarks->bookmarks[id] = bookmark;

	free(tok);
	free(buffer);
}

int
main (int argc, char **argv)
{
	char *commands[] = {"print", "add", "rm", "find", "edit", "tag-search", "list-tags"};

	void (*functions[7]) (Bookmarks *bookmarks, char **argv, int argc) = 
	{print, add, rm, find, edit, tag_search, list_tags};	
	
	if (!strcmp(argv[1], "-h")) {
		help();
		exit(0);
	}
	else if (!strcmp(argv[1], "-v")) {
		copyright();
		exit(0);
	}

	if (argc < 3) {
		printf("Error: Invalid number of arguments\n");
		copyright();
		usage();
		exit(1);
	}

	FILE *db = efopen(argv[argc-1], "r");
	Bookmarks *bookmarks = read_bookmarks(db);
	fclose(db);

	bool found=false;
	for (int i=0; i<7; i++){
		if (!strcmp(commands[i], argv[1])) {
			functions[i](bookmarks, argv, argc);
			found=true;
		}
	}

	if (!found) {
		printf("Error: Command not found: %s!\n", argv[1]);
		exit(1);
	}


	db = fopen(argv[argc-1], "w");
	write_bookmarks(bookmarks, db);
	fclose(db);

	free_bookmarks(bookmarks);

	return 0;
}


