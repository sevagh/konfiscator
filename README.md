2020 do-over - replace malloc with a basic fit-free list implementation in C.

Not thread-safe.

Implementation goals:
1. Implement basic "first-fit free list" malloc from https://embeddedartistry.com/blog/2017/02/15/implementing-malloc-first-fit-free-list/ - use the kernel doubly linked list
2. Follow along the book [Modern C](https://modernc.gforge.inria.fr/) to write good quality C (or, at least, better than my usual)

Uses the LD_PRELOAD trick to test the custom malloc in the Docker container, with node_exporter stats for Prometheus.

### malloc api

Simple, only malloc and free.

```
void *malloc(size_t size);
void free(void *ptr);
```

Errors:

```
ERRORS
       calloc(), malloc(), realloc(), and reallocarray() can  fail  with  the
       following error:

       ENOMEM Out  of memory.  Possibly, the application hit the RLIMIT_AS or
              RLIMIT_DATA limit described in getrlimit(2).
```
