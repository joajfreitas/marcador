#include <stdlib.h>

#define STRING_LEN 128

typedef struct bookmark {
	unsigned int index;
	char *url;
	char *tags[20];
} bookmark;

void read_bookmark(char *buffer, char *tok, bookmark **bk);

void print_bookmark(bookmark *bk);
