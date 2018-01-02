#include <stddef.h>

void *konfiscator_malloc(size_t size);
void konfiscator_free(void *ptr);
void *konfiscator_calloc(size_t nmemb, size_t size);
void *konfiscator_realloc(void *ptr, size_t size);
extern void konfiscator_print();
