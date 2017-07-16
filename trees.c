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



