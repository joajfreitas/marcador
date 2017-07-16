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
