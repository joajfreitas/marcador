#ifndef TREES_H
#define TREES_H

typedef struct tree {
	struct tree *left;
	struct tree *right;
	char *string;
} tree;

tree *init_tree();
void insert_tree(tree **, char *);
void print_tree(tree *);

#endif
