#include <error.h>
#include <stdio.h>
#include <stdlib.h>

FILE *
efopen(const char *path, const char *mode) {
	FILE *fp = fopen(path, mode);
	if (fp == NULL) {
		printf("Can't open %s\n", path);
		exit(EXIT_FAILURE);
	}
	return fp;
}
