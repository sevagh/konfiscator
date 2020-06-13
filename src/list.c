#include "list.h"
#include <stdlib.h>

typedef struct ll_head {
  struct ll_head *next;
  struct ll_head *prev;
} ll_t;

ll_t *ll_new(void) {
  ll_t *new = malloc(sizeof(ll_t));
  return new;
}

ll_t *ll_next(ll_t *l) { return l->next; }

ll_t *ll_prev(ll_t *l) { return l->prev; }

void ll_set_next(ll_t *l, ll_t *o) {
  l->next = o;
  o->prev = l;
}

void ll_set_prev(ll_t *l, ll_t *o) {
  l->prev = o;
  o->next = l;
}

// traverse and free all nodes
void ll_destroy(ll_t *l) {
  ll_t *curr = l;
  while (curr) {
    ll_t *next = ll_next(curr);
    free(curr);
    curr = next;
  }
}
