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
#include <string.h>

#include "trees.h"

tree *
init_tree(void) {
	tree *t=NULL;
	return t;
}

tree *
new_node(tree *left, tree *right, char *s) {
	tree *new = (tree *) malloc(sizeof(tree));
	new->left = NULL;
	new->right=NULL;
	new->string = s;

	return new;
}

void 
insert_tree(tree **t, char *s) {
	if (*t==NULL) {
		*t = new_node(NULL, NULL, s);
	}

	int cmp = strcmp((*t)->string, s);
	if (cmp < 0) {
		insert_tree(&((*t)->right), s);
	}
	else if (cmp > 0) {
		insert_tree(&((*t)->left), s);
	}
	return;
}

void 
print_tree(tree *t) {
	if (t == NULL) {
		return;
	}

	print_tree(t->left);
	printf("%s\n", t->string);
	print_tree(t->right);
}



