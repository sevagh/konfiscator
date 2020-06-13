#ifndef LINKED_LIST_H
#define LINKED_LIST_H

/*
 * based on the following definition:
 *
	typedef struct ll_head {
	     struct ll_head *next;
	     struct ll_head *prev;
	} ll_t;
 */

typedef struct ll_head ll_t;
ll_t* ll_new(void);
ll_t* ll_next(ll_t*);
ll_t* ll_prev(ll_t*);
void ll_set_next(ll_t*, ll_t*);
void ll_set_prev(ll_t*, ll_t*);
void ll_destroy(ll_t*);

#endif /* LINKED_LIST_H */
