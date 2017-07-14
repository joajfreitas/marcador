#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "bookmarks.h"

void
read_bookmark(char *buffer, char *tok, bookmark **bk) {
	*bk = (bookmark *) malloc(sizeof(bookmark));

	(*bk)->index = atoi(strtok_r(buffer, ".", &tok));
	(*bk)->url= strtok_r(NULL, " - ", &tok);
	tok+=2;
	(*bk)->tags[0] = strtok_r(strtok_r(NULL, "", &tok), ",", &tok);

	for (int i=1; (*bk)->tags[i] = strtok_r(NULL, ",", &tok); i++);
}

void
print_bookmark(bookmark *bk) {
	printf("%d. %s - ", bk->index, bk->url);
	if (bk->tags[0]) {
		printf("%s", bk->tags[0]);
	}
	for (int i=1; bk->tags[i]; i++) printf(",%s", bk->tags[i]);
}






