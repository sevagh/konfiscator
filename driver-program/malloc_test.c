/* from https://raw.githubusercontent.com/embeddedartistry/embedded-resources/master/examples/c/malloc_test.c */

#include <stdlib.h>
#include <memory.h>
#include <stdio.h>

int main(void)
{
	int *ints = malloc(50*sizeof(int));

	for (int i = 0; i < 50; ++i) {
		ints[i] = i;
	}

	for (int i = 0; i < 50; ++i) {
		printf("index %d: value %d\n", i, ints[i]);
	}

	free(ints);
}
