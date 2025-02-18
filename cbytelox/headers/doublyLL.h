/*
* Author: deepencoding
* Purpose: Quick little doubly LL implementation
*/

#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
	int val;
	struct Node* left;
	struct Node* right;
} Node;

Node* createNode(int data) {
	Node* head = (Node*) malloc(sizeof(Node));
	head->val = data;
	head->left = NULL;
	head->right = NULL;
	return head;
}

void PushF(Node** root, int data) {
	if (!(*root)) puts("Invalid root pointer.\n");
	Node* cur = *root;
	Node* nh = createNode(data);
	nh->right = cur;
	cur->left = nh;
	*root = nh;
}

void PushB(Node** root, int data) {
	if (!(*root)) puts("Invalid root pointer.\n");
	Node* cur = *root;
	while (cur && cur->right) cur = cur->right;
	Node* nn = createNode(data);
	nn->left = cur;
	cur->right = nn;
}

int PopF(Node** root) {
	if (!(*root)) puts("Invalid root pointer.\n");
	Node* cur = *root;
	*root = cur->right;
	int ret = cur->val;
	cur->right->left = NULL;
	free(cur);
	return ret;
}


int PopB(Node** root) {
	if (!(*root)) puts("Invalid root pointer.\n");
	Node* cur = *root;
	while (cur && cur->right) cur = cur->right;
	cur->left->right = NULL;
	int ret = cur->val;
	free(cur);
	return ret;
}

void PrintF(Node** root) {
	if (!(*root)) printf("Invalid root pointer.");
	Node* cur = *root;
	printf("Forward List: ");
	while (cur && cur->right) {
		printf("%d ", cur->val);
		cur = cur->right;
	}
	printf("%d\n", cur->val);
}
