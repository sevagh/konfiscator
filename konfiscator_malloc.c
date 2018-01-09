#include "./konfiscator_malloc.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
    if (argc < 2) {
        fprintf(stderr, "Usage: malloc <size in bytes>\n");
        exit(1);
    }

    long int desired_size = strtol(argv[1], NULL, 10);
    if ((errno == EINVAL) || (errno == ERANGE))  {
        fprintf(stderr, "%s couldn't be converted to an int\n", argv[1]);
        exit(1);
    }

    printf("Using desired size: %lu\n", desired_size);

    char *x = konfiscator_malloc(desired_size);
    if (x == NULL) {
	    fprintf(stderr, "konfiscator_malloc error\n");
    }
    for (int i = 0; i < desired_size; ++i)
	    x[i] = 'a';
    konfiscator_free(x);

    return 0;
}
