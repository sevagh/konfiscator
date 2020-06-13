2020 do-over - replace malloc with a basic fit-free list implementation in C.

Implementation goals:
1. Implement basic "first-fit free list" malloc from https://embeddedartistry.com/blog/2017/02/15/implementing-malloc-first-fit-free-list/ - use the kernel doubly linked list
2. Follow along the book [Modern C](https://modernc.gforge.inria.fr/) to write good quality C (or, at least, better than my usual)

### malloc api

```
void *malloc(size_t size);
void free(void *ptr);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void *reallocarray(void *ptr, size_t nmemb, size_t size);
```

Errors:

```
ERRORS
       calloc(), malloc(), realloc(), and reallocarray() can  fail  with  the
       following error:

       ENOMEM Out  of memory.  Possibly, the application hit the RLIMIT_AS or
              RLIMIT_DATA limit described in getrlimit(2).
```
