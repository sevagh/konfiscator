#include "konfiscator.h"

// use the linux kernel doubly linked list
#include "list.h"

void *malloc(size_t size) {
  (void)size;
  return NULL;
}
