#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>

#include "util.h"
#include "bookmarks.h"

#define TAGS_MAX 32

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
add(char *url, char *path, char **tags) {
	FILE *db = efopen(path, "r");
	uint64_t lines = 0;
	char ch = 0;

	while (!feof(db)) {
		ch = fgetc(db);
		if (ch == '\n') {
			lines++;
		}
	}

	if (url[strlen(url)-1] == ',') url[strlen(url)-1] = 0;

	fclose(db);
	db = efopen(path, "a");
	fprintf(db, "%lu. %s - ", lines, url);
	for (int i=0; tags[i] != NULL; i++) {
		fprintf(db, "%s", tags[i]);
	}
	fprintf(db, "\n");
}

void 
search(char *query, char *path , bool remove) {
	FILE *db = efopen(path ,"r");
	char buffer[128];
	while(fgets(buffer, 128, db)) {
		if (strstr(buffer, query)) {
			printf("%s", buffer);
		}
	}
}

void 
tag_search(char *query, char *path, bool remove) {
	FILE *db = efopen(path ,"r");
	char buffer[128];
	char *tags;

	while (fgets(buffer, 128, db)) {
		for (int i=0; i<strlen(buffer); i++) {
			if (buffer[i] == '-') {
				tags = buffer+i;
			}
		}
		if (strstr(tags, query)) {
			printf("%s", buffer);
		}
	}
}

void 
print(char *path) {
	FILE *db = efopen(path, "r");
	char buffer[128];
	while (fgets(buffer, 128, db)) {
		printf("%s", buffer);
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
void 
find(char *query, char *path) {
	FILE *db = efopen(path, "r");
	char buffer[128];
	
	int index = atoi(query);
	while(fgets(buffer, 128, db)) {
		if (index == atoi(buffer)) {
			printf("%s\n", get_url(buffer));
			break;
		}
	}
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

char *get_tag(FILE *db) 
{
	char *buffer = (char *) malloc(sizeof(char) * 128);
	fgets(buffer, 128, db);	
	while(*buffer++) {
		if (*buffer == '-' && *(buffer-1) == ' ' && *(buffer+1) == ' ')
			return buffer+2;
	}
	return NULL;
}

void tag_list(char *path) 
{
	char **tags = (char **) calloc(128, sizeof(char *));
	for (int i = 0; i < 128; ++i) {
		tags[i] = calloc(128, sizeof(char));
	}
	
	FILE *db = efopen(path, "r");
	char *tag;
	char *tmp;
	char *aux = malloc(sizeof(char) * 128);
	while (tag = get_tag(db)) {
		printf("this tag\n");
		tmp = tag;
		int i=0;
		for (i=0; *tag++; i++) {
			if (*tag == ',') {
				strcpy(aux, tmp);
				aux[i+1] = 0;
				i=-2;
				tmp = tag+1;
				strcpy(tags[djb2(aux) % 128], aux); //TODO: No colisions
			}
		}
		tmp[i-1] = 0;
		strcpy(tags[djb2(tmp) % 128], tmp);
	}
	for (int i=0; i<128; i++) {
		if (tags[i][0] != 0) {
			printf("%s\n", tags[i]);
		}
	}
}

int
main (int argc, char **argv)
{
	char *subopts, *value;
	int opt, index, lcount;
	char *next;
	char *tags[10] = {0};
	arguments args = {0};
	opterr = 0;
	if (argc < 2) {
		puts("Error: Invalid number of arguments");
		usage();
		exit(1);
	}

	while ((opt = getopt(argc, argv, "a:t:rs:hp:f:l")) != -1) {
		switch (opt) {
			case 'p':
				args.print = true;
				break;
			case 's':
				args.search = true;
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
	/*
	if (args.add) {
		for (int i=0; i<TAGS_MAX; i++) {
			if (tags[i] == NULL) {
				break;
			}
		}
		add(tags[0], args.path, tags+1);
	}
	else if (args.search) {
		search(args.query, args.path, args.remove);
	}
	else if (args.tag_search) {
		tag_search(args.query, args.path, args.remove);
	}
	else if (args.print) {
		print(args.path);
	}
	else if (args.find) {
		find(args.query, args.path);
	}
	else if (args.tags_list) {
		tag_list(args.path);
	}
	*/
	FILE *db = efopen(args.path, "r");
	int bk_size = 10;
	int bk_occupied = 0;
	bookmark **bookmarks = (bookmark **) malloc(sizeof(bookmark*) * bk_size);

	char buffer[STRING_LEN];
	char *tok = (char *) malloc(sizeof(char) * STRING_LEN);

	while(fgets(buffer, STRING_LEN, db)) {
		if (bk_size == bk_occupied+1) {
			printf("realloc");
			bk_size *= 2;
			bookmarks = (bookmark **) realloc(bookmarks, bk_size*sizeof(bookmark*));
		}
		read_bookmark(buffer, tok, &bookmarks[bk_occupied]);
		for (int j=0; j<bk_occupied; j++) {
			print_bookmark(bookmarks[j]);
			putchar('\n');
		}
		bk_occupied++;
	}

	for (int j=0; j<bk_occupied; j++) {
		printf("%d\n", j);
		printf("%s\n", bookmarks[j]->url);
		printf("%p\n", bookmarks[j]);
	}
	printf("size: %u\n", sizeof(bookmark));
	return 0;
}
