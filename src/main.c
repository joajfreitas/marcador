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
	bool add, remove, search, tag_search, print, find, tags_list;
	char *query;
	char *url;
	char *tag_list[TAGS_MAX];
	char *path;
} arguments;
	
void 
usage(void)
{
	printf("usage: bk2 \n");
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
print(Bookmarks *bookmarks) {
	for (int i=0; i<bookmarks->occupied; i++) {
		print_bookmark(bookmarks->bookmarks[i], stdout);
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

unsigned long
djb2(unsigned char *str)
{
    unsigned long hash = 5381;
    int c;

    while (c = *str++)
        hash = ((hash << 5) + hash) + c; /* hash * 33 + c */

    return hash;
}

char *
get_tag(FILE *db) 
{
	char *buffer = (char *) malloc(sizeof(char) * 128);
	fgets(buffer, 128, db);	
	while(*buffer++) {
		if (*buffer == '-' && *(buffer-1) == ' ' && *(buffer+1) == ' ')
			return buffer+2;
	}
	return NULL;
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
		puts("Error: Invalid number of arguments");
		usage();
		exit(1);
	}

	while ((opt = getopt(argc, argv, "a:t:rhp:f:l")) != -1) {
		switch (opt) {
			case 'p':
				args.print = true;
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
			case 'h':
				usage();
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
		print(bookmarks);
	}
	else if (args.tags_list) {
		tag_list(bookmarks);
	}
	
	db = fopen(args.path, "w");
	write_bookmarks(bookmarks, db);
	fclose(db);

	free_bookmarks(bookmarks);

	return 0;
}


