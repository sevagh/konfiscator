#include "./malloc.h"
#include <errno.h>
#include <stdio.h>
#include <mcheck.h>
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

    mtrace();
    void *x = malloc(desired_size);
    free(x);
    muntrace();

    return 0;
}
