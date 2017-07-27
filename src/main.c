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

#define VERSION "0.2"

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

#define short(string) (string)[strlen((string))-1] == ',' ? 0 : (string)[strlen((string))-1]

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
	printf("-h\t\thelp\n");
	printf("-p [mode]\tprint bookmarks\n");
	printf("-t [tag]\tsearch bookmarks by tag\n");
	printf("-r\t\tremove bookmark\n");
	printf("-f [id]\t\tfind bookmark by ID\n");
	printf("-l\t\tlist tags\n");
	printf("-a [url] [tags]\tadd bookmark\n");
}

void
add(char **tags, Bookmarks *bookmarks) {
	uint64_t lines = 0;
	char *url = tags[0];
	char **t = tags+1;

	url[strlen(url) - 1	] = short(url);
	Bookmark *bookmark = (Bookmark *) malloc(sizeof(Bookmark));
	bookmark->url = url;
	if (bookmarks->bookmarks[0])
		bookmark->index = bookmarks->bookmarks[bookmarks->occupied-1]->index+1;
	else
		bookmark->index = 0;
	for (int i=0; t[i] != NULL; i++) {
		t[i][strlen(t[i])-1] = short(t[i]);
		bookmark->tags[i] = t[i];
	}
	insert_bookmark(bookmarks, bookmark);
}

void
find(Bookmarks *bookmarks, char *query, bool remove) {
	bool check_query = false;
	for (int i=0; i<strlen(query); i++)
		if (isdigit(query[i])) {
			check_query = true;
			break;
		}

	if (!check_query) {
		printf("Invalid query\n");
		exit(1);
	}

	unsigned int index = strtol(query, NULL, 10);
	if (index > bookmarks->occupied) {
		printf("Error: Bookmark doesn't exist in DB");
	}

	printf("%s\n", bookmarks->bookmarks[index]->url);
	if (remove) {
		free_bookmark(bookmarks->bookmarks[index]);
		bookmarks->bookmarks[index]=NULL;
		printf("Removed bookmark\n");
	}
}

void
tag_search(Bookmarks *bookmarks, char *query, bool remove) {
	Bookmark *aux;
	for (int i=0; i<bookmarks->occupied; i++) {
		aux = bookmarks->bookmarks[i];
		for (int j=0; aux->tags[j]; j++) {
			if (strstr(aux->tags[j], query)) {
				print_bookmark(aux, stdout);
			}
		}
	}
}

void
print(Bookmarks *bookmarks, char *query) {
	int option = atoi(query);

	switch (option) {
		case 0:
			for (int i=0; i<bookmarks->occupied; i++)
				print_bookmark(bookmarks->bookmarks[i], stdout);
			break;
		case 1:
			for (int i=0; i<bookmarks->occupied; i++)
				printf("%s\n", bookmarks->bookmarks[i]->url);
			break;
	}
}

char *
get_url(char *buffer) {
	while(*buffer++ != '.') ;
	buffer++;
	char *aux = buffer;
	while(*aux++ != 0)
		if (*aux == '-' && *(aux-1) == ' ' && *(aux+1) == ' ')
			*(aux-1) = 0;

	return buffer;
}

void tag_list(Bookmarks *bookmarks)
{
	tree *t = init_tree();
	for (int i=0; i<bookmarks->occupied; i++) {
		Bookmark *aux = bookmarks->bookmarks[i];
		for (int j=0; aux->tags[j]; j++) {
			insert_tree(&t, aux->tags[j]);
		}
	}

	print_tree(t);
}

void edit(Bookmarks *bookmarks, char *query) {
	unsigned int id = atoi(query);

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
	char *subopts, *value;
	int opt, index, lcount=0;
	char *next;
	char *tags[10] = {0};
	arguments args = {0};
	opterr = 0;
	if (argc < 2) {
		copyright();
		usage();
		exit(0);
	}

	while ((opt = getopt(argc, argv, "a:t:rhp:f:lve:")) != -1) {
		switch (opt) {
			case 'h':
				help();
				exit(0);
				break;
			case 'v':
				copyright();
				exit(0);
				break;
			case 'p':
				args.print = true;
				args.query = optarg;
				break;
			case 't':
				args.tag_search = true;
				args.query = optarg;
				break;
			case 'r':
				args.remove = true;
				break;
			case 'f':
				args.find = true;
				args.query = optarg;
				break;
			case 'l':
				args.tags_list = true;
				break;
			case 'e':
				args.edit = true;
				args.query = optarg;
				break;
			case 'a':
				args.add = 1;
				index = optind-1;
				while (index < argc-1){
					next = strdup(argv[index]); /* get login */
					index++;
					if (next[0] != '-'){         /* check if optarg is next switch */
						tags[lcount++] = next;
					}
					else break;
				}
				optind = index - 1;
				break;
			case '?':
				if (optopt == 't') {
					printf("Please provide an argument for tag search\n");
				}
				else {
					printf("Error\n");
				}
				exit(1);
				break;
			default:
				abort();
		}
	}

	args.path = argv[argc-1];

	FILE *db = efopen(args.path, "r");
	Bookmarks *bookmarks = read_bookmarks(db);
	fclose(db);

	if (args.add) {
		add(tags, bookmarks);
	}
	else if (args.tag_search) {
		tag_search(bookmarks, args.query, args.remove);
	}
	else if (args.find) {
		find(bookmarks, args.query, args.remove);
	}
	else if (args.print) {
		print(bookmarks, args.query);
	}
	else if (args.tags_list) {
		tag_list(bookmarks);
	}
	else if (args.edit) {
		edit(bookmarks, args.query);
	}

	db = fopen(args.path, "w");
	write_bookmarks(bookmarks, db);
	fclose(db);

	free_bookmarks(bookmarks);

	return 0;
}


